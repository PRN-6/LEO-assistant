mod commands;
mod executor;

use std::io;

use commands::{parse_command,Command};

fn main() {
    println!("Leo assistent started");

    loop{
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("execution failed");

        match parse_command(&input) {
            Command::OpenVSCode => executor::open_vscode(),
            Command::OpenTerminal => executor::open_terminal(),
            Command::OpenFirefox => executor::open_firefox(),
            Command::Shutdown => executor::shutdown(),
            Command::Restart => executor::restart(),
            Command::Sleep => executor::sleep(),
            Command::Unknown => println!("Unknown command"),
        }
    }
}
