use cpal::{traits::DeviceTrait, Device};

fn main() {
    use cpal::traits::HostTrait;
    let host = cpal::default_host();

    let input_devices = host.input_devices().expect("no input device available");

    // Print all input devices
    for device in input_devices {
        println!("{}", device.name().unwrap());
    }
}
