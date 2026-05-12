//! # Wi-Fi Connection Module
//!
//! This module provides functions for managing network connections under Linux
//! using the NetworkManager CLI (`nmcli`).
//!
//! ## How It Works
//! Integration is handled via shell commands that are executed with administrator privileges
//! using `pkexec`. A graphical spinner informs the user about the progress
//! of the connection.
//!
//! ## Dependencies
//! The module requires the following components to be installed on the target system:
//! * **NetworkManager**: For the actual control of the hardware.
//! * **Polkit (pkexec)**: For securely requesting privileges in the GUI.
//! * **atlbase**: For the UI components (spinner/dialogs).

use atlbase::ui_prelude::command_pkexec_spinner;

/// Connects the system to a Wi-Fi network using NetworkManager.
///
/// This function uses `nmcli` to establish a connection. If root privileges
/// are required, a graphical authentication dialog is displayed via `pkexec`.
/// A spinner dialog is displayed during the process.
///
/// # Arguments
///
/// * `parent` - Reference to the `adw::ApplicationWindow` to which the spinner dialog is bound.
/// * `ssid` - The name of the Wi-Fi network.
/// * `password` - The password for the network.
///
/// # Prerequisites
///
/// * An active `NetworkManager` service.
/// * Installed binaries of `nmcli` and `pkexec`.
///
/// # Troubleshooting
///
/// If the error `802-11-wireless-security.key-mgmt` occurs, try 
/// deleting existing profiles for this SSID:
/// ```bash
/// nmcli connection delete “Your_SSID”
/// ```
///
/// # Security Notes
///
/// The password is passed to the shell process in plain text. In security-critical 
/// environments, the direct D-Bus bindings (`libnm`) should be used instead.
pub fn connect_to_wifi(
	parent: &adw::ApplicationWindow,
	ssid: &str, 
	password: &str
) {
    println!("Versuche Verbindung zu '{}' herzustellen...", ssid);
    
	let cmd = format!("nmcli device wifi connect '{}' password '{}'", ssid, password);
    
    command_pkexec_spinner(
        parent,
        &cmd,
        "WLAN Verbinden",
        "WLAN wird Verbunden..."
    );
}
