[![CI](https://github.com/kkent030315/notify-icon-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/kkent030315/notify-icon-rs/actions/workflows/ci.yml)

# notify-icon-rs

A safe, ergonomic Rust wrapper around the Windows `Shell_NotifyIcon` API for managing system tray icons on Windows platforms.

This library provides a high-level interface to create, modify, and manage notification icons in the Windows system tray. It handles the complexities of the underlying Windows API while providing a builder pattern for easy configuration and Rust-style error handling.

## Features

- **Safe wrapper**: Memory-safe abstraction over the raw Windows API
- **Builder pattern**: Fluent, chainable API for configuring notification icons
- **Error handling**: Proper Rust `Result` types instead of raw Windows error codes
- **UTF-16 handling**: Automatic conversion of Rust strings to Windows-compatible UTF-16
- **Version support**: Support for different Windows notification icon interface versions
- **Comprehensive functionality**: Support for tooltips, balloon notifications, GUIDs, and more

## Supported Windows Versions

This library supports Windows 95 and later versions, with enhanced functionality on:
- Windows Vista and later (improved balloon notification behavior)
- Windows 7 and later (additional notification features)

## Platform Requirements

- **Target**: Windows only (`#[cfg(windows)]` should be used when integrating)
- **Dependencies**: Requires the `windows` crate for Windows API bindings

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
notify-icon = "0.1.0"
```

or invoke command:

```bash
cargo add notify-icon
```

## Quick Start

```rust
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::{LoadIconW, WM_USER};
use windows_tray_icon::NotifyIcon;

// Create and configure a notification icon
let icon = NotifyIcon::new()
    .window_handle(hwnd) // Window to receive messages
    .tip("My Application") // Tooltip text
    .icon(icon_handle) // Icon to display
    .callback_message(WM_USER + 1); // Message ID for callbacks

// Add the icon to the system tray
icon.notify_add()?;

// Later, remove the icon
icon.notify_delete()?;
```

## Usage Examples

### Using GUIDs for Icon Persistence

```rust
use windows::core::GUID;

let icon = NotifyIcon::new()
    .window_handle(hwnd)
    .guid(GUID::from_u128(0x12345678_1234_1234_1234_123456789ABC))
    .tip("Persistent Icon")
    .icon(icon_handle);

icon.notify_add()?;
```

### Setting Interface Version for Enhanced Features

```rust
// Use Windows Vista+ behavior
let icon = NotifyIcon::new()
    .window_handle(hwnd)
    .version(3) // NOTIFYICON_VERSION_4
    .tip("Modern Icon");

icon.notify_add()?;
icon.notify_set_version()?; // Apply the version setting
```

### Modifying Existing Icons

```rust
// Change the tooltip of an existing icon
let updated_icon = icon.tip("Updated tooltip text");
updated_icon.notify_modify()?;
```

## Message Handling

When users interact with the notification icon, Windows sends messages to the specified window. Here's a common message handling pattern:

```rust
use windows::Win32::UI::WindowsAndMessaging::{WM_USER, WM_LBUTTONUP, WM_RBUTTONUP};

// In your window procedure
match msg {
    WM_USER + 1 => { // Your callback message
        match lparam {
            x if x == WM_LBUTTONUP as isize => {
                // Handle left click
                println!("Left click on tray icon");
            },
            x if x == WM_RBUTTONUP as isize => {
                // Handle right click - typically show context menu
                show_context_menu();
            },
            _ => {}
        }
    },
    _ => {}
}
```

## Error Handling

All notification operations return `windows::core::Result<()>`.

```rust
match icon.notify_add() {
    Ok(()) => println!("Icon added successfully"),
    Err(e) => eprintln!("Failed to add icon: {}", e),
}
```

## Thread Safety

The `NotifyIcon` struct is safe to use across threads.

## Limitations

- **Windows only**: This library only works on Windows platforms

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

## License

MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built on top of the excellent [`windows`](https://github.com/microsoft/windows-rs) crate
