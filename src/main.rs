const DEFAULT_ASCII_ART: &str = "(\\/)\n\
                                 (^w^)\n\
                                 (()()";

const DEFAULT_CONFIG: &str = ":userhost,\
                              os:os,\
                              cpu:cpu,\
                              mem:memory,";

use std::path::Path;
use std::fs;
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

fn get_user() -> String {
    if let Ok(username) = whoami::username() {
        return username
    } else {
        return "unknown".to_string()
    }
}


fn get_hostname() -> String {
    if let Ok(hostname) = whoami::hostname() {
        return hostname
    } else {
        return "unknown".to_string()
    }
}

fn get_uptime() -> String {
    if let Ok(uptime) = System::uptime() {
        if uptime / 3600 / 24 > 0 {
            return format!("{} days, {} hrs", uptime / 3600 /24, uptime / 3600 % 24);
        }

        if uptime / 3600 > 0 {
            return format!("{} hrs, {} mins", uptime / 3600, uptime / 60 % 60);
        }
        
        if uptime / 60 > 0 {
            return format!("{} mins, {} secs", uptime / 60, uptime % 60);
        }

        return format!("{} secs", uptime);
    } else {
        return "unknown".to_string();
    }
}

// self-descriptive
fn get_longest_line_length(text: &String) -> usize {
    let lines = text.split("\n");
    
    return lines.max_by_key(|x| x.len()).unwrap().len();
}

// gets padding for a sting
fn pad(text: &str, length: usize) -> String {
    return format!("{}{}", text, " ".repeat(length - text.len()));
}

fn print_info(line: &str, line_length: usize, info_title: String, title_length: usize, info_content:String) {
    if info_title.is_empty() {
        println!("{}  \x1b[1m{}\x1b[22m", pad(line, line_length), info_content);
    } else {
        println!("{}  \x1b[1m{}\x1b[22m {}", pad(line, line_length), pad(&info_title, title_length), info_content);
    }
}

fn get_ascii_art() -> String {
    // use content of env var PAWFETCH_ASCIIART if it exists
    if let Ok(env_ascii_art) = std::env::var("PAWFETCH_ASCIIART") {
        return env_ascii_art;
    }
    
    if let Some(mut path) = std::env::home_dir() {
        path.push(".config/pawfetch/ascii_art");
        if path.exists() && let Ok(config_ascii_art) = fs::read_to_string(path) {
            return config_ascii_art;
        }
    }
    
    // if all else fails, use default ascii art
    return DEFAULT_ASCII_ART.to_string();
}

fn get_info_lines() -> Option<Vec<(String, String)>> {
    let mut info_lines: Vec<(String, String)> = Vec::new();
    
    let mut info_lines_string: String = DEFAULT_CONFIG.to_string();
    
    if let Some(mut path) = std::env::home_dir() {
        path.push(".config/pawfetch/infolines");
        if path.exists() && let Ok(config_result) = fs::read_to_string(path) {
            if !config_result.is_empty() && config_result.contains(":") {
                info_lines_string = config_result;
            }
        }
    }

    if let Ok(env_result) = std::env::var("PAWFETCH_INFOLINES") {
        if !env_result.is_empty() && env_result.contains(":") {
            info_lines_string = env_result.to_string();
        }
    }

    let split = info_lines_string.split(",");
        
    for line in split {
        line.trim()
            .split_once(":")
            .map(|(title, content)|
                info_lines.push(
                    (title.to_string(), content.to_string())
                )
            );
    }

    return Some(info_lines);
}

fn get_info(info_string: String) -> String {
    match info_string.as_str() {
        "os"        => get_os(),
        "cpu"       => get_cpu(),
        "memory"    => get_memory(),
        "swap"      => get_swap(),
        "user"      => get_user(),
        "hostname"  => get_hostname(),
        "userhost"  => format!("{}@{}",get_user(),get_hostname()),
        "uptime"    => get_uptime(),
        ""          => "".to_string(),
        _           => "unknown".to_string(),
    }
}

fn main() {
    let info_lines: Vec<(String, String)>;
    match get_info_lines() {
        Some(lines) => info_lines = lines,
        None => info_lines = Vec::new(),
    }

    let ascii_art = get_ascii_art();

    // gets the longest line in the ascii art
    let max_length = get_longest_line_length(&ascii_art);
    
    let ascii_lines = ascii_art.split("\n");

    let infotitle_max_len = info_lines.clone().into_iter().max_by_key(|x| x.0.len()).unwrap().0.len();
    let mut info_lines_iter = info_lines.into_iter();

    for line in ascii_lines {
        let info_line = info_lines_iter.next();

        if info_line != None {
            let (info_title, info_content) = info_line.unwrap();
            print_info(line, max_length, info_title, infotitle_max_len, get_info(info_content));
        } else {
            println!("{}", line);
        }
    }

    for line in info_lines_iter {
        let (info_title, info_content) = line;
        print_info("", max_length, info_title, infotitle_max_len, get_info(info_content));
    }
}
