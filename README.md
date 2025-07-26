# xbkbremap

An Xbox controller emulator for Linux that maps keyboard keys to buttons on a
virtual controller.

## Prerequisites

1. Add necessary udev rules file.
```
# /etc/udev/rules.d/99-xbkbremap.rules
KERNEL=="uinput", MODE="0660", GROUP="input", OPTIONS+="static_node=uinput"
KERNEL=="event*", NAME="input/%k", MODE="0660", GROUP="input"
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
4. Run with superuser permissions: `./target/release/xbkbremap "profile name"`

## Important notes:

**Press F12 to stop intercepting keyboard.**

You cannot use your keyboard while the program is running.
Which means you should have a setup where you can enter the game you want without using the keyboard.

## Ideas

- [ ] Implement some sort of virtual keyboard that intercepts only the configured keys, instead of capturing the entire keyboard.

## LICENSE

[MIT](./LICENSE)
