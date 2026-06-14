// Run with: cargo run --bin list_devices
use cpal::traits::{DeviceTrait, HostTrait};

fn main() {
    let host = cpal::default_host();
    println!("=== Input Devices ===");
    for d in host.input_devices().unwrap() {
        println!("  {:?}", d.name());
    }
    println!("=== Default Input ===");
    if let Some(d) = host.default_input_device() {
        println!("  {:?}", d.name());
    }
}
