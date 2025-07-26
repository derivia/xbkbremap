use anyhow::{anyhow, Context, Result};
use evdev::Device;
use std::io;

pub fn list_keyboards() -> Result<Vec<Device>> {
    let devices = evdev::enumerate()
        .map(|t| t.1)
        .filter(|d| {
            d.supported_keys()
                .map_or(false, |keys| keys.contains(evdev::Key::KEY_ENTER))
        })
        .collect::<Vec<_>>();
    Ok(devices)
}

pub fn choose_keyboard() -> Result<Device> {
    let mut keyboards = list_keyboards()?;
    if keyboards.is_empty() {
        return Err(anyhow!("No keyboards found."));
    }

    println!("Select the keyboard to capture:");
    for (i, device) in keyboards.iter().enumerate() {
        println!(
            "  [{}] {} ({})",
            i,
            device.name().unwrap_or("Nameless device"),
            device.physical_path().unwrap_or("Unknown path")
        );
    }

    print!("Enter the device number: ");
    io::Write::flush(&mut io::stdout())?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let index: usize = input
        .trim()
        .parse()
        .context("Invalid input. Enter a number.")?;

    if index >= keyboards.len() {
        return Err(anyhow!("Invalid device number."));
    }

    let device = keyboards.remove(index);
    Ok(device)
}

pub fn grab_device(device: &mut Device) -> Result<()> {
    device
        .grab()
        .context("Failed to grab keyboard device. Run with sudo or configure udev rules.")?;

    println!(
        "Device '{}' successfully grabbed. Press F12 to exit.",
        device.name().unwrap_or("unknown")
    );

    Ok(())
}
