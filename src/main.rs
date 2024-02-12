use std::env;
use std::error::Error;
use std::path::Path;

use rusty_audio::Audio;

fn main() -> Result<(), Box<dyn Error>> {
    let here = env::current_dir()?;
    let base_sounds_dir = Path::new("").join(&here).join("sounds");

    let mut audio = Audio::new();
    audio.add(
        "explode",
        Path::new("").join(&base_sounds_dir).join("explode.wav"),
    );
    audio.add(
        "lose",
        Path::new("").join(&base_sounds_dir).join("lose.wav"),
    );
    audio.add(
        "move",
        Path::new("").join(&base_sounds_dir).join("move.wav"),
    );
    audio.add("pew", Path::new("").join(&base_sounds_dir).join("pew.wav"));
    audio.add(
        "startup",
        Path::new("").join(&base_sounds_dir).join("./startup.wav"),
    );
    audio.add("win", Path::new("").join(&base_sounds_dir).join("win.wav"));
    audio.play("startup");
    audio.wait();
    Ok(())
}
