use std::error::Error;
use std::path::Path;
use std::sync::mpsc;
use std::time::Duration;
use std::{env, io, thread};

use crossterm::cursor::{Hide, Show};
use crossterm::event::{self, Event, KeyCode};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{terminal, ExecutableCommand};
use invaders::frame::new_frame;
use invaders::render::render;
use invaders::{frame, render};
use rusty_audio::Audio;

fn main() -> Result<(), Box<dyn Error>> {
    let here = env::current_dir()?;
    let base_sounds_dir = Path::new("").join(&here).join("sounds");

    // Setup audio

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

    // Setup terminal
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    // Render loop in separate thread
    // channel to communicate
    // we can use crossbeam ones in a real project
    let (render_tx, render_rx) = mpsc::channel();
    let render_handler = thread::spawn(move || {
        let mut last_frame = frame::new_frame();
        let mut stdout = io::stdout();
        render::render(&mut stdout, &last_frame, &last_frame, true);

        loop {
            let curr_frame = match render_rx.recv() {
                Ok(x) => x,
                Err(_) => break,
            };
            render::render(&mut stdout, &last_frame, &curr_frame, false);
            last_frame = curr_frame;
        }
    });

    // Game Loop
    'gameloop: loop {
        // Per-frame init
        let curr_frame = new_frame();

        // Input
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    // exit
                    KeyCode::Esc | KeyCode::Char('q') => {
                        audio.play("lose");
                        break 'gameloop;
                    }
                    _ => {}
                }
            }
        }

        // Draw & render
        // first few times it can fail because it will start doing it before the thread set up,
        // so the rx will not exist yet
        let _ = render_tx.send(curr_frame);
        thread::sleep(Duration::from_millis(1));
    }

    // Cleanup
    drop(render_tx); // maybe unnecessary in newst Rust
    render_handler.join().unwrap();
    audio.wait();
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
