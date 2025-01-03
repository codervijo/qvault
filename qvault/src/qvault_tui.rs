use std::io::{self, Write};
use termion::raw::{IntoRawMode, RawTerminal};
use termion::{clear, cursor, color};
use termion::{
    event::{Event, Key},
    input::TermRead,
};

pub struct QvaultTerminal {
    terminal: RawTerminal<io::Stdout>,
    input_row: u16,
    input_col: u16,
    last_input_row: u16,
    last_input_col: u16,
    output_row: u16,
    hbar_row: u16,
}

impl QvaultTerminal {
    // Constructor to initialize the terminal in raw mode with mouse support
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let terminal = io::stdout().into_raw_mode()?; // Convert stdout into raw mode

        Ok(QvaultTerminal {
            terminal,
            input_row: 1,
            input_col: 1,
            last_input_col: 0,
            last_input_row: 0,
            output_row: 1,
            hbar_row: 1,
        })
    }

    pub fn flush(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        //self.terminal.show_cursor()?; // Ensure cursor is shown when shutting down
        let _ = self.terminal.flush();

        Ok(())
    }

    pub fn show_msg(&mut self, msg: String) -> Result<(), Box<dyn std::error::Error>> {
        writeln!(self.terminal, "{}{}", cursor::Goto(1, 2), msg)?;
        self.terminal.flush()?;

        Ok(())
    }

    pub fn show_output_title(&mut self, title: String) -> Result<(), Box<dyn std::error::Error>> {
        let title = if title.is_empty() {
            "SEARCH RESULTS".to_string()
        } else {
            title
        };

        // Get terminal size
        let (term_width, _) = termion::terminal_size().unwrap();

        // Title decorations and formatting
        let decoration = "\u{1F340}\u{1F340}\u{1F340}\u{1F340}\u{1F340}";
        let full_title = format!("{} {} {}", decoration, title.to_uppercase(), decoration);

        // Calculate the starting column to center the title
        let start_col;
        if full_title.len() < term_width.into() {
            start_col = (term_width as usize - full_title.len()) / 2;
        } else {
            start_col = 1;
        }

        // Print the title at the centered position
        writeln!(
            self.terminal,
            "{}{}{}{}",
            cursor::Goto(start_col as u16, self.output_row + 1), // Move cursor to centered position
            "\x1b[1m",                                           // Start bold text
            full_title,                                          // Full title with decorations
            "\x1b[0m"                                            // Reset text formatting
        ).unwrap();

        self.terminal.flush()?;

        Ok(())
    }

    pub fn show_output_message(&mut self, line: u16, msg: String) -> Result<(), Box<dyn std::error::Error>> {
        writeln!(
            self.terminal,
            "{}{}{}",
            cursor::Goto(1, self.output_row + 5+1+line), // Move cursor to the correct position
            "\u{1F7E2}  ", // Left border decoration
            msg,                 // Title in uppercase
        )?;
        self.terminal.flush()?;

        Ok(())
    }

    pub fn show_output_url(&mut self, url: &str) {
        // Move the cursor to the specified position
        write!(self.terminal, "{}", cursor::Goto(1, self.output_row+15)).unwrap();

        // Use the OSC 8 escape sequence to create a clickable hyperlink
        // Format: \x1b]8;;<URL>\x1b\\<text>\x1b]8;;\x1b\\
        let text = format!("{} URL : {}", "\u{1F7E2}", url);
        write!(
            self.terminal,
            "\x1b]8;;{}\x1b\\{}\x1b]8;;\x1b\\",
            url,        // The actual URL
            text // The text to display
        )
        .unwrap();

        // Flush the terminal to ensure the output is written
        self.terminal.flush().unwrap();
    }


    pub fn show_output_nav(&mut self, num_results: usize) -> Result<(), Box<dyn std::error::Error>> {
        // Safely get terminal dimensions
        let (width, height) = termion::terminal_size()?;

        // Center the "num Search Results"
        let results_text = format!("{num_results} Search Results");
        let center_x = (width / 2) as u16;
        //let row = (height / 2) as u16;
        let text_x = center_x.saturating_sub((results_text.len() / 2) as u16); // Safe subtraction

        // Print the centered "num Search Results"
        write!(
            self.terminal,
            "{}{}{}",
            cursor::Goto(text_x, self.hbar_row-2),
            color::Fg(color::Blue),
            results_text
        )?;

        if num_results > 1 {
            // Print the "Next Page →" aligned to the right
            let right_text = "Next Page → ";
            let right_x = width.saturating_sub(right_text.len() as u16);
            write!(
                self.terminal,
                "{}{}{}{}{}",
                cursor::Goto(right_x, self.hbar_row-2),
                color::Fg(color::Green),
                Self::style("highlight"),
                right_text,
                Self::style("reset")
            )?;

            // Move cursor to navigate search output
            write!(self.terminal, "{}", cursor::Goto(right_x, self.hbar_row-2))?;
        }
        self.terminal.flush()?;

        Ok(())
    }

    pub fn navigate_search_output(&mut self) -> Result<i16, Box<dyn std::error::Error>> {
        let mut count = 0;
        // Start reading events from the terminal
        for event in io::stdin().events() {
            match event? {
                Event::Key(Key::Char('\n')) => {
                    count += 1;
                    break;
                }
                Event::Key(Key::Right) => {
                    count += 1;
                    break;
                }
                Event::Key(Key::Down) => {
                    count += 1;
                    break;
                }
                Event::Key(Key::Char('\t')) => {
                    count = 0;
                    break;
                }
                Event::Key(Key::Backspace) => {
                    count -= 1;
                    break;
                }
                Event::Key(Key::Left) => {
                    count -= 1;
                    break;
                }
                _ => {}
            }
        }

        self.terminal.flush()?;

        Ok(count)
    }

    fn draw_horizontal_bar(&mut self, width: u16) -> Result<(), Box<dyn std::error::Error>> {
        write!(self.terminal, "\x1b[48;5;12m")?; // Set background to light blue
        for _ in 0..width {
            write!(self.terminal, " ")?; // Fill the line with spaces
        }
        Ok(())
    }

    pub fn show_qvault_screen(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        //writeln!(self.terminal, "{}", msg)?;
        // Get the terminal size (columns and rows)
        let (width, height) = termion::terminal_size()?;

        // Calculate the row position near the bottom
        self.hbar_row = height - 1;
        self.input_row = self.hbar_row + 1;

        // Move cursor to the calculated row position and column 1 (start of the line)
        write!(
            self.terminal,
            "{}{}{}",
            termion::clear::All,            // Clear the screen
            cursor::Goto(1, self.hbar_row), // Move cursor
            "\x1b[48;5;12m" // Set the background to light blue (color code 12 in 256 palette)
        )?;
        // Create a horizontal line of 80 characters of light blue (adjust the width if needed)
        for _ in 0..width {
            write!(self.terminal, " ")?; // Print spaces to fill the line
        }

        // Reset the color to default
        write!(self.terminal, "{}{}", "\x1b[0m", cursor::Show)?;

        // Move the cursor to the next line but go to column 1 (column 0 in Rust)
        write!(
            self.terminal,
            "{}{}",
            cursor::Down(1),
            cursor::Goto(1, self.input_row)
        )?;

        self.flush()?;

        Ok(())
    }

    pub fn clear_output_screen(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let (width, height) = termion::terminal_size()?; // Get terminal size
        self.hbar_row = height - 1; // Horizontal bar at the second-to-last row
        self.input_row = self.hbar_row + 1; // Input row below the bar

        // Clear the screen and move cursor to the horizontal bar row
        write!(
            self.terminal,
            "{}{}",
            clear::All,
            cursor::Goto(1, self.hbar_row)
        )?;

        // Draw the horizontal bar with a blue background
        self.draw_horizontal_bar(width)?;

        // Reset terminal colors and position cursor for input
        write!(
            self.terminal,
            "{}{}{}",
            "\x1b[0m",             // Reset color to default
            cursor::Show,          // Show the cursor
            cursor::Goto(1, self.input_row) // Move to input row
        )?;

        self.flush()?; // Ensure everything is written to the terminal

        Ok(())
    }

    pub fn show_prompt(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Move to the input row and clear the current line
        write!(
            self.terminal,
            "{}{}\u{1F50D}>",                // Clear the line and display the prompt
            cursor::Goto(1, self.input_row), // Move to input row, column 1
            clear::CurrentLine               // Clear the entire current line
        )?;
        self.flush()?;
        self.input_col = 3;

        Ok(())
    }

    pub fn write_bar_message(&mut self, msg: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Save the current cursor position
        write!(self.terminal, "{}", cursor::Save)?;

        // Move the cursor to the horizontal bar row and the calculated column
        write!(
            self.terminal,
            "{}{}\u{1F30D}  \u{1F4DA}  {}{}",
            cursor::Goto(1, self.hbar_row), // Move cursor
            "\x1b[1;37m\x1b[48;5;12m",      // Set bold white text with light blue background
            msg,                            // Write the message
            "\x1b[0m"                       // Reset text style
        )?;

        //println!("Search \u{1F50D}");
        // Get the terminal size (width and height)
        let (width, _) = termion::terminal_size()?;

        // Move the cursor to the last column of the first row
        write!(
            self.terminal,
            "{}\u{1F5C4}", // Unicode for file cabinet emoji
            cursor::Goto(width - 2, self.hbar_row)
        )?;

        // Restore the cursor to its original position
        write!(self.terminal, "{}", cursor::Restore)?;

        self.terminal.flush()?;
        Ok(())
    }

    fn tui_draw_box(&mut self, x: u16, y: u16, width: u16, height: u16) {
        // Unicode codes for box-drawing characters
        let horizontal = "\u{2500}"; // ─
        let vertical = "\u{2502}";   // │
        let top_left = "\u{250C}";   // ┌
        let top_right = "\u{2510}";  // ┐
        let bottom_left = "\u{2514}";// └
        let bottom_right = "\u{2518}";// ┘

        // Draw top and bottom borders
        for i in 0..width {
            write!(self.terminal, "{}{}", cursor::Goto(x + i, y), horizontal).unwrap(); // Top border
            write!(self.terminal, "{}{}", cursor::Goto(x + i, y + height - 1), horizontal).unwrap(); // Bottom border
        }

        // Draw left and right borders
        for i in 0..height {
            write!(self.terminal, "{}{}", cursor::Goto(x, y + i), vertical).unwrap(); // Left border
            write!(self.terminal, "{}{}", cursor::Goto(x + width - 1, y + i), vertical).unwrap(); // Right border
        }

        // Draw corners
        write!(self.terminal, "{}{}", cursor::Goto(x, y), top_left).unwrap(); // Top-left corner
        write!(self.terminal, "{}{}", cursor::Goto(x + width - 1, y), top_right).unwrap(); // Top-right corner
        write!(self.terminal, "{}{}", cursor::Goto(x, y + height - 1), bottom_left).unwrap(); // Bottom-left corner
        write!(self.terminal, "{}{}", cursor::Goto(x + width - 1, y + height - 1), bottom_right).unwrap(); // Bottom-right corner

        self.terminal.flush().unwrap();
    }

    /// Resets terminal styles
    fn reset_code() -> &'static str {
        "\x1b[0m"
    }

    /// Returns the ANSI color code string
    fn style(style: &str) -> &'static str {
        match style {
            "cyan" => "\x1b[36m",
            "yellow" => "\x1b[33m",
            "blue_bg" => "\x1b[44m",
            "green" => "\x1b[32m",
            "red" => "\x1b[31m",
            "bold" => "\x1b[1m",           // Bold text
            "dim" => "\x1b[2m",            // Dim text
            "highlight" => "\x1b[7m",      // Reverse video (background/foreground swap)
            "reset" => "\x1b[0m",          // Reset all styles
            _ => "\x1b[0m",                // Default to reset
        }
    }

    fn render_settings_ui(
        &mut self,
        username: &str,
        password: &str,
        active_field: usize,
    ) -> io::Result<()> {
        //write!(self.terminal, "{}{}", clear::All, cursor::Goto(1, 1))?;
        //self.clear_output_screen();

        // Title
        /*
        write!(
            self.terminal,
            "{}{}Login Form{}",
            Self::style("bold"),
            cursor::Goto(10, 1),
            Self::style("reset")
        )?;
        */

        // Username field
        //self.draw_box(18, 5, 33, 3, active_field == 0)?;
        self.tui_draw_box(18, 5, 33, 3);
        write!(
            self.terminal,
            "{}{}{}Brave API Key:{}{}",
            cursor::Goto(19, 5),
            Self::style(if active_field == 0 { "highlight" } else { "dim" }),
            Self::style("cyan"),
            Self::style("reset"),
            Self::style("reset"),
        )?;
        write!(
            self.terminal,
            "{}{: <20}",
            cursor::Goto(19, 6),
            username,
        )?;

        // Password field
        //self.draw_box(18, 9, 33, 3, active_field == 1)?;
        self.tui_draw_box(18, 9, 33, 3);
        write!(
            self.terminal,
            "{}{}{}OpenAI API Key:{}{}",
            cursor::Goto(19, 9),
            Self::style(if active_field == 1 { "highlight" } else { "dim" }),
            Self::style("cyan"),
            Self::style("reset"),
            Self::style("reset"),
        )?;
        write!(
            self.terminal,
            "{}{: <20}",
            cursor::Goto(19, 10),
            password
        )?;

        // Cancel button
        write!(
            self.terminal,
            "{}{}[ {}Cancel{} ]{}",
            cursor::Goto(10, 15),
            Self::style(if active_field == 2 { "highlight" } else { "dim" }),
            Self::style("yellow"),
            Self::style("reset"),
            Self::style("reset")
        )?;

        // Submit button
        write!(
            self.terminal,
            "{}{}[ {}Submit{} ]{}",
            cursor::Goto(45, 15),
            Self::style(if active_field == 3 { "highlight" } else { "dim" }),
            Self::style("green"),
            Self::style("reset"),
            Self::style("reset")
        )?;

        self.terminal.flush()
    }

    pub fn tui_show_settings(&mut self) -> io::Result<()> {
        // Set up raw mode for terminal
        let stdin = io::stdin();

        // Input fields and states
        let mut username = String::new();
        let mut password = String::new();
        let mut active_field = 0; // 0: username, 1: password, 2: cancel, 3: submit

        self.render_settings_ui(&username, &password, active_field)?;

        // Input loop
        for key in stdin.keys() {
            match key? {
                Key::Char('\t') => {
                    // Tab to switch fields
                    active_field = (active_field + 1) % 4;
                }
                Key::Char('\n') => {
                    // Enter to submit
                    if active_field == 3 {
                        write!(
                            self.terminal,
                            "{}{}\nSubmitted! Username: {}, Password: {}\nPress any key to exit.",
                            cursor::Goto(1, 15),
                            clear::AfterCursor,
                            username,
                            password
                        )?;
                        self.terminal.flush()?;
                        break;
                    }

                    // Enter to cancel
                    if active_field == 2 {
                        write!(
                            self.terminal,
                            "{}{}\nCancelled!!",
                            cursor::Goto(1, 15),
                            clear::AfterCursor
                        )?;
                        self.terminal.flush()?;
                        break;
                    }
                }
                Key::Backspace => {
                    // Backspace for editing
                    if active_field == 0 && !username.is_empty() {
                        username.pop();
                    } else if active_field == 1 && !password.is_empty() {
                        password.pop();
                    }
                }
                Key::Char(c) => {
                    // Append characters to the active field
                    if active_field == 0 {
                        username.push(c);
                    } else if active_field == 1 {
                        password.push(c);
                    }
                }
                Key::Esc => break, // Exit on Esc key
                _ => {}
            }
            self.render_settings_ui(&username, &password, active_field)?;
        }

        Ok(())
    }

    pub fn tui_show_help(&mut self, help: Vec<String>) {
        // Get terminal size
        let (term_width, term_height) = termion::terminal_size().unwrap();

        // Box dimensions
        let box_width = 60;
        let box_height = 15;

        // Calculate top-left corner to center the box
        let x = (term_width - box_width) / 2;
        let y = (term_height - box_height) / 2;

        // Draw the rectangle
        self.tui_draw_box(x, y, box_width, box_height);
        let mut line = 1;

        for s in help {
            writeln!(self.terminal, "{}{}", cursor::Goto(x+5, y+line+3), s);
            line += 1;
        }
    }

    pub fn tui_get_input(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        // Create a buffer to store the user input
        let mut buffer = String::new();

        // Inform the user
        //write!(self.terminal, "{}Reading input in raw mode, press 'q' to quit{}",     cursor::Goto(1, 1), cursor::Hide)?;
        self.terminal.flush()?;

        // Start reading events from the terminal
        for event in io::stdin().events() {
            match event? {
                Event::Key(Key::Char('\n')) => break, // Stop at Enter key
                Event::Key(Key::Char(c)) => {
                    // Add character to the buffer
                    buffer.push(c);
                }
                Event::Key(Key::Backspace) => {
                    // Remove the last character from the buffer
                    buffer.pop();
                }
                _ => {}
            }

            // Display the current input in the terminal
            write!(
                self.terminal,
                "{}{}",
                cursor::Goto(self.input_col, self.input_row),
                buffer
            )?;
            self.terminal.flush()?;
        }

        // Show the final input once user presses 'q'
        //write!(self.terminal, "{}You entered: {}", cursor::Goto(1, 4), buffer)?;
        self.terminal.flush()?;

        // Return the collected input
        Ok(buffer)
    }

    // Method to shut down and restore terminal settings
    pub fn shutdown(mut self) {
        let _ = write!(
            self.terminal,
            "{}{}{}",
            termion::clear::All,
            cursor::Show,
            cursor::Goto(1, 1)
        );
        let _ = self.terminal.flush();
        //self.terminal.show_cursor()?; // Ensure cursor is shown when shutting
    }
}
