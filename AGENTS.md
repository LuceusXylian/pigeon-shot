# Pigeon Shot Guidelines

## Code Style
- Rust 2021 edition with standard snake_case naming
- GTK4 UI patterns: builder pattern for widgets, `glib::clone!()` macro for weak reference capture in closures
- Error handling: `Result<T, Box<dyn std::error::Error>>` for flexibility
- No Command usage. Do not use any external programms to capture screenshots. Use wayland standards to capture screenshots.
- No bullshit like `return minimal 1x1 Pixbuf instead of erroring`
- Change tests only if asked

## Architecture
- Rust 1.70+
- GTK4 + libadwaita for modern GNOME integration
- Linux only & Wayland only
- GNOME or KDE
- Wayland screenshot capture via DBus (GNOME Shell service primary, XDG Portal fallback)
- Data flow: Capture → Pixbuf → Preview window → Save/Copy/Close actions

## Conventions
- Create screenshots with the `wlroots` crate of the current monitor (where the cursor is).
- Screenshots saved to `~/Pictures/Screenshots/` with timestamp format `screenshot_YYYYMMDD_HHMMSS.png`
- App ID: `io.github.pigeonshot` (reverse domain for GNOME/Wayland)
- Graceful fallbacks: GNOME DBus → XDG Portal
- Temporary files for Wayland captures cleaned up automatically

See [README.md](README.md) for detailed build instructions, dependencies, and usage.