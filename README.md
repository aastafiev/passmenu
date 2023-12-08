# Description

A simple passmenu program using rofi.

## Dependencies

- Rust language for building (Packages: expanduser, glob)
- [rofi](https://github.com/davatorium/rofi)
- [pass](https://www.passwordstore.org/)

## Install

1. Build with `cargo build --release`
2. Copy `target/release/passmenu` to `~/.local/bin` or `/usr/bin`
3. `chmod +x your/path/passmenu`
4. If you using i3wm apply your keybindig.

## Usage

After run you will see all your stored password.
Using `rofi -dmenu` in case-insensitive mode.

Press "Enter" on it will copy entity password (in fact `pass show -c`). "Esc" exits passmenu.

"Shifr+Enter" opens detailed menu containing all lines, except password (first line), from selected password entity.
Press "Enter" to copy line from detailed menu. "Esc" or "Alt+Shift+Tab" return to previous menu.
