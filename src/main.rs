mod config;
mod controller;
mod input;

use anyhow::{anyhow, Context, Result};
use clap::Parser;
use evdev::{EventType, Key};
use std::io::{self, Write};
use std::thread;
use std::time::Duration;
use termios::{tcsetattr, Termios, ECHO, ICANON, TCSANOW};

use crate::config::load_profiles;
use crate::controller::VirtualController;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    profile_name: String,
}

fn set_raw_mode() -> io::Result<Termios> {
    let fd = 0;
    let mut termios = Termios::from_fd(fd)?;
    let orig = termios.clone();
    termios.c_lflag &= !(ICANON | ECHO);
    tcsetattr(fd, TCSANOW, &termios)?;
    Ok(orig)
}

fn restore_mode(orig: &Termios) -> io::Result<()> {
    let fd = 0;
    tcsetattr(fd, TCSANOW, orig)?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    println!("Starting xbkbremap...");

    let profiles = load_profiles().context("Failed to load configuration profiles")?;
    let profile = profiles
        .into_iter()
        .find(|p| p.name == args.profile_name)
        .ok_or_else(|| anyhow!("Profile '{}' not found.", args.profile_name))?;

    println!("Profile '{}' loaded.", profile.name);

    let mut keyboard = input::choose_keyboard().context("Failed to select keyboard")?;

    input::grab_device(&mut keyboard).context("Failed to capture keyboard")?;

    let mut controller = VirtualController::new().context("Failed to create virtual controller")?;

    println!("\n--- Active Mapping ---");
    for (key, button) in &profile.mappings {
        println!("{:?} -> {:?}", key, button);
    }
    println!("------------------------\n");
    println!("Ready! Remapper is active. Press F12 to stop.");

    let orig_termios = set_raw_mode().context("Failed to set terminal to raw mode")?;

    let task = tokio::task::spawn_blocking(move || -> Result<()> {
        // FIXME: After F12 is pressed, the program should exit gracefully without needing to press Ctrl + C.
        loop {
            let mut had_event = false;
            match keyboard.fetch_events() {
                Ok(events) => {
                    for event in events {
                        if event.event_type() != EventType::KEY {
                            continue;
                        }

                        let key = Key(event.code());
                        let value = event.value();

                        if key == Key::KEY_F12 && value == 1 {
                            print!("F12 key pressed. Exiting...\r");
                            io::stdout().flush().ok();
                            return Ok(());
                        }

                        if let Some(xbox_button) = profile.mappings.get(&key) {
                            if value == 0 || value == 1 {
                                if let Err(e) = controller.handle_button_action(*xbox_button, value)
                                {
                                    eprintln!("Error sending controller event: {}", e);
                                }
                            }
                        }
                        had_event = true;
                    }
                }
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {}
                Err(e) => {
                    eprintln!("Error reading keyboard events: {}", e);
                    return Err(e.into());
                }
            }

            if !had_event {
                thread::sleep(Duration::from_millis(1));
            }
        }
    });

    task.await??;

    restore_mode(&orig_termios).context("Failed to restore terminal mode")?;

    println!("Program terminated. Devices released.");
    Ok(())
}
