mod commands;
mod executor;
mod config;

use commands::{parse_command, Command};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use vosk::{Model, Recognizer, DecodingState};
use std::sync::{Arc, Mutex};

fn main() {
    println!("Leo assistant started - listening for voice commands...");

    let cfg = config::load_config();

    // Load the Vosk model from the model/ folder
    let model = Model::new("model").expect("Could not find the model folder");
    let recognizer = Arc::new(Mutex::new(
        Recognizer::new(&model, 16000.0).expect("Could not create recognizer"),
    ));

    let host = cpal::default_host();

    // Prefer PipeWire (modern audio server) → laptop mic → fallback default
    let preferred = ["pipewire", "sysdefault:CARD=acp63", "default:CARD=acp63"];
    let device = host.input_devices()
        .expect("Could not enumerate input devices")
        .find(|d| {
            d.name()
                .map(|n| preferred.iter().any(|p| n == *p))
                .unwrap_or(false)
        })
        .or_else(|| host.default_input_device())
        .expect("No input device found");

    println!("Using microphone: {}", device.name().unwrap_or("Unknown".to_string()));

    let config = cpal::StreamConfig {
        channels: 1,
        sample_rate: cpal::SampleRate(16000),
        buffer_size: cpal::BufferSize::Default,
    };

    let rec_clone = Arc::clone(&recognizer);

    // Move cfg into an Arc so it can be shared with the audio thread
    let cfg = Arc::new(cfg);
    let cfg_clone = Arc::clone(&cfg);

    let stream = device.build_input_stream(
        &config,
        move |data: &[i16], _| {
            let mut rec = rec_clone.lock().unwrap();
            if let Ok(DecodingState::Finalized) = rec.accept_waveform(data) {
                let result = rec.result();
                if let Some(single) = result.single() {
                    let text = single.text;
                    if !text.is_empty() {
                        println!("Heard: {}", text);

                        let action_key = match parse_command(text) {
                            Command::OpenVSCode   => Some("vscode"),
                            Command::OpenTerminal => Some("terminal"),
                            Command::OpenFirefox  => Some("firefox"),
                            Command::OpenBrave    => Some("brave"),
                            Command::CloseBrave   => Some("close_brave"),
                            Command::OpenWhatsdesk => Some("whatsdesk"),
                            Command::VolumeUp     => Some("volume_up"),
                            Command::VolumeDown   => Some("volume_down"),
                            Command::OpenAntigravity => Some("antigravity"),
                            Command::Shutdown     => Some("shutdown"),
                            Command::Restart      => Some("restart"),
                            Command::Sleep        => Some("sleep"),
                            Command::Unknown      => { println!("Unknown command"); None }
                        };

                        if let Some(key) = action_key {
                            if let Some(cmd_line) = cfg_clone.apps.get(key) {
                                executor::launch(cmd_line);
                            }
                        }
                    }
                }
            }
        },
        |err| eprintln!("Stream error: {}", err),
        None,
    ).expect("Failed to build audio stream");

    stream.play().expect("Failed to start audio stream");

    // Keep the program running forever
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
