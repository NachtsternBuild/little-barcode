# Little Barcode

A modern, lightweight QR-Code and barcode utility written in Rust using GTK4 and Libadwaita.

Little Barcode allows users to scan, analyze, and generate QR codes and barcodes through a clean desktop interface designed for Linux systems.

---

# Overview

Little Barcode is a GTK4/Libadwaita desktop application focused on simplicity, speed, and native Linux integration.

The application provides:

- QR-Code and barcode scanning
- QR-Code and barcode generation
- Automatic content detection
- Wi-Fi QR-Code support
- URL detection with browser integration
- ISBN and EAN recognition
- Modern GNOME/Libadwaita user experience
- Native Linux dialogs and file handling

The project is built entirely in Rust and uses modern GTK technologies for responsiveness and maintainability.

---

# Features

## Scan QR-Codes and Barcodes

Little Barcode can analyze image files containing:

- QR-Codes
- EAN-13 barcodes
- ISBN codes
- Wi-Fi QR-Codes
- URLs
- Generic text codes

Supported image formats are handled through the `image` crate and decoded using the `rxing` barcode library.

---

## Smart Content Detection

The application automatically detects the type of scanned content.

### Supported Content Types

| Type | Description |
|---|---|
| URL | Opens websites directly in the browser |
| Wi-Fi | Extracts SSID and password |
| EAN-13 | Detects commercial product codes |
| ISBN | Recognizes book identifiers |
| vCard | Detects digital business cards |
| Unknown | Displays raw barcode data |

---

## Wi-Fi Integration

Wi-Fi QR-Codes can be used to connect directly to wireless networks.

Detected information:

- SSID
- Password
- Security type

The application integrates with Linux network management functionality.

---

## QR-Code and Barcode Generation

Little Barcode also includes a generator interface.

Users can create:

- QR-Codes
- Text codes
- URL codes
- Wi-Fi QR-Codes
- Barcode formats

Generated codes can be exported and saved locally.

---

## Native Linux Desktop Experience

Little Barcode uses:

- GTK4
- Libadwaita
- GNOME Human Interface Guidelines

This ensures:

- Responsive layouts
- Native dialogs
- Proper Wayland support
- Modern animations
- Dark mode support
- High DPI compatibility

---

# Technology Stack

## Programming Language

- Rust Edition 2024

## GUI Framework

- GTK4
- Libadwaita

## Barcode Processing

- rxing
- image

## Additional Libraries

| Crate | Purpose |
|---|---|
| regex | Content classification |
| chrono | Timestamps and metadata |
| home | Home directory handling |
| dirs-next | Cross-platform directory lookup |
| atlbase | UI helper and utility framework |

---

# Architecture

The application is structured into modular components.

## Project Structure

```text
src/
├── helper/
│   ├── connect_wifi.rs
│   ├── handler.rs
│   └── regex_check.rs
│
├── ui/
│   ├── core/
│   │   ├── generate_code.rs
│   │   └── stack.rs
│   │
│   └── layout/
│       ├── create_code.rs
│       └── create_home_page.rs
│
├── helper.rs
├── main.rs
└── ui.rs
```

---

# Module Documentation

## `main.rs`

Application entry point.

Responsibilities:

- Initialize Libadwaita application
- Configure syslog integration
- Create the main window
- Build the stack-based navigation system
- Initialize the header bar
- Load the home page

---

## `helper/handler.rs`

Responsible for barcode parsing and image decoding.

Responsibilities:

- Read image files
- Decode QR-Codes
- Decode barcode formats
- Return parsed text content
- Handle decoding errors

---

## `helper/regex_check.rs`

Performs content classification using regular expressions.

Responsibilities:

- URL detection
- Wi-Fi QR-Code parsing
- ISBN validation
- EAN detection
- vCard recognition
- Generic content fallback

---

## `helper/connect_wifi.rs`

Linux Wi-Fi integration module.

Responsibilities:

- Connect to wireless networks
- Parse Wi-Fi credentials
- Trigger NetworkManager functionality

---

## `ui/layout/create_home_page.rs`

Creates the main dashboard.

Features:

- Scan button
- Generate button
- File chooser integration
- Dynamic dialogs
- Content-specific actions

---

## `ui/layout/create_code.rs`

Contains the QR-Code and barcode generator interface.

Responsibilities:

- User input forms
- Code generation actions
- Export dialogs
- Preview handling

---

## `ui/core/stack.rs`

Navigation abstraction for stack-based page switching.

Responsibilities:

- Page transitions
- Navigation management
- Dynamic UI switching

