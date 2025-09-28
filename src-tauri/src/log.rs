use chrono::{DateTime, Local};
use std::fs::File;
use std::io::Write;

use crate::file_storage;

pub fn error(message: &str) {
    log(message, "ERROR");
}

pub fn info(message: &str) {
    log(message, "INFO");
}

fn log(message: &str, level: &str) {
    let current_local: DateTime<Local> = Local::now();
    let formatted_message = &format!("{} [{}] {}", current_local, level, message);
    if cfg!(dev) {
        log_to_console(formatted_message);
    }
    log_to_file(formatted_message);
}

fn ensure_log_dir(path: &std::path::Path) -> std::io::Result<()> {
    match path.parent() {
        Some(parent) => {
            return std::fs::create_dir_all(parent);
        }
        None => return Ok(()),
    }
}

fn open_log_file(file_path: &std::path::Path) -> Result<File, String> {
    if !file_path.exists() {
        if let Err(e) = ensure_log_dir(&file_path) {
            return Err(format!("Could not create log directory: {}", e));
        }

        if let Err(e) = std::fs::write(&file_path, "") {
            return Err(format!("Could not create log file: {}", e));
        }
    }

    match File::options().append(true).open(file_path) {
        Ok(file) => {
            return Ok(file);
        }
        Err(e) => {
            return Err(format!("Could not open log file: {}", e));
        }
    }
}

fn log_to_file(message: &str) {
    let file_path = file_storage::get_logs_path();

    match open_log_file(&file_path) {
        Ok(mut file) => {
            let result = writeln!(file, "{}", message);
            if let Err(e) = result {
                log_to_console(&format!("Failed to write log: {}", e));
            }
        }
        Err(e) => {
            log_to_console(&e);
        }
    }
}

fn log_to_console(message: &str) {
    println!("{}", message);
}
