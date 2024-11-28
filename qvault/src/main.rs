use std::io::{self, Write};
use termion::event::{Event, Key};
use termion::input::{TermRead};
use termion::raw::{IntoRawMode, RawTerminal};
use termion::cursor;

mod qvault_tui;
use qvault_tui::QvaultTerminal;

type Terminal = RawTerminal<io::Stdout>;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up terminal
    let mut qtui = qvault_tui::QvaultTerminal::new()?;

    qtui.show_qvault_screen()?;

    // Main event loop
    qtui.show_prompt()?;
    for evt in io::stdin().events() {
        qtui.show_prompt()?;
        match evt? {
            Event::Key(Key::Char('q')) => break, // Exit on 'q'
            _ => continue,
        }
    }

    Ok(())
}

fn display_main_menu(terminal: &mut QvaultTerminal) -> Result<(), Box<dyn std::error::Error>> {
    /*write!(
        terminal,
        "{}{}Welcome to Termion Demo!{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        cursor::Hide
    )?;*/
    terminal.show_msg(format!("{}{}Welcome to Termion Demo!{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        cursor::Hide))?;
    terminal.show_msg("Select an option:".to_string())?;
    terminal.show_msg("1. Keyboard Input Demo".to_string())?;
    terminal.show_msg("3. Cursor Movement Demo".to_string())?;
    terminal.show_msg("4. Horizontal Lines Demo".to_string())?;
    terminal.show_msg("5. Horizontal Bar Demo".to_string())?;
    terminal.show_msg("Press 'q' to quit.".to_string())?;
    terminal.flush()?;

    Ok(())
}

fn keyboard_demo(terminal: &mut QvaultTerminal) -> Result<(), Box<dyn std::error::Error>> {
    /*write!(
        terminal,
        "{}{}Keyboard Input Demo - Press keys, 'q' to quit{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        cursor::Hide
    )?;*/
    terminal.show_msg(format!("{}{}Keyboard Input Demo - Press keys, 'q' to quit{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        cursor::Hide))?;
    terminal.flush()?;

    for evt in io::stdin().events() {
        match evt? {
            Event::Key(Key::Char('q')) => break,
            Event::Key(key) => terminal.show_msg(format!("Key pressed: {:?}", key))?,
            _ => continue,
        }
        terminal.flush()?;
    }

    Ok(())
}

fn cursor_movement_demo(terminal: &mut QvaultTerminal) -> Result<(), Box<dyn std::error::Error>> {
    /*write!(
        terminal,
        "{}{}Cursor Movement Demo - Observe cursor changes, 'q' to quit{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        cursor::Hide
    )?;*/
    terminal.show_msg(format!("{}{}Cursor Movement Demo - Observe cursor changes, 'q' to quit{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        cursor::Hide))?;
    terminal.flush()?;

    let positions = vec![
        (5, 5),
        (10, 5),
        (15, 5),
        (15, 10),
        (10, 10),
        (5, 10),
        (5, 5),
    ];

    for (x, y) in positions {
        /* TODO write!(terminal, "{}*", termion::cursor::Goto(x, y))?; */
        terminal.flush()?;
        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    Ok(())
}

fn horizontal_lines_demo(terminal: &mut QvaultTerminal) -> Result<(), Box<dyn std::error::Error>> {
    /*write!(
        terminal,
        "{}{}Horizontal Lines Demo:{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        cursor::Hide
    )?; */
    terminal.show_msg(format!("{}{}Horizontal Lines Demo:{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        cursor::Hide))?;
    terminal.show_msg("Showing horizontal lines:".to_string())?;

    for i in 1..10 {
        terminal.show_msg(format!("{}{}", termion::cursor::Goto(1, i * 2), "-".repeat(30)))?;
        terminal.flush()?;
    }

    Ok(())
}

fn horizontal_bar_demo(terminal: &mut QvaultTerminal) -> Result<(), Box<dyn std::error::Error>> {
    /*write!(
        terminal,
        "{}{}Horizontal Bar Demo:{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        cursor::Hide
    )?; */
    terminal.show_msg(format!("{}{}Horizontal Bar Demo:{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        cursor::Hide))?;
    terminal.show_msg(format!("Showing horizontal bar with color:"))?;

    /*write!(
        terminal,
        "{}{}{}",
        termion::color::Bg(termion::color::Red),
        termion::style::Bold,
        " ".repeat(30)
    )?; */
    terminal.show_msg(format!("{}{}{}",
        termion::color::Bg(termion::color::Red),
        termion::style::Bold,
        " ".repeat(30)))?;
    terminal.flush()?;

    std::thread::sleep(std::time::Duration::from_secs(2));

    Ok(())
}
