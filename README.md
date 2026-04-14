# Pigeon Shot 📷

A lightweight screenshot tool written in Rust using GTK4 and libadwaita.

![Pigeon Shot GIF](./images/pigeon_shot.gif)


## Features

- 🖼️ Full-screen screenshot capture
- 💾 Save to `~/Pictures/Screenshots/` directory
- 📋 Copy to clipboard support
- 🎨 Modern libadwaita UI
- ⚡ Fast and lightweight

## Dependencies / Installation
See [install.sh](/install.sh) for dependencies or installation.

## Print Key Setup

To trigger screenshots with the Print key, set up a keyboard shortcut:

### GNOME Desktop

Add to keyboard shortcuts:
- Settings → Keyboard Shortcuts → Custom Shortcuts
- Command: `pigeon-shot`
- Shortcut: `Print`

### Using xbindkeys

Create `~/.xbindkeysrc`:

```
"pigeon-shot"
    Print
```

Then run:
```bash
xbindkeys
```

### Using systemd/DBus (Alternative)

For a system-wide solution, create a service file.

## License

MIT
