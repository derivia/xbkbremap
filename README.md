# xbkbremap

An Xbox controller emulator for Linux that maps keyboard keys to buttons on a
virtual controller.

## Prerequisites

1. Add necessary udev rules file.
```
# /etc/udev/rules.d/99-xbkbremap.rules

KERNEL=="uinput", MODE="0660", GROUP="input", OPTIONS+="static\_node=uinput"
```
2. Add yourself to the `input` group.
```
sudo usermod -aG input $USER
```
3. Reload udev rules.
```
sudo udevadm control --reload-rules && sudo udevadm trigger
```

## Usage

1. Compile the project with `cargo build --release`.
2. Copy config.example.json to `~/.config/xbkbremap/config.json` to use it as base configuration file.
3. Edit the copied configuration file to your liking.
4. Run: `./target/release/xbkbremap "profile name"`

## Important notes:

Press F12 to stop the program.

## LICENSE

[MIT](./LICENSE)
