use crate::qvault_tui::{QvaultTerminal}; // Adjust imports based on your module structure
mod qvault_search;
use crate::qvault_log;
use serde::{Serialize, Deserialize};
use std::fmt;
use std::str::FromStr;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum QvaultCmdName {
    Error,
    Help,
    History,
    Exit,
    Search,
    Set,
    Ai,
}

impl FromStr for QvaultCmdName {
    type Err = String;

    fn from_str(c: &str) -> Result<Self, Self::Err> {
        if !c.starts_with('/') {
            return Ok(QvaultCmdName::Search);
        }

        match c.to_lowercase().as_str() {
            "/help" => Ok(QvaultCmdName::Help),
            "/history" => Ok(QvaultCmdName::History),
            "/exit" => Ok(QvaultCmdName::Exit),
            "/search" => Ok(QvaultCmdName::Search),
            "/set" => Ok(QvaultCmdName::Set),
            "/ai" => Ok(QvaultCmdName::Ai),
            _ => Ok(QvaultCmdName::Error),
        }
    }
}

impl fmt::Display for QvaultCmdName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            QvaultCmdName::Help => "Help",
            QvaultCmdName::History => "History",
            QvaultCmdName::Exit => "Exit",
            QvaultCmdName::Search => "Search",
            QvaultCmdName::Set => "Set",
            QvaultCmdName::Ai => "AI",
            QvaultCmdName::Error => "Error",
        };
        write!(f, "{}", name)
    }
}

pub fn handle_search(args: &[String], term: &mut QvaultTerminal) {
    term.show_msg(format!("Searching for args: {:?}", args));
    if !args.is_empty() {
        match qvault_search::search_brave(&args[0]) {
            Ok(result) => {
                term.show_output_title(result.title().to_string());
                term.show_output_message(1, result.snippet().to_string());
            }
            Err(err) => {
                term.show_msg(format!("Search failed: {}", err));
            }
        }
    }
}

pub fn handle_exit(_args: &[String], term: &mut QvaultTerminal) {
    term.show_msg("Quitting".to_string());
}

pub fn handle_help(_args: &[String], term: &mut QvaultTerminal) {
    term.show_msg("Displaying help".to_string());
}

pub fn handle_history(_args: &[String], term: &mut QvaultTerminal) {
    term.show_msg("History command invoked".to_string());
}

pub fn handle_set(_args: &[String], term: &mut QvaultTerminal) {
    term.show_msg("Set command invoked".to_string());
}

pub fn handle_ai(args: &[String], term: &mut QvaultTerminal) {
    term.show_output_title("AI command invoked".to_string());
    qvault_log::log_info("AI ommand executed:", format_args!("{}", args.join(", ")));
}

impl QvaultCmdName {
    pub fn get_handler(&self) -> fn(&[String], &mut QvaultTerminal) {
        match self {
            QvaultCmdName::Search => handle_search,
            QvaultCmdName::Help => handle_help,
            QvaultCmdName::Exit => handle_exit,
            QvaultCmdName::History => handle_history,
            QvaultCmdName::Set => handle_set,
            QvaultCmdName::Ai => handle_ai,
            QvaultCmdName::Error => handle_search,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct QvaultCmd {
    name: QvaultCmdName,
    args: Vec<String>,
}

impl fmt::Display for QvaultCmd {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "QvaultCmd {{ name: {}, args: {:?} }}",
            self.name,
            self.args
        )
    }
}

impl QvaultCmd {
    pub fn from_input(c: &str) -> Result<Self, String> {
        let mut toks = c.split_whitespace();
        let cmd = toks.next().ok_or("No command found in input")?;
        let qcmd = QvaultCmdName::from_str(cmd)?;

        let args = toks.into_iter().map(String::from).collect();

        Ok(QvaultCmd { name: qcmd, args })
    }

    pub fn handle_cmd(&self, term: &mut QvaultTerminal) {
        let handler = self.name.get_handler();
        handler(&self.args, term);
    }

    pub fn log_it(&self) {
        qvault_log::log_info("Command executed:", format_args!("{}", self.name));
    }
}
