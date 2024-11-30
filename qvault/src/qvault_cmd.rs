pub enum QvaultCmdName {
    Help,
    History,
    Exit,
    Search,
    Set,
}

use std::str::FromStr;

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
        match c.to_lowercase().as_str() {
            "help" => Ok(QvaultCmdName::Help),
            "/help" => Ok(QvaultCmdName::Help),
            "history" => Ok(QvaultCmdName::History),
            "/history" => Ok(QvaultCmdName::History),
            "exit" => Ok(QvaultCmdName::Exit),
            "/exit" => Ok(QvaultCmdName::Exit),
            "search" => Ok(QvaultCmdName::Search),
            "/search" => Ok(QvaultCmdName::Search),
            "set" => Ok(QvaultCmdName::Set),
            "/set" => Ok(QvaultCmdName::Set),
            _ => Err(format!("Invalid Command {}", c)),
        }
    }
}

pub struct QvaultCmd {
    name: QvaultCmdName,
    args: Vec<String>,
}

impl QvaultCmd {
    pub fn from_input(c: &str) -> Result<Self, String> {
        let mut toks = c.split_whitespace();

        let cmd = toks.next().ok_or("No command found in input")?;
        let qcmd = QvaultCmdName::from_str(cmd)?;

        // Collect the remaining tokens as arguments
        let args = toks.into_iter().map(String::from).collect();

        Ok(QvaultCmd { name: qcmd, args })
    }

    //pub fn handle_cmd() {
    //}
}
