const ASCII_ART: &str = "(\\/)\n\
                         (^w^)\n\
                         (()()";

use std::path::Path;
use os_release::OsRelease;
use sysinfo::System;

// functions for obtaining system info
fn get_os() -> String {
    // check if os release exists
    if Path::new("/etc/os-release").exists() {
        let osrelease = match OsRelease::new() {
            Err(_) => return std::env::consts::OS.to_string(),
            Ok(osrelease) => osrelease,
        };

        return osrelease.pretty_name.to_string().to_lowercase();
    } else {
        return std::env::consts::OS.to_string();
    }
}

fn get_cpu() -> String {
    if let Ok(mut sys) = System::new() {
        sys.refresh_cpu_all();
        return sys.cpus()[0].brand().to_string().to_lowercase();
    } else {
        return "unknown".to_string();
    }
}

fn get_memory() -> String {
    if let Ok(mut sys) = System::new() {
        sys.refresh_memory();
        return format!("{} gib", sys.total_memory() / 1024 / 1024 / 1024);
    } else {
        return "unknown".to_string();
    }
}

fn get_swap() -> String {
    if let Ok(mut sys) = System::new() {
        sys.refresh_memory();
        return format!("{} gib", sys.total_swap() / 1024 / 1024 / 1024);
    } else {
        return "unknown".to_string();
    }
}

// self-descriptive
fn get_longest_line_length(text: &str) -> usize {
    let lines = text.split("\n");
    
    return lines.max_by_key(|x| x.len()).unwrap().len();
}

// gets padding for a sting
fn pad(text: &str, length: usize) -> String {
    return format!("{}{}", text, " ".repeat(length - text.len()));
}

fn print_info(line: &str, line_length: usize, info_title: &str, title_length: usize, info_content:String) {
    println!("{}  \x1b[1m{}\x1b[22m {}", pad(line, line_length), pad(info_title, title_length), info_content);
}

fn main() {
    // ("title", info_function)
    let info_lines = [
        ("os", get_os()),
        ("cpu", get_cpu()),
        ("mem", get_memory()),
        ("swp", get_swap())
    ];

    // gets the longest line in the ascii art
    let max_length = get_longest_line_length(ASCII_ART);
    
    let ascii_lines = ASCII_ART.split("\n");

    let infotitle_max_len = info_lines.clone().into_iter().max_by_key(|x| x.0.len()).unwrap().0.len();
    let mut info_lines_iter = info_lines.into_iter();

    for line in ascii_lines {
        let info_line = info_lines_iter.next();

        if info_line != None {
            let (info_title, info_content) = info_line.unwrap();
            print_info(line, max_length, info_title, infotitle_max_len, info_content);
        } else {
            println!("{}", line);
        }
    }

    for line in info_lines_iter {
        let (info_title, info_content) = line;
        print_info("", max_length, info_title, infotitle_max_len, info_content);
    }
}
