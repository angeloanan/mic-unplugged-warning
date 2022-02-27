use std::time::Duration;

use chrono::Utc;
use cpal::traits::DeviceTrait;
use rodio::{source::Source, Decoder, OutputStream};

fn main() {
    use cpal::traits::HostTrait;

    println!("");
    println!("> Unplugged Microphone Notifier");
    println!("Notifies whenever your mic is unplugged");
    println!("---");
    println!("Created by Christopher Angelo <angelo@angeloanan.xyz>");
    println!("");

    let initial_input_devices = cpal::default_host()
        .input_devices()
        .expect("No input device available");
    let input_devices_names: Vec<String> = initial_input_devices
        .into_iter()
        .map(|device| device.name().unwrap())
        .collect();

    println!("");
    println!("Select an Input Device to watch:");
    let mut select = cli_select::Select::new(&input_devices_names);
    let selected_input_device = select.start();

    println!("");
    println!(
        "You selected {}. Checking for every 5 seconds!",
        selected_input_device
    );

    loop {
        std::thread::sleep(Duration::from_millis(5000));

        let timestamp = Utc::now().to_rfc3339();
        let mut has_input_device = false;

        println!("[{}] Checking mic input...", timestamp);

        let input_devices = cpal::default_host()
            .input_devices()
            .expect("No input device available");

        for device in input_devices {
            if device.name().unwrap() == selected_input_device.clone() {
                has_input_device = true;
            }
        }

        if !has_input_device {
            println!(
                "[{}] Audio input {} is gone. Creating notification...",
                timestamp, selected_input_device
            );
            send_notification();
        } else {
            println!(
                "[{}] Audio input {} is still there.",
                timestamp, selected_input_device
            );
        }
    }
}

fn send_notification() {
    notify_rust::Notification::new()
        .appname("Mic unplugged")
        .summary("Mic unplugged")
        .body("Your mic is unplugged, u bafoon")
        .timeout(1000)
        .show()
        .unwrap();

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    let audio_bytes = include_bytes!(".././resource/warning.wav");
    let audio_buffer = std::io::Cursor::new(audio_bytes.as_ref());
    let audio_source = Decoder::new_wav(audio_buffer).unwrap();
    stream_handle
        .play_raw(audio_source.convert_samples())
        .unwrap();

    std::thread::sleep(std::time::Duration::from_secs(5));
}
