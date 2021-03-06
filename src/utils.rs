use std::{path::PathBuf, process, str::FromStr};

pub fn parse_cron_frequency(frequency: &str) -> String {
    let cron_str = frequency
        .starts_with("Every(")
        .then(|| remove_cron_freq_prefix(frequency))
        .or_else(|| Some(frequency));
    let cron_str = match cron_str {
        Some(c) => c,
        None => {
            eprintln!("Please enter a valid cron string.");
            process::exit(1);
        }
    };
    cron_str.split(" ").collect::<Vec<&str>>().len().ne(&6).then(||{
        eprintln!("Please enter a valid cron string. Hint: StewardX's cron frequency needs to take 6 crontime inputs, like * * * * * *");
        process::exit(1)
    });
    match cron::Schedule::from_str(cron_str) {
        Ok(_a) => {}
        Err(_e) => {
            eprintln!("Please enter a valid cron string.");
            process::exit(1);
        }
    };
    let parsed_frequency = format!("Every({})", cron_str);
    return parsed_frequency;
}

pub fn remove_cron_freq_prefix(frequency: &str) -> &str {
    let mut chars = frequency.chars();
    while let Some(a) = chars.next() {
        if a.to_string() == "(" {
            break;
        }
    }
    chars.next_back();
    chars.as_str()
}

pub fn truncate_string_elliptic(string: String, to: usize) -> String {
    let mut cloned = string.clone();
    if cloned.len().ge(&(to - 1)) {
        cloned.truncate(to - 1);
        format!("{}…", cloned)
    } else {
        cloned
    }
}

pub fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

pub fn format_date(date: chrono::NaiveDateTime) -> String {
    date.format("%Y-%m-%d %H:%M:%S UTC").to_string()
}

pub fn get_nodejs_compatible_arch() -> &'static str {
    let arch = std::env::consts::ARCH;
    return match arch {
        "x86_64" => "x64",
        "x86" => "x32",
        "aarch64" => "arm64",
        _ => arch
    };
}

pub fn get_binary_dir() -> PathBuf {
    match std::env::var("STEWARDX_DIR") {
        Ok(dir) => match PathBuf::from_str(&dir) {
            Ok(dir) => dir,
            Err(e) => {
                panic!("{}", e);
            }
        },
        Err(_) => {
            match home::home_dir() {
                Some(mut s) => {
                    s.push(".stewardx");
                    s
                },
                None => {
                    return PathBuf::from_str("$HOME/.stewardx").unwrap();
                }
            }
        }
    }
}

pub fn create_stewardx_dirs() {
    match std::fs::create_dir_all(get_binary_dir()) {
        Ok(_) => {}
        Err(e) => {
            panic!("{}", e);
        }
    }
}

pub fn get_socket_path() -> PathBuf {
    let socket_dir = std::env::var("STEWARDX_DIR").unwrap_or_else(|_| String::from("/tmp/"));
    let mut socket_path = PathBuf::from(socket_dir);
    socket_path.push("stewardx.sock");
    socket_path
}
