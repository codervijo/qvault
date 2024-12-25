use sled::{Db, IVec};
use std::str;
use chrono::{DateTime, Utc};

#[derive(Clone)]
pub struct QvaultHistory {
    db: Db,
}

/// Global database path
static DB_PATH: &str = "qvault_history.db";

impl QvaultHistory {
    /// Opens or creates a database file
    pub fn new() -> Result<Self, sled::Error> {
        let db = sled::open(DB_PATH)?;
        Ok(Self {
            db
        })
    }

    /// Adds a command to the history. Key is auto-incrementing.
    pub fn add_command(&self, command: &str) -> Result<(), sled::Error> {
        let id = self.db.generate_id()?; // Generates a unique ID for the key.
        self.db.insert(id.to_be_bytes(), command.as_bytes())?;
        Ok(())
    }

    /// Retrieves the entire command history as a vector of (key, command) pairs.
    pub fn get_history(&self) -> Result<Vec<(u64, String)>, sled::Error> {
        let mut commands = Vec::new();
        for item in self.db.iter() {
            let (key, value) = item?;
            let id = u64::from_be_bytes(key.as_ref().try_into().unwrap());

            // Handle Utf8Error explicitly
            let command = match str::from_utf8(&value) {
                Ok(cmd) => cmd.to_string(),
                Err(err) => {
                    eprintln!("Error decoding command: {}", err);
                    continue; // Skip invalid entries
                }
            };
            commands.push((id, command));
        }
        Ok(commands)
    }
    

    /// Clears the entire command history.
    pub fn clear_history(&self) -> Result<(), sled::Error> {
        self.db.clear()?;
        Ok(())
    }

    /// Displays the command history to the console.
    pub fn display_history(&self) -> Result<(), sled::Error> {
        let history = self.get_history()?;
        for (id, cmd) in history {
            println!("{:>3}: {}", id, cmd);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_history() {
        let db_path = "testdb.db";
        let history = QvaultHistory::new(db_path).expect("Failed to create database");

        // Add commands
        history.add_command("ls").unwrap();
        history.add_command("cd /").unwrap();
        history.add_command("pwd").unwrap();

        // Verify history retrieval
        let commands = history.get_history().unwrap();
        assert_eq!(
            commands.into_iter().map(|(_, cmd)| cmd).collect::<Vec<_>>(),
            vec!["ls", "cd /", "pwd"]
        );

        // Clear history and verify
        history.clear_history().unwrap();
        assert!(history.get_history().unwrap().is_empty());

        // Clean up the test database
        std::fs::remove_dir_all(db_path).unwrap();
    }
}
