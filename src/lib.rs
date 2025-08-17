//! # notify-icon-rs
//!
//! A safe, ergonomic Rust wrapper around the Windows `Shell_NotifyIcon` API for
//! managing system tray icons on Windows platforms.
//!
//! This library provides a high-level interface to create, modify, and manage
//! notification icons in the Windows system tray. It handles the complexities
//! of the underlying Windows API while providing a builder pattern for easy
//! configuration and Rust-style error handling.
//!
//! ## Features
//!
//! - **Safe wrapper**: Memory-safe abstraction over the raw Windows API
//! - **Builder pattern**: Fluent, chainable API for configuring notification
//!   icons
//! - **Error handling**: Proper Rust [`Result`] types instead of raw Windows
//!   error codes
//! - **UTF-16 handling**: Automatic conversion of Rust strings to
//!   Windows-compatible UTF-16
//! - **Version support**: Support for different Windows notification icon
//!   interface versions
//! - **Comprehensive functionality**: Support for tooltips, balloon
//!   notifications, GUIDs, and more
//!
//! ## Supported Windows Versions
//!
//! This library should supports Windows 95 and later versions, with enhanced
//! functionality on:
//! - Windows Vista and later (improved balloon notification behavior)
//! - Windows 7 and later (additional notification features)
//!
//! ## Platform Requirements
//!
//! - **Target**: Windows only (`#[cfg(windows)]` should be used when
//!   integrating)
//! - **Dependencies**: Requires the `windows` crate for Windows API bindings
//!
//! ## Basic Usage
//!
//! ```rust,no_run,ignore
//! use windows::Win32::Foundation::HWND;
//! use windows::Win32::UI::WindowsAndMessaging::LoadIconW;
//! use windows::Win32::UI::WindowsAndMessaging::WM_USER;
//! use notify_icon::NotifyIcon;
//!
//! const IDI_APP_ICON: PCWSTR = PCWSTR(101 as *const u16);
//!
//! // Create and configure a notification icon
//! let icon = unsafe { LoadIconW(hinstance, IDI_APP_ICON) }.unwrap_or_default();
//! let icon = NotifyIcon::new()
//!     .window_handle(hwnd)                    // Window to receive messages
//!     .tip("My Application")                  // Tooltip text
//!     .icon(icon_handle)                      // Icon to display
//!     .callback_message(WM_USER + 1);        // Message ID for callbacks
//!
//! // Add the icon to the system tray
//! icon.notify_add()?;
//!
//! // Later, remove the icon
//! icon.notify_delete()?;
//! ```
//!
//! ## Advanced Usage
//!
//! ### Using GUIDs for Icon Persistence
//!
//! ```rust,no_run,ignore
//! use windows::core::GUID;
//!
//! let icon = NotifyIcon::new()
//!     .window_handle(hwnd)
//!     .guid(GUID::from_u128(0x12345678_1234_1234_1234_123456789ABC))
//!     .tip("Persistent Icon")
//!     .icon(icon_handle);
//!
//! icon.notify_add()?;
//! ```
//!
//! ### Setting Interface Version for Enhanced Features
//!
//! ```rust,no_run,ignore
//! use notify_icon::NotifyIcon;
//!
//! // Use Windows Vista+ behavior
//! let icon = NotifyIcon::new()
//!     .window_handle(hwnd)
//!     .version(3)  // NOTIFYICON_VERSION_4
//!     .tip("Modern Icon");
//!
//! icon.notify_add()?;
//! icon.notify_set_version()?;  // Apply the version setting
//! ```
//!
//! ### Modifying Existing Icons
//!
//! ```rust,no_run,ignore
//! // Change the tooltip of an existing icon
//! let updated_icon = icon.tip("Updated tooltip text");
//! updated_icon.notify_modify()?;
//! ```
//!
//! ## Message Handling
//!
//! When users interact with the notification icon, Windows sends messages to
//! the specified window. Common message handling pattern:
//!
//! ```rust,no_run,ignore
//! // In your window procedure
//! match msg {
//!     WM_USER + 1 => {  // Your callback message
//!         match lparam {
//!             WM_LBUTTONUP => {
//!                 // Handle left click
//!             },
//!             WM_RBUTTONUP => {
//!                 // Handle right click - typically show context menu
//!             },
//!             _ => {}
//!         }
//!     },
//!     _ => {}
//! }
//! ```
//!
//! ## Error Handling
//!
//! All notification operations return [`windows::core::Result<()>`].
//!
//! ## Thread Safety
//!
//! The [`NotifyIcon`] struct is safe to use across threads.
//!
//! ## Limitations
//!
//! - **Windows only**: This library only works on Windows platforms

