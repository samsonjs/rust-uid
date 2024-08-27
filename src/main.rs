use sysinfo::{System};

fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();
    if let Ok(current_pid) = sysinfo::get_current_pid() {
        println!("Current pid: {}", current_pid);
        if let Some(uid) = sys.process(current_pid)
                              .and_then(|process| process.user_id()) {
            println!("User id: {:?}", uid);
            let uid_u32 = get_uid_as_u32(uid);
            println!("User id u32: {}", uid_u32);
            let port = 43737 + 300;
            let max_port = 65535;
            let max_uid: u32 = max_port - port as u32;
            println!("Max user id for port {}: {}", port, max_uid);
            let wrapped_uid: u16 = (uid_u32 % max_uid) as u16;
            println!("User id wrapped: {}", wrapped_uid);
            let user_port = port + wrapped_uid;
            println!("User port: {}", user_port);
        } else {
            println!("Error: Could not get user id from current pid {}", current_pid);
        }
    } else {
        println!("Error: Could not get current pid from sysinfo");
    }
}

#[cfg(unix)]
fn get_uid_as_u32(uid: &sysinfo::Uid) -> u32 {
    *uid.clone()
}

#[cfg(windows)]
fn get_uid_as_u32(uid: &sysinfo::windows::sid::Sid) -> u32 {
    // On Windows, extract the RID from the SID
    uid.to_string()
        .rsplit('-')
        .next()
        .and_then(|rid| rid.parse::<u32>().ok())
        .unwrap_or(0)
}
