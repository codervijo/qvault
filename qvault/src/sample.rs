use std::io::{self, Write};
use termion::event::{Event, Key};
use termion::input::{TermRead};
use termion::raw::{IntoRawMode, RawTerminal};
use termion::cursor;

type Terminal = RawTerminal<io::Stdout>;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up terminal
    let mut terminal = io::stdout().into_raw_mode()?; // Convert stdout into raw mode

    // Display the main menu
    display_main_menu(&mut terminal)?;

    // Main event loop
    for evt in io::stdin().events() {
        match evt? {
            Event::Key(Key::Char('q')) => break, // Exit on 'q'
            Event::Key(Key::Char('1')) => keyboard_demo(&mut terminal)?,
            Event::Key(Key::Char('3')) => cursor_movement_demo(&mut terminal)?,
            Event::Key(Key::Char('4')) => horizontal_lines_demo(&mut terminal)?,
            Event::Key(Key::Char('5')) => horizontal_bar_demo(&mut terminal)?,
            _ => continue,
        }
        display_main_menu(&mut terminal)?;
    }

    Ok(())
}

fn display_main_menu(terminal: &mut Terminal) -> io::Result<()> {
    write!(
        terminal,
        "{}{}Welcome to Termion Demo!{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        cursor::Hide
    )?;
    writeln!(terminal, "Select an option:")?;
    writeln!(terminal, "1. Keyboard Input Demo")?;
    writeln!(terminal, "3. Cursor Movement Demo")?;
    writeln!(terminal, "4. Horizontal Lines Demo")?;
    writeln!(terminal, "5. Horizontal Bar Demo")?;
    writeln!(terminal, "Press 'q' to quit.")?;
    terminal.flush()
}

fn keyboard_demo(terminal: &mut Terminal) -> io::Result<()> {
    write!(
        terminal,
        "{}{}Keyboard Input Demo - Press keys, 'q' to quit{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        cursor::Hide
    )?;
    terminal.flush()?;

    for evt in io::stdin().events() {
        match evt? {
            Event::Key(Key::Char('q')) => break,
            Event::Key(key) => writeln!(terminal, "Key pressed: {:?}", key)?,
            _ => continue,
        }
        terminal.flush()?;
    }

    Ok(())
}

fn cursor_movement_demo(terminal: &mut Terminal) -> io::Result<()> {
    write!(
        terminal,
        "{}{}Cursor Movement Demo - Observe cursor changes, 'q' to quit{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        cursor::Hide
    )?;
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
        write!(terminal, "{}*", termion::cursor::Goto(x, y))?;
        terminal.flush()?;
        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    Ok(())
}

fn horizontal_lines_demo(terminal: &mut Terminal) -> io::Result<()> {
    write!(
        terminal,
        "{}{}Horizontal Lines Demo:{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        cursor::Hide
    )?;
    writeln!(terminal, "Showing horizontal lines:")?;

    for i in 1..10 {
        write!(terminal, "{}{}", termion::cursor::Goto(1, i * 2), "-".repeat(30))?;
        terminal.flush()?;
    }

    Ok(())
}

fn horizontal_bar_demo(terminal: &mut Terminal) -> io::Result<()> {
    write!(
        terminal,
        "{}{}Horizontal Bar Demo:{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        cursor::Hide
    )?;
    writeln!(terminal, "Showing horizontal bar with color:")?;

    write!(
        terminal,
        "{}{}{}",
        termion::color::Bg(termion::color::Red),
        termion::style::Bold,
        " ".repeat(30)
    )?;
    terminal.flush()?;

    std::thread::sleep(std::time::Duration::from_secs(2));

    Ok(())
}