use windows::{
    Win32::{
        Foundation::{FALSE, HWND},
        UI::{
            Shell::{
                NIF_GUID, NIF_ICON, NIF_MESSAGE, NIF_SHOWTIP, NIF_TIP, NIM_ADD, NIM_DELETE,
                NIM_MODIFY, NIM_SETFOCUS, NIM_SETVERSION, NOTIFY_ICON_DATA_FLAGS,
                NOTIFY_ICON_MESSAGE, NOTIFYICONDATAW, Shell_NotifyIconW,
            },
            WindowsAndMessaging::HICON,
        },
    },
    core::GUID,
};

/// A wrapper around the Windows NOTIFYICONDATAW structure for managing system
/// tray icons in Windows.
pub struct NotifyIcon {
    /// Underlying internal data.
    data: NOTIFYICONDATAW,
}

impl Default for NotifyIcon {
    fn default() -> Self {
        Self {
            data: NOTIFYICONDATAW {
                cbSize: std::mem::size_of::<NOTIFYICONDATAW>() as _,
                ..Default::default()
            },
        }
    }
}

impl NotifyIcon {
    /// Creates a new [NotifyIcon] instance with default values.
    ///
    /// This is equivalent to calling [`NotifyIcon::default()`].
    ///
    /// # Returns
    ///
    /// A new [`NotifyIcon`] instance with the [`NOTIFYICONDATAW::cbSize`] field
    /// properly initialized.
    pub fn new() -> NotifyIcon {
        Self::default()
    }

    /// Sets a flag in the notification icon data structure.
    ///
    /// This method uses a bitwise OR operation to add the specified flag to the
    /// existing flags in the [`NOTIFYICONDATAW::uFlags`].
    ///
    /// # Arguments
    ///
    /// * `flag` - A [`NOTIFY_ICON_DATA_FLAGS`] value to be added to the current
    ///   flags
    ///
    /// # Returns
    ///
    /// Self for method chaining
    pub fn flag(mut self, flag: NOTIFY_ICON_DATA_FLAGS) -> Self {
        self.data.uFlags |= flag;
        self
    }

    /// Sets the window handle that will receive notification messages.
    ///
    /// This method specifies the window that will receive callback messages
    /// when the user interacts with the notification icon. The window
    /// handle is required for the notification icon to function properly.
    ///
    /// # Arguments
    ///
    /// * `handle` - A handle ([HWND]) to the window that will receive
    ///   notification messages
    ///
    /// # Returns
    ///
    /// Self for method chaining
    pub fn window_handle(mut self, handle: HWND) -> Self {
        self.data.hWnd = handle;
        self.flag(NIF_MESSAGE)
    }

    /// Sets the tooltip text for the notification icon.
    ///
    /// The tooltip text is displayed when the user hovers over the icon in the
    /// system tray. The text is converted to UTF-16 format and truncated if
    /// it exceeds the maximum length. Automatically sets the [NIF_TIP] and
    /// [NIF_SHOWTIP] flags.
    ///
    /// # Arguments
    ///
    /// * `s` - The tooltip text as any type that can be converted into a String
    ///
    /// # Returns
    ///
    /// Self for method chaining
    pub fn tip(mut self, s: impl Into<String>) -> Self {
        let s = s.into();
        let tip_utf16 = s.encode_utf16().chain(Some(0)).collect::<Vec<u16>>();
        let max_len = self.data.szTip.len() - 1;
        if tip_utf16.len() <= max_len + 1 {
            self.data.szTip[..tip_utf16.len()].copy_from_slice(&tip_utf16);
        } else {
            self.data.szTip[..max_len].copy_from_slice(&tip_utf16[..max_len]);
            self.data.szTip[max_len] = 0;
        }
        self.flag(NIF_TIP | NIF_SHOWTIP)
    }

