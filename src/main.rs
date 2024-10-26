use std::process::Command;
use std::str;
use std::fs;

fn invokecommand(command: &str, params: &str) -> String {
    let output = {
        Command::new(command)
            .args(params.split_whitespace())
            .output()
            .expect("Failed to retrieve info")
    };

    let command = match str::from_utf8(&output.stdout) {
        Ok(v) => v.trim(),
        Err(e) => panic!("Invalid UTF-8 Sequence: {}", e)
    };

    command.to_string()
}

fn get_kernel_version() -> String {
   
    let kernel_version = invokecommand("uname", "-r");

    kernel_version.to_string()
}

fn get_title() -> String {

    let user = invokecommand("id", "-un");
    let hostname = invokecommand("hostname", "");

    let title = user.to_owned() + "@" + &hostname;

    title.to_string()

}

fn get_system_board() -> String {
    let contents = fs::read_to_string("/sys/devices/virtual/dmi/id/board_name")
        .expect("Unable to read file");
    
    contents
}

fn main() {
    let os = os_info::get();
    let returned_kernel = get_kernel_version();
    let returned_title = get_title();
    let returned_system = get_system_board();

    println!("{}",{returned_title});
    println!("{}","-----------");
    println!("{}       {}", "OS:", os.os_type());
    println!("{}     {}", "Arch:", os.architecture().unwrap_or("Unknown"));
    println!("{}   {}", "Kernel:", {returned_kernel});
    println!("{}   {}", "System:", {returned_system});
}
