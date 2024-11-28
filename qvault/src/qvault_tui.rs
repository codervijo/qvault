use std::io::{self, Write};
use termion::raw::{IntoRawMode, RawTerminal};
use termion::cursor;

pub struct QvaultTerminal {
    terminal: RawTerminal<io::Stdout>,
}

impl QvaultTerminal {
    // Constructor to initialize the terminal in raw mode with mouse support
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let mut terminal = io::stdout().into_raw_mode()?; // Convert stdout into raw mode

        Ok(QvaultTerminal { terminal })
    }

    // Getter for the terminal
    //pub fn get_terminal(&mut self) -> RawTerminal<io::Stdout> {
    //    self.terminal
    //}

    pub fn flush(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        //self.terminal.show_cursor()?; // Ensure cursor is shown when shutting down
        let _ = self.terminal.flush();

        Ok(())
    }

    pub fn show_msg(&mut self, msg: String) -> Result<(), Box<dyn std::error::Error>> {
        writeln!(self.terminal, "{}", msg)?;

        Ok(())
    }

    // Method to shut down and restore terminal settings
    pub fn shutdown(mut self) -> Result<(), Box<dyn std::error::Error>> {
        writeln!(self.terminal, "{}", cursor::Show);
        self.terminal.flush()?;
        //self.terminal.show_cursor()?; // Ensure cursor is shown when shutting down
        Ok(())
    }
}


