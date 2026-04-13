#!/bin/bash

# Pigeon Shot - Installation script

set -e

echo "🐦 Pigeon Shot Installation"
echo "=============================="

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust is not installed. Please install Rust from https://rustup.rs/"
    exit 1
fi

# Check dependencies
echo "Checking dependencies..."

# Detect package manager
if command -v apt &> /dev/null; then
    echo "Installing dependencies (Debian/Ubuntu)..."
    sudo apt install -y libgtk-3-dev libgtk-4-dev libadwaita-1-dev libgbm-dev
elif command -v dnf &> /dev/null; then
    echo "Installing dependencies (Fedora)..."
    sudo dnf install -y gtk4-devel gtk3-devel libadwaita-devel libgbm-devel
elif command -v pacman &> /dev/null; then
    echo "Installing dependencies (Arch)..."
    sudo pacman -S gtk3 gtk4 libadwaita libgbm
else
    echo "⚠️  Could not detect package manager. Please install:"
    echo "   - GTK 3 development files"
    echo "   - GTK 4 development files"
    echo "   - libadwaita development files"
fi

# Build the application
echo ""
echo "Building Pigeon Shot..."
cargo build --release

BINARY="target/release/pigeon-shot"

if [ -f "$BINARY" ]; then
    echo "✓ Build successful!"
else
    echo "❌ Build failed"
    exit 1
fi

# Ask for installation location
read -p "Install to /usr/local/bin/pigeon-shot? (y/n) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    sudo cp "$BINARY" /usr/local/bin/pigeon-shot
    chmod +x /usr/local/bin/pigeon-shot
    echo "✓ Installed to /usr/local/bin/pigeon-shot"

    # Install desktop entry
    sudo cp pigeonshot.desktop /usr/share/applications/
    echo "✓ Desktop entry installed"
else
    echo "You can run the binary at: ./$BINARY"
fi


echo ""
echo "🎉 Installation complete!"
echo ""
echo "Usage:"
echo "  pigeon-shot              # Launch application"
echo ""
echo "For more information, see README.md"
