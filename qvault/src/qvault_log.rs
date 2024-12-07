use log::{Level, Metadata, Record};
use std::fs::{File, OpenOptions};
use std::sync::Mutex;
use std::io::{Write, BufWriter};
use once_cell::sync::Lazy; // For lazy initialization
use chrono::Utc;
use log::Log;
use std::fmt::Arguments;

struct FileLogger {
    file: Option<Mutex<BufWriter<File>>>, // Use Option to safely allow closing the file
}

impl log::Log for FileLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            if let Some(ref file_mutex) = self.file.as_ref() { // Borrow the file safely
                if let Ok(mut file) = file_mutex.lock() {
                    let log_entry = format!(
                        "{} - [{}] {}\n",
                        Utc::now().to_rfc3339(),
                        record.level(),
                        record.args()
                    );
                    let _ = file.write_all(log_entry.as_bytes());
                    let _ = file.flush();
                }
            }
        }
    }

    fn flush(&self) {
        if let Some(ref file_mutex) = self.file.as_ref() { // Safely borrow
            if let Ok(mut file) = file_mutex.lock() {
                let _ = file.flush();
            }
        }
    }
}

impl Drop for FileLogger {
    fn drop(&mut self) {
        if let Some(ref file_mutex) = self.file.as_ref() {
            if let Ok(mut file) = file_mutex.lock() {
                let _ = file.flush();
            }
        }
        println!("Logger dropped, file closed.");
    }
}

// Lazy initialization of the logger
static LOGGER: Lazy<FileLogger> = Lazy::new(|| {
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("/var/log/qvault.log")
        .expect("Failed to open log file");
    FileLogger {
        file: Some(Mutex::new(BufWriter::new(file))),
    }
});

/// Initialize the file logger
pub fn init_log() {
    log::set_logger(&*LOGGER)
        .map(|()| log::set_max_level(log::LevelFilter::Info))
        .expect("Failed to set logger");
}

/// Shutdown and flush the logs before exiting
pub fn shutdown_log() {
    if let Some(ref file_mutex) = LOGGER.file.as_ref() {
        if let Ok(mut file) = file_mutex.lock() {
            let _ = file.flush();
            println!("Shutdown and flushed log data.");
        }
    }
}

pub fn log_info(message: &str, args: Arguments) {
    let formatted_message = format!("{}{}", message, args); // Convert Arguments to string
    // Assuming logging to a file or stdout
    log::info!("{}", formatted_message); // Print or log the formatted message
    LOGGER.flush();
}


pub fn log_warn(message: String) {
    log::warn!("{}", message);
}

pub fn log_error(message: String) {
    log::error!("{}", message);
}