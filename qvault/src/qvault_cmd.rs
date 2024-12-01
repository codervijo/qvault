use crate::qvault_tui;
use crate::qvault_tui::QvaultTerminal;
use std::str::FromStr;

pub enum QvaultCmdName {
    Help,
    History,
    Exit,
    Search,
    Set,
}

impl ToString for QvaultCmdName {
    fn to_string(&self) -> String {
        match self {
            QvaultCmdName::Help => "help".to_string(),
            QvaultCmdName::History => "history".to_string(),
            QvaultCmdName::Exit => "exit".to_string(),
            QvaultCmdName::Search => "search".to_string(),
            QvaultCmdName::Set => "set".to_string(),
        }
    }
}

impl FromStr for QvaultCmdName {
    type Err = String;

    fn from_str(c: &str) -> Result<Self, Self::Err> {
        // Check if the input doesn't start with `/`
        if !c.starts_with('/') {
            return Ok(QvaultCmdName::Search);
        }

        match c.to_lowercase().as_str() {
            "/help" => Ok(QvaultCmdName::Help),
            "/history" => Ok(QvaultCmdName::History),
            "/exit" => Ok(QvaultCmdName::Exit),
            "/search" => Ok(QvaultCmdName::Search),
            "/set" => Ok(QvaultCmdName::Set),
            _ => Err(format!("Invalid Command {}", c)),
        }
    }
}

pub fn handle_search(_args: &[String], term: &mut QvaultTerminal) {
    term.show_msg("Searching done".to_string());
}

pub fn handle_exit(_args: &[String], term: &mut QvaultTerminal) {
    term.show_msg("Quitting".to_string());
}

pub fn handle_help(_args: &[String], term: &mut QvaultTerminal) {
    term.show_msg("Opening or not".to_string());
}

pub fn handle_history(_args: &[String], term: &mut QvaultTerminal) {
    term.show_msg("History".to_string());
}

pub fn handle_set(_args: &[String], term: &mut QvaultTerminal) {
    term.show_msg("Settings".to_string());
}

impl QvaultCmdName {
    pub fn get_handler(&self) -> fn(&[String], term: &mut QvaultTerminal) {
        match self {
            QvaultCmdName::Search => handle_search,
            QvaultCmdName::Help => handle_help,
            QvaultCmdName::Exit => handle_exit,
            QvaultCmdName::History => handle_history,
            QvaultCmdName::Set => handle_set,
        }
    }
}

pub struct QvaultCmd {
    name: QvaultCmdName,
    args: Vec<String>,
    handler: fn(&[String], &mut QvaultTerminal),
}

impl QvaultCmd {
    pub fn from_input(c: &str) -> Result<Self, String> {
        let mut toks = c.split_whitespace();

        let cmd = toks.next().ok_or("No command found in input")?;
        let qcmd = QvaultCmdName::from_str(cmd)?;

        // Collect the remaining tokens as arguments
        let args = toks.into_iter().map(String::from).collect();
        let handler = qcmd.get_handler();

        Ok(QvaultCmd { name: qcmd, args, handler })
    }

    pub fn handle_cmd(&self, term: &mut QvaultTerminal) {
        (self.handler)(&self.args, term);
    }
}
