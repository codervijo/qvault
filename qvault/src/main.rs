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
    loop {
        qtui.show_prompt()?;
        let iput = qtui.tui_get_input()?;
        qtui.show_msg(format!("Got input string {}", iput));
        if iput == "exit" {
            break;
        }
    }

    Ok(())
}