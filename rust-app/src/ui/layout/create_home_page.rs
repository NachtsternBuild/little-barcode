//! # Home Page Module (Dashboard)
//!
//! The Home module provides the application's entry page. It serves as a central
//! hub for the two main functions: scanning existing codes and 
//! navigating to create new codes.
//!
//! ## Core Functions
//! * **Code Analysis**: Allows you to select an image file, extracts the content 
//!   and recognizes the data type (URL, Wi-Fi, EAN, etc.).
//! * **Interactive Dialogs**: Depending on the recognized content, the UI offers specific actions 
//!   (e.g., a button to open a browser or connect to a Wi-Fi network).
//! * **Navigation**: Link to the generator module via the [`Page`] enum.
use adw::prelude::*;
use gtk4::{
	Box as GtkBox,
	Orientation, 
	Align,
	Stack, 
};
use atlbase::design::utils::create_special_button;
use atlbase::design::dialogs::*;
use atlbase::helper::commands::open_url::open_url;
use std::path::PathBuf;
use atlbase::design::file_chooser;
use crate::Page;
use crate::BarcodeHandler;
use crate::Detector;
use crate::switch_to_page;
use crate::BarcodeContent;
use crate::connect_to_wifi;

/// Creates the application's home page (dashboard).
/// 
/// This page contains the main buttons for user interaction and defines
/// the callback logic for the file selection dialog and subsequent content processing.
///
/// # Arguments
///
/// * `stack` - A reference to the global [`Stack`] for navigating between pages.
///
/// # Scanning Logic
///
/// When the user scans a code, the following sequence is triggered:
/// 1. **File Selection**: A native file dialog opens.
/// 2. **Parsing**: The `BarcodeHandler` attempts to convert graphical data into text.
/// 3. **Recognition**: The `Detector` classifies the text (e.g., as [`BarcodeContent::Wifi`]).
/// 4. **Action**: 
///    * For **URLs**: A dialog with an “Open in Browser” button appears.
///    * For **Wi-Fi**: A dialog with a “Connect” button appears, which calls `connect_to_wifi`.
///    * For **standard codes**: An information dialog displays the raw data.
///
/// # Return value
///
/// Returns a [`GtkBox`] widget containing the centered main buttons.
pub fn create_home_page(stack: &Stack) -> GtkBox {
	let container = GtkBox::new(Orientation::Vertical, 12); // spacing between the main groups
    container.set_valign(Align::Center);
    container.set_hexpand(true);
    container.set_vexpand(true);
    container.set_margin_top(24);
    container.set_margin_bottom(24);
    container.set_margin_start(24);
    container.set_margin_end(24);
	
	// clone stack for page handeling
	let stack_clone_code = stack.clone();
	
	// clone the container for scanning the code
	let container_ref = container.clone();
	
	let start_scanning = move |path: PathBuf| {
        let handler_path = path.to_string_lossy().into_owned();
        let handler = BarcodeHandler::new();
        let detector = Detector::new();

        if let Ok((text, _format)) = handler.read_any_code(&handler_path) {
            let content_type = detector.detect(&text);
            
            // check the content type
            match content_type {
   				BarcodeContent::Url(url) => {
   					let content_box = GtkBox::builder()
    					.orientation(Orientation::Vertical)
    					.spacing(10)
	    				.build();
 					
 					let url_clone = url.clone();
					let content_button = create_special_button::create_button(
						"Im Browser öffnen",
						move |_| {
							// atlbase function to open url in the browser
							open_url(&url_clone);
						}
					);
					content_box.append(&content_button);
   					
        			show_custom_content_dialog(
        				&container_ref, 
        				"URL", 
        				&url.clone(), 
        				"OK", 
        				&content_box
        			);
    			}
    			
    			BarcodeContent::Ean13(ean) => {
    				show_alert_dialog(
        				&container_ref,
        				"EAN-13",
        				&ean,
        				"OK"
        			);
    			}
    			
    			BarcodeContent::Isbn(isbn) => {
        			show_alert_dialog(
        				&container_ref,
        				"ISBN",
        				&isbn,
        				"OK"
        			);
    			}
    			
    			BarcodeContent::VCard(vcard_data) => {
    				show_alert_dialog(
    					&container_ref,
    					"Visitenkarte",
    					&vcard_data,
    					"OK"
    				);
    			}
    			
    			BarcodeContent::Wifi { ssid, pass } => {
        			let info = format!("SSID: {}\nPasswort: {}", ssid, pass);       			
        			let content_box = GtkBox::builder()
    					.orientation(Orientation::Vertical)
    					.spacing(10)
	    				.build();
 					
					let content_button = create_special_button::create_button_icon(
						"network-wireless-acquiring-symbolic",
						"Verbinden",
						move |btn| {
							if let Some(window) = btn.root().and_downcast_ref::<adw::ApplicationWindow>() {
								// function to connect to wifi
								connect_to_wifi(
									window,
									&ssid,
									&pass
								);
							}
						}
					);	
					content_button.add_css_class("destructive-action");		
					content_box.append(&content_button);
					   					
        			show_custom_content_dialog(
        				&container_ref, 
        				"WLAN", 
        				&info, 
        				"OK", 
        				&content_box
        			);
    			}
    			
    			BarcodeContent::Unknown(text) => {
    				show_info_button_dialog(
    					&container_ref,
    					&text,
    					"OK"
    				);
    			}
			}
        }
    };
	
	let scan_button = create_special_button::create_button_icon_position(
		"scanner-symbolic",
		"Scan QR-Code/Barcode",
		Align::Center,
		move |btn| {
			file_chooser::show_file_chooser(btn, start_scanning.clone());
		}
		
	);
	
	let create_code_button = create_special_button::create_button_icon_position(
		"qr-code-symbolic",
		"Erstelle QR-Code/Barcode",
		Align::Center,
		move |_| {
			switch_to_page(&stack_clone_code, Page::CreateCode);
		}
	);
		
	container.append(&scan_button);
	container.append(&create_code_button);
	
	container
}
