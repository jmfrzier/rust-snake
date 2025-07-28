use rodio::{Decoder, OutputStream, Sink};
use std::{fs::File, io::BufReader};

// Simple approach: create audio stream each time we play a sound
fn play(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let (_stream, handle) = OutputStream::try_default()?;
    let file = File::open(path)?;
    let source = Decoder::new(BufReader::new(file))?;
    let sink = Sink::try_new(&handle)?;

    sink.append(source);
    sink.sleep_until_end();

    Ok(())
}

pub fn init_audio() -> Result<(), Box<dyn std::error::Error>> {
    // Test that audio system works
    OutputStream::try_default()?;
    Ok(())
}

pub fn play_eat_sound() {
    if let Err(e) = play("assets/eat.wav") {
        eprintln!("Failed to play eat sound: {}", e);
    }
}

pub fn play_die_sound() {
    if let Err(e) = play("assets/die.wav") {
        eprintln!("Failed to play die sound: {}", e);
    }
}
