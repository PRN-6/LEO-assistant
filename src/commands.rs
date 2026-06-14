pub enum Command{
    OpenVSCode,
    OpenTerminal,
    OpenFirefox,
    OpenBrave,
    CloseBrave,
    OpenWhatsdesk,
    VolumeUp,
    VolumeDown,
    OpenAntigravity,
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
        "browser" => Command::OpenBrave,
        "close browser" => Command::CloseBrave,
        "message" => Command::OpenWhatsdesk,
        "volume up" => Command::VolumeUp,
        "volume down" => Command::VolumeDown,
        "gravity" => Command::OpenAntigravity,
        "shutdown" => Command::Shutdown,
        "restart" => Command::Restart,
        "sleep" => Command::Sleep,
        _ => Command::Unknown,
    }
}