use log::{Level, Metadata, Record};
use std::fs::{File, OpenOptions};
use std::sync::Mutex;
use std::io::Write;
use once_cell::sync::Lazy; // For lazy initialization

struct FileLogger {
    file: Mutex<File>,
}

impl log::Log for FileLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info // Change level as needed
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            if let Ok(mut file) = self.file.lock() {
                let log_entry = format!(
                    "{} - [{}] {}\n",
                    chrono::Utc::now().to_rfc3339(),
                    record.level(),
                    record.args()
                );
                let _ = file.write_all(log_entry.as_bytes());
            }
        }
    }

    fn flush(&self) {
        if let Ok(mut file) = self.file.lock() {
            let _ = file.flush();
        }
    }
}

// Lazy initialization of the logger
static LOGGER: Lazy<FileLogger> = Lazy::new(|| {
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("qvault.log")
        .expect("Failed to open log file");
    FileLogger {
        file: Mutex::new(file),
    }
});

/// Initialize the file logger
pub fn init_log() {
    log::set_logger(&*LOGGER)
        .map(|()| log::set_max_level(log::LevelFilter::Info))
        .expect("Failed to set logger");
}

/// Convenience functions for logging
use std::fmt::{self, Arguments};

pub fn log_info(message: &str, args: Arguments) {
    let formatted_message = format!("{}", args); // Convert Arguments to string
    // Assuming logging to a file or stdout
    log::info!("{}", formatted_message); // Print or log the formatted message
}


pub fn log_warn(message: String) {
    log::warn!("{}", message);
}

pub fn log_error(message: String) {
    log::error!("{}", message);
}
