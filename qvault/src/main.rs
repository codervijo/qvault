mod qvault_tui;
use qvault_tui::QvaultTerminal;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up terminal
    let mut qtui = qvault_tui::QvaultTerminal::new()?;

    qtui.show_qvault_screen()?;

    // Main event loop
    loop {
        qtui.write_bar_message("Enter search query below");
        qtui.show_prompt()?;
        let iput = qtui.tui_get_input()?;
        qtui.show_msg(format!("Got input string {}", iput));
        if iput == "exit" {
            break;
        }
    }

    qtui.shutdown();

    Ok(())
}
