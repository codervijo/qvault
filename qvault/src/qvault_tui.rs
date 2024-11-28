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

    pub fn show_qvault_screen(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        //writeln!(self.terminal, "{}", msg)?;
        // Get the terminal size (columns and rows)
        let (width, height) = termion::terminal_size()?;

        // Calculate the row position near the bottom
        let row_position = height - 1;

        // Move cursor to the calculated row position and column 1 (start of the line)
        write!(
            self.terminal,
            "{}{}{}",
            termion::clear::All,          // Clear the screen
            cursor::Goto(1, row_position), // Move cursor to 2/3 from the bottom
            "\x1b[48;5;12m"               // Set the background to light blue (color code 12 in 256 palette)
        )?;
        // Create a horizontal line of 80 characters of light blue (adjust the width if needed)
        for _ in 0..width {
            write!(self.terminal, " ")?;  // Print spaces to fill the line
        }

        // Reset the color to default
        write!(self.terminal, "{}{}", "\x1b[0m", cursor::Show)?;

        // Move the cursor to the next line but go to column 1 (column 0 in Rust)
        write!(self.terminal, "{}{}", cursor::Down(1), cursor::Goto(1, row_position + 1))?;

        self.terminal.flush();

        Ok(())
    }

    pub fn show_prompt(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        write!(self.terminal, ">")?;
        self.terminal.flush();

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