---

## `ui/core/generate_code.rs`

Core generation engine.

Responsibilities:

- QR-Code creation
- Barcode generation
- File export
- Image rendering

---

# User Interface

## Main Window

The application uses:

- `adw::Application`
- `adw::ToolbarView`
- `adw::HeaderBar`
- `gtk4::Stack`

The navigation is implemented using stack transitions.

### Stack Transition

```rust
StackTransitionType::SlideLeftRight
```

This creates smooth animated page navigation.

---

# Workflow

## Scanning Workflow

```text
User selects image
        ↓
Image decoding
        ↓
Content detection
        ↓
Action selection
        ↓
Dialog presentation
```

---

## Generation Workflow

```text
User enters content
        ↓
Generator processes data
        ↓
QR/Barcode created
        ↓
Preview displayed
        ↓
Export image
```

---

# Installation

## Requirements

### System Dependencies

Install GTK4 and Libadwaita development packages.

### Ubuntu/Debian

```bash
sudo apt install \
    build-essential \
    pkg-config \
    libgtk-4-dev \
    libadwaita-1-dev
```

### Fedora

```bash
sudo dnf install \
    gtk4-devel \
    libadwaita-devel
```

### Arch Linux

```bash
sudo pacman -S \
    gtk4 \
    libadwaita
```

---

# Build Instructions

## Clone Repository

```bash
git clone <repository-url>
cd little-barcode
```

## Build Project

```bash
cargo build --release
```

## Run Application

```bash
cargo run
```

---

# Cargo Dependencies

```toml
[dependencies]
regex = "1.12"
rxing = { version = "0.6", features = ["image_formats"] }
image = "0.24"
home = "0.5"
chrono = "0.4"
adw = { version = "0.9.1", package = "libadwaita", features = ["v1_8"] }
gtk4 = { version = "0.11", features = ["v4_20"] }
atlbase = "0.1"
dirs-next = "2"
```

---

# Logging

Little Barcode initializes a syslog domain during startup.

```rust
init_syslog("little_barcode")
```

Benefits:

- Centralized logging
- Better debugging
- System integration
- Runtime diagnostics

---

# Error Handling

The project uses Rust's `Result`-based error handling.

Handled scenarios include:

- Invalid image files
- Unsupported barcode formats
- Parsing failures
- Missing Wi-Fi credentials
- Invalid URLs
- File export failures

---

# Design Goals

The project follows several core principles.

## Simplicity

A minimal interface focused on essential functionality.

## Native Integration

Deep integration with Linux desktop technologies.

## Performance

Efficient barcode decoding and rendering.

## Maintainability

Modular Rust codebase with clean separation of concerns.

## User Experience

Modern Libadwaita-based GNOME design.

---

# Security Considerations

Little Barcode does not:

- Upload scanned data
- Use cloud services
- Track user activity
- Send telemetry

All processing happens locally on the user's system.

---

# Future Improvements

Potential future features:

- Camera-based live scanning
- Drag & drop support
- Clipboard integration
- SVG export
- Batch barcode processing
- More barcode formats
- Mobile Linux support
- Flatpak packaging
- Localization support
- History management

---

# Example Use Cases

## Example 1 — Scan a Website QR-Code

1. Open Little Barcode
2. Select “Scan QR-Code/Barcode”
3. Choose an image file
4. The application detects the URL
5. Open the page directly in the browser

---

## Example 2 — Connect to Wi-Fi

1. Scan a Wi-Fi QR-Code
2. The application extracts credentials
3. Press “Connect”
4. NetworkManager handles the connection

---

## Example 3 — Generate a QR-Code

1. Open the generator page
2. Enter text or URL
3. Generate the code
4. Save the image locally

---

# Development

## Recommended Tools

- Rust nightly/stable
- cargo
- rust-analyzer
- GNOME Builder
- Visual Studio Code

---

# Code Style

The project follows:

- Idiomatic Rust conventions
- Modular architecture
- Clear separation between UI and logic
- Extensive inline documentation

---

# License

This project is licensed under the GNU General Public License v3.0.

See the `LICENSE` file for more information.

---

# Credits

## Technologies

- Rust
- GTK4
- Libadwaita
- rxing
- GNOME Platform

## Project

Little Barcode
by NachtsternBuild

---

# Summary

Little Barcode is a lightweight, modern, and privacy-friendly barcode utility for Linux.

It combines:

- Rust performance
- GTK4 responsiveness
- Libadwaita design
- Native Linux integration
- Smart QR-Code handling

into a clean and maintainable desktop application.

