mod qvault_cmd;
mod qvault_tui;
mod qvault_log;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    qvault_log::init_log();

    // Set up terminal
    let mut qtui = qvault_tui::QvaultTerminal::new()?;

    qtui.show_qvault_screen()?;

    // Main event loop
    loop {
        qvault_log::log_info("Looping ..", format_args!(""));
        let _ = qtui.write_bar_message("Enter search query below");
        qtui.show_prompt()?;
        let iput = qtui.tui_get_input()?;
        qvault_log::log_info("Got input from User >>", format_args!("{}", iput));
        let qcmd = qvault_cmd::QvaultCmd::from_input(&iput)?;
        qcmd.clone().log_it();
        qcmd.handle_cmd(&mut qtui);
        //let _ = qtui.show_msg(format!("Got input string {}", iput));
        if iput == "exit" {
            break;
        }
    }

    qvault_log::log_info("Exiting from qvault session", format_args!(""));
    qtui.shutdown();
    qvault_log::shutdown_log();

    Ok(())
}
