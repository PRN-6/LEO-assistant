pub enum Command{
    OpenVSCode,
    OpenTerminal,
    OpenFirefox,
    Shutdown,
    Restart,
    Sleep,
    Unknown,
}

pub fn parse_command(input: &str) -> Command {
    match input.to_lowercase().trim(){
        "open vscode" => Command::OpenVSCode,
        "open terminal" => Command::OpenTerminal,
        "open firefox" => Command::OpenFirefox,
        "shutdown" => Command::Shutdown,
        "restart" => Command::Restart,
        "sleep" => Command::Sleep,
        _ => Command::Unknown,
    }
}