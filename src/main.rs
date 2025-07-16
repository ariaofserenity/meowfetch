use colored::*;
use core::str;
use gethostname::gethostname;
use std::env;
use std::fs;
use std::process::exit;
use unicode_width::UnicodeWidthStr;
use users::get_current_username;

const SYSTEM_BOARD: &str = "/sys/devices/virtual/dmi/id/board_name";
const KERNEL: &str = "/proc/sys/kernel/osrelease";

pub struct CustomColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl CustomColor {
    pub fn apply_to<'a>(&self, text: &'a str) -> ColoredString {
        text.truecolor(self.r, self.g, self.b)
    }
}

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
    match env::var(env) {
        Ok(val) => val,
        Err(_) => "Unknown".to_string(),
    }
}

fn main() {
    let c_pink = CustomColor {
        r: 255,
        g: 111,
        b: 145,
    };
    let c_blue = CustomColor {
        r: 27,
        g: 140,
        b: 157,
    };
    let os = os_info::get();
    let os_type_str = os.os_type().to_string();
    let kernel_info = get_info_from_file(KERNEL);
    let title = get_title();
    let system_info = get_info_from_file(SYSTEM_BOARD);
    let terminal = get_env("TERM_PROGRAM");
    let wm = get_env("XDG_CURRENT_DESKTOP");
    let arch = os.architecture().unwrap_or("Unknown").to_string();
    let sep = "-----------";

    const ASCII_ART: &str = r#"
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣤⣀⣀⠀⠀⣀⡠⠴⠒⠚⠉⠉⠓⠒⠦⣄⣶⠒⣷⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠘⡷⢬⣉⠉⠁⠀⠀⠀⠀⠀⠀⠀⠀⠠⡌⠻⣧⢻⣧⣤⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣖⠗⡋⢹⠀⠀⢰⡄⠀⠀⢸⣷⡀⠀⣠⠽⣆⢼⣇⢻⣸⡄⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⢀⡜⣡⣶⢋⡏⠙⢢⣏⣇⠀⠀⠈⣇⡵⡏⠀⠀⢹⡏⢾⣿⠃⢿⡆⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⣾⢿⢻⣏⣿⡇⡄⣾⠀⠹⡄⠄⠀⡇⠀⠹⣤⠈⠹⣿⣾⢸⠀⢘⣷⣄⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⢠⣴⣯⣿⣽⣿⣷⢸⡗⠦⣄⡹⣼⣄⣿⣴⠛⠹⡄⡇⣿⣿⠾⠚⢹⢿⢽⣽⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⣸⣿⣞⣾⣿⢿⣯⢻⢻⡴⠞⠁⠈⠻⣿⣌⡉⠓⣿⣰⡿⠀⠀⠀⠸⡜⡾⣿⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⢀⡴⣡⠊⢸⣹⠁⠈⠙⣾⡄⠁⠀⢰⠛⠉⠉⠉⢳⣀⣿⣿⠃⠀⠀⣀⣀⣧⣿⡞⣷⡀⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⣠⠋⡴⠁⠀⠸⢿⣤⣤⣤⣹⣿⣷⣶⣾⣷⣶⣶⣺⣋⣽⣿⣷⠶⠟⠛⠋⢧⠀⠀⠸⡜⣷⠀⠀⠀⠀⠀⠀⠀⠀
⠀⠀⢀⡜⠁⡰⠁⠀⠀⢠⡿⠀⠀⠀⠉⠉⠉⠙⢻⡟⣹⣿⠃⣿⠋⠁⠀⠀⠀⠀⠀⠸⡄⠀⠀⢣⠹⣧⠀⠀⠀⠀⠀⠀⠀
⠀⢠⠏⡀⢠⠇⠀⠀⢠⡿⠁⠀⠀⠀⠀⣤⣶⡴⠚⢻⠡⣸⠀⢹⣆⠀⠀⠀⠀⠀⠀⠀⡇⠀⠀⠸⡄⢻⣇⠀⠀⠀⠀⠀⠀
⢀⡏⣼⠁⢸⠀⠀⠀⣾⠃⠀⠀⠀⠀⠀⢻⣿⣧⣀⣬⠋⠁⠀⣠⣿⣶⣆⠀⠀⠀⠀⠀⡇⠀⠀⠀⡇⠈⣿⡀⠀⠀⠀⠀⠀
⣸⣸⣿⠀⡇⠀⢰⣸⡟⠀⠀⠀⣀⣠⠴⠚⣟⣻⣧⣯⣗⣤⣾⣿⣿⡿⠋⠀⠀⠀⠀⣸⣤⠀⠀⠀⡇⡆⢻⠃⠀⠀⠀⠀⠀
⣿⡿⢸⡀⣇⠀⣸⣿⡁⠀⣾⣻⡁⣀⣤⣶⠟⠋⠉⠛⢿⣋⣻⡏⠉⠀⠀⠀⠀⠀⢰⣿⡇⠀⠀⠀⣷⡇⣸⡄⠀⠀⠀⠀⠀
⠿⠇⠀⢧⢸⠀⣿⡿⠇⠀⠈⠛⠛⠋⠉⠀⠀⠀⠀⠀⡟⠀⣿⠇⠀⠀⠀⠀⠀⢠⣿⣿⡇⠀⠀⣰⡿⣧⣿⠃⠀⠀⠀⠀⠀
⠀⠀⠀⠀⢿⣄⣹⡇⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢰⡇⠀⣿⠀⠀⠀⠀⠀⠀⣸⡿⢸⠁⢠⣾⠋⢰⣿⡏⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠉⠛⠛⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣼⣶⣶⡿⠀⠀⠀⠀⠀⠀⠉⠁⢸⣶⡟⠁⠀⠾⠟⠀⠀⠀⠀⠀⠀⠀
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠉⠉⠀⠀⠀⠀⠀⠀⠀⠀⠀⠈⠉⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
"#;

    let ascii_lines: Vec<&str> = ASCII_ART.trim_matches('\n').lines().collect();

    let info_lines = vec![
        format!("{}", c_pink.apply_to(&title)),
        format!("{}", c_pink.apply_to(&sep)),
        format!(
            "{}       {}",
            c_pink.apply_to("OS:"),
            c_pink.apply_to(&os_type_str)
        ),
        format!(
            "{}   {}",
            c_pink.apply_to("Kernel:"),
            c_pink.apply_to(&kernel_info)
        ),
        format!(
            "{}     {}",
            c_pink.apply_to("Arch:"),
            c_pink.apply_to(&arch)
        ),
        format!(
            "{}   {}",
            c_pink.apply_to("System:"),
            c_pink.apply_to(&system_info)
        ),
        format!(
            "{} {}",
            c_pink.apply_to("Terminal:"),
            c_pink.apply_to(&terminal)
        ),
        format!("{}       {}", c_pink.apply_to("WM:"), c_pink.apply_to(&wm)),
    ];

    let art_width = ascii_lines
        .iter()
        .map(|line| UnicodeWidthStr::width(*line))
        .max()
        .unwrap_or(0);

    let colored_ascii_lines: Vec<String> = ascii_lines
        .iter()
        .map(|line| c_blue.apply_to(line).to_string())
        .collect();

    let max_lines = ascii_lines.len().max(info_lines.len());
    for i in 0..max_lines {
        let art_line = colored_ascii_lines.get(i).map(|s| s.as_str()).unwrap_or("");
        let info_line = info_lines.get(i).map(|s| s.as_str()).unwrap_or("");

        println!("{:<width$} {}", art_line, info_line, width = art_width);
    }
}
