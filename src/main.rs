use core::str;
use gethostname::gethostname;
use std::fs;
use std::process::exit;
use users::get_current_username;
use std::env;

const SYSTEM_BOARD: &str = "/sys/devices/virtual/dmi/id/board_name";
const KERNEL: &str = "/proc/version";

fn get_info_from_file(filename: &str) -> String {
    let contents = fs::read_to_string(filename);

    match contents {
        Ok(data) => data.trim().to_string(),
        Err(e) => {
            println!("Error reading file: {}", e);
            exit(1)
        }
    }
}

fn get_title() -> String {
    let user = match get_current_username() {
        Some(uname) => uname,
        None => {
            println!("Unable to get username");
            exit(1)
        }
    };

    let hostname = gethostname();

    let title = user.to_string_lossy() + "@" + hostname.to_string_lossy();

    title.to_string()
}

fn get_env(env: &str) -> String {
    let term = env;

    match env::var(term) {
        Ok(term) => term,
        Err(_) => {
            println!("Unable to read $TERM");
            exit(1)
        }
    }
}

fn main() {
    let os = os_info::get();
    let kernel_info = get_info_from_file(KERNEL);
    let title = get_title();
    let system_info = get_info_from_file(SYSTEM_BOARD);
    let terminal = get_env("TERM_PROGRAM");
    let wm = get_env("XDG_CURRENT_DESKTOP");

    println!("{}", { title });
    println!("{}", "-----------");
    println!("{}       {}", "OS:", os.os_type());
    println!("{}   {}", "Kernel:", { kernel_info });
    println!("{}     {}", "Arch:", os.architecture().unwrap_or("Unknown"));
    println!("{}   {}", "System:", { system_info });
    println!("{} {}", "Terminal:", { terminal });
    println!("{}       {}", "WM:", { wm });
}
