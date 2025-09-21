mod config;
mod controller;

use anyhow::{anyhow, Context, Result};
use clap::Parser;
use rdev::{listen, EventType, Key};
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;

use crate::config::load_profiles;
use crate::controller::VirtualController;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    profile_name: String,
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

    let controller = Arc::new(Mutex::new(
        VirtualController::new().context("Failed to create virtual controller")?,
    ));

    println!("\n--- Active Mapping ---");
    for (key, button) in &profile.mappings {
        println!("{:?} -> {:?}", key, button);
    }
    println!("------------------------\n");
    println!("Ready! Remapper is active. Press F12 to stop.");

    let (tx, mut rx) = mpsc::channel(1);

    let mappings = Arc::new(profile.mappings);
    let controller_clone = Arc::clone(&controller);

    let listen_task = tokio::task::spawn_blocking(move || {
        let callback = move |event: rdev::Event| {
            let key = match event.event_type {
                EventType::KeyPress(key) => (key, 1),
                EventType::KeyRelease(key) => (key, 0),
                _ => return,
            };

            if key.0 == Key::F12 && key.1 == 1 {
                let _ = tx.blocking_send(());
                return;
            }

            if let Some(xbox_button) = mappings.get(&key.0) {
                let mut controller = controller_clone.lock().unwrap();
                if let Err(e) = controller.handle_button_action(*xbox_button, key.1) {
                    eprintln!("Error sending controller event: {}", e);
                }
            }
        };

        if let Err(error) = listen(callback) {
            eprintln!("Error listening for keyboard events: {:?}", error);
        }
    });

    tokio::select! {
        _ = listen_task => {},
        _ = rx.recv() => {
            println!("F12 key pressed. Exiting...");
        }
    }

    println!("Program terminated.");
    Ok(())
}