    /// Sets the icon for the notification area.
    ///
    /// This method assigns an icon handle to the notification icon and
    /// automatically sets the [NIF_ICON] flag to indicate that the icon field
    /// is valid.
    ///
    /// # Arguments
    ///
    /// * `icon` - An [HICON] handle to the icon to be displayed in the system
    ///   tray
    ///
    /// # Returns
    ///
    /// Self for method chaining
    pub fn icon(mut self, icon: HICON) -> Self {
        self.data.hIcon = icon;
        self.flag(NIF_ICON)
    }

    /// Sets the icon for balloon notifications.
    ///
    /// This icon is displayed in balloon tip notifications. The method
    /// automatically sets the [NIF_ICON] flag to indicate that the balloon
    /// icon field is valid.
    ///
    /// # Arguments
    ///
    /// * `icon` - An [HICON] handle to the icon to be displayed in balloon
    ///   notifications
    ///
    /// # Returns
    ///
    /// Self for method chaining
    pub fn balloon_icon(mut self, icon: HICON) -> Self {
        self.data.hBalloonIcon = icon;
        self.flag(NIF_ICON)
    }

    /// Sets the callback message identifier for the notification icon.
    ///
    /// When the user interacts with the notification icon (clicks,
    /// double-clicks, etc.), Windows sends this message to the window
    /// procedure. Automatically sets the [NIF_MESSAGE] flag.
    ///
    /// # Arguments
    ///
    /// * `callback_msg` - The message identifier that will be sent to the
    ///   window procedure
    ///
    /// # Returns
    ///
    /// Self for method chaining
    pub fn callback_message(mut self, callback_msg: u32) -> Self {
        self.data.uCallbackMessage = callback_msg;
        self.flag(NIF_MESSAGE)
    }

    /// Sets a GUID for the notification icon.
    ///
    /// The GUID provides a unique identifier for the notification icon, which
    /// can be useful for maintaining icon state across application
    /// restarts. Automatically sets the [NIF_GUID] flag.
    ///
    /// # Arguments
    ///
    /// * `guid` - A 128-bit unsigned integer representing the GUID
    ///
    /// # Returns
    ///
    /// Self for method chaining
    pub fn guid(mut self, guid: impl Into<GUID>) -> Self {
        self.data.guidItem = guid.into();
        self.flag(NIF_GUID)
    }

    /// Sets the timeout duration for balloon tip notifications.
    ///
    /// This value specifies how long the balloon tip should be displayed before
    /// automatically disappearing. The timeout is specified in milliseconds.
    ///
    /// **Note**: This field is deprecated as of Windows Vista. On Vista and
    /// later, notification display times are based on system accessibility
    /// settings. This field is only effective on Windows 2000 and Windows
    /// XP.
    ///
    /// The system enforces minimum (10 seconds) and maximum (30 seconds)
    /// timeout values.
    ///
    /// # Arguments
    ///
    /// * `timeout` - Timeout duration in milliseconds (only effective on
    ///   Windows 2000/XP)
    ///
    /// # Returns
    ///
    /// Self for method chaining
    pub fn timeout(mut self, timeout: u32) -> Self {
        self.data.Anonymous.uTimeout = timeout;
        self
    }

