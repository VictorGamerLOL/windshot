# Windshot

Windshot is a screenshotting application for Linux Wayland, written in Rust and using GTK4 for its graphical interface.

## Plans:
- [x] Working drawing of editing commands
- [x] Working drawing of selection area
- [ ] Display on the overlay layer of any compositor supporting wlr-layer-shell
<!--TODO-->


## Requirements
- Linux (Wayland session)
- Rust (for building from source)
- GTK4 development libraries

## Building

### Arch Linux (Recommended)

Windshot currently requires the latest GTK4, which is best supported on Arch Linux.

1. Install Rust:
   ```fish
   sudo pacman -S rust
   ```
2. Install GTK4 and dependencies:
   ```fish
   sudo pacman -S gtk4
   ```
3. Clone the repository:
   ```fish
   git clone https://github.com/yourusername/windshot.git
   cd windshot
   ```
4. Build the project:
   ```fish
   cargo build --release
   ```

Other distributions may not have a recent enough GTK4 to compile Windshot successfully.

## Running
After building, run the application:
```fish
cargo run --release
```
Or execute the binary from `target/release/windshot`.

## License
GPL-3.0-or-later