    /// Sets the version of the Shell notification icon interface to use.
    ///
    /// This method specifies which version of the notification icon interface
    /// should be used, which affects the behavior of certain notification
    /// features. The version determines whether to use Windows 95-style or
    /// newer behavior for icon interactions.
    ///
    /// **Note**: This field shares the same memory location as `uTimeout` in a
    /// union. This method should only be used when sending a
    /// [`NIM_SETVERSION`] message via [`NotifyIcon::notify_set_version`]. For
    /// balloon notifications, use [`NotifyIcon::timeout`] instead.
    ///
    /// Common version values:
    /// - `0` (`NOTIFYICON_VERSION`): Use Windows 95-style behavior (default)
    /// - `3` (`NOTIFYICON_VERSION_4`): Use Windows Vista and later behavior
    /// - `4`: Use Windows 7 and later behavior
    ///
    /// # Arguments
    ///
    /// * `version` - The Shell notification icon interface version to use
    ///
    /// # Returns
    ///
    /// Self for method chaining
    pub fn version(mut self, version: u32) -> Self {
        self.data.Anonymous.uVersion = version;
        self
    }

    /// Sends a notification message to the Windows shell.
    ///
    /// This is the core method that communicates with the Windows shell to
    /// perform operations on the notification icon. It calls the
    /// [Shell_NotifyIconW] function with the specified message and the
    /// current icon data.
    ///
    /// # Arguments
    ///
    /// * `message` - The type of operation to perform (add, delete, modify,
    ///   etc.)
    ///
    /// # Returns
    ///
    /// A [`windows::core::Result<()>`] indicating success or failure
    ///
    /// # Errors
    ///
    /// Returns an error if the Shell_NotifyIconW function fails
    pub fn notify(&self, message: NOTIFY_ICON_MESSAGE) -> windows::core::Result<()> {
        (unsafe { Shell_NotifyIconW(message, &self.data) } != FALSE)
            .then_some(())
            .ok_or_else(windows::core::Error::from_win32)
    }

    /// Adds the notification icon to the system tray.
    ///
    /// This method sends a [NIM_ADD] message to add the notification icon to
    /// the notification area. The icon will appear in the system tray.
    ///
    /// # Returns
    ///
    /// A [`windows::core::Result<()>`] indicating success or failure
    ///
    /// # Errors
    ///
    /// Returns an error if the add operation fails
    pub fn notify_add(&self) -> windows::core::Result<()> {
        self.notify(NIM_ADD)
    }

    /// Removes the notification icon from the system tray.
    ///
    /// This method sends a [NIM_DELETE] message to remove the notification icon
    /// from the notification area. The icon will disappear from the system
    /// tray.
    ///
    /// # Returns
    ///
    /// A [`windows::core::Result<()>`] indicating success or failure
    ///
    /// # Errors
    ///
    /// Returns an error if the delete operation fails
    pub fn notify_delete(&self) -> windows::core::Result<()> {
        self.notify(NIM_DELETE)
    }

    /// Modifies an existing notification icon in the system tray.
    ///
    /// This method sends a [NIM_MODIFY] message to update the properties of an
    /// existing notification icon. Only the fields that have their
    /// corresponding flags set will be updated.
    ///
    /// # Returns
    ///
    /// A [`windows::core::Result<()>`] indicating success or failure
    ///
    /// # Errors
    ///
    /// Returns an error if the modify operation fails
    pub fn notify_modify(&self) -> windows::core::Result<()> {
        self.notify(NIM_MODIFY)
    }

    /// Sets focus to the notification icon.
    ///
    /// This method sends a [NIM_SETFOCUS] message to give focus to the
    /// notification icon, which can be useful for accessibility purposes.
    ///
    /// # Returns
    ///
    /// A [`windows::core::Result<()>`] indicating success or failure
    ///
    /// # Errors
    ///
    /// Returns an error if the set focus operation fails
    pub fn notify_set_focus(&self) -> windows::core::Result<()> {
        self.notify(NIM_SETFOCUS)
    }

    /// Sets the version of the notification icon interface.
    ///
    /// This method sends a [NIM_SETVERSION] message to specify which version of
    /// the notification icon interface to use. This affects the behavior of
    /// certain notification features.
    ///
    /// # Returns
    ///
    /// A [`windows::core::Result<()>`] indicating success or failure
    ///
    /// # Errors
    ///
    /// Returns an error if the set version operation fails
    pub fn notify_set_version(&self) -> windows::core::Result<()> {
        self.notify(NIM_SETVERSION)
    }
}
