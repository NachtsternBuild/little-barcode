//! # Barcode and QR Code Generation Module
//!
//! This module orchestrates the process of code generation, validation, and 
//! graphical display within a Libadwaita application.
//!
//! ## Workflow
//! 1. **Creation**: Generates an image file (QR or barcode) in the user’s download directory.
//! 2. **Verification**: Immediately re-reads the generated code to check its integrity.
//! 3. **Recognition**: Analyzes the content (URL, Wi-Fi, vCard, etc.) using a `Detector`.
//! 4. **Presentation**: Opens a corresponding UI dialog to display the result.
//!
//! ## Prerequisites
//! The module requires write access to the download directory and an active GTK/Libadwaita event loop.
use gtk4::Widget;
use adw::prelude::*;
use atlbase::design::dialogs::*;
use dirs_next::download_dir;
use std::path::PathBuf;
use crate::BarcodeHandler;
use crate::Detector;
use crate::BarcodeContent;

/// Handles the entire code generation and display process.
///
/// This function is the central entry point when a user wants to
/// generate a new code. It handles file management, type detection,
/// and selecting the correct dialog type.
///
/// # Arguments
///
/// * `parent` - The widget (usually an `adw::ApplicationWindow`) over which the dialogs should appear.
/// * `content` - The string content to be encoded in the code.
/// * `is_qr` - `true` for a QR code, `false` for a classic barcode (e.g., EAN13).
///
/// # Behavior
///
/// * **Storage location**: The file is saved in the system’s default download folder.
/// * **Dimensions**: QR codes are displayed at 300x300px by default, barcodes at 100x300px.
/// * **Type detection**: Based on the content (`BarcodeContent`), a specific dialog 
///   with corresponding metadata (e.g., SSID for Wi-Fi) is displayed.
///
/// # Panics
///
/// The function expects a download directory to be defined in the system. 
/// If this is not the case, execution aborts with an error message.
pub fn execute_generation(
	parent: &impl IsA<Widget>, 
	content: String, 
	is_qr: bool
) {
	let handler = BarcodeHandler::new();
    let detector = Detector::new();
    
	if let Ok(file_name) = handler.create_code(&content, is_qr) {
	    println!("File saved as: {}", file_name);
	    
    	let mut downloads: PathBuf = download_dir().expect("Downloads-Verzeichnis nicht gefunden");
    	downloads.push(&file_name);
    	let file_path: &str = downloads.to_str().expect("Pfad enthält ungültige UTF-8-Zeichen");
     	
     	let width = if is_qr {
     		300
     	} else {
     		100
     	};
     	
     	let heigth = 300;
     	     	
	    // read the code back
	    if let Ok((text, format)) = handler.read_any_code(&file_name) {
	        println!("Content: {}, Format: {}", text, format);
	        let content_type = detector.detect(&text);
	        match content_type {
   				BarcodeContent::Url(url) => {	   					
   					show_image_dialog(
				    	parent,
						"URL",
						&url,
						"OK",
						&file_path,
						width,
						heigth
					);
    			}
    			
    			BarcodeContent::Ean13(ean) => {
    				show_image_dialog(
				    	parent,
						"EAN13",
						&ean,
						"OK",
						&file_path,
						width,
						heigth
					);
        			
    			}
    			
    			BarcodeContent::Isbn(isbn) => {
        			show_image_dialog(
        				parent,
        				"ISBN",
        				&isbn,
        				"OK",
        				&file_path,
        				width,
        				heigth
        			);
    			}
    			
    			BarcodeContent::VCard(vcard_data) => {
    				show_image_dialog(
        				parent,
        				"Visitenkarte",
        				&vcard_data,
        				"OK",
        				&file_path,
        				width,
        				heigth
        			);
    			}
    			
    			BarcodeContent::Wifi { ssid, pass } => {
        			let info = format!("SSID: {}\nPasswort: {}", ssid, pass);
        			show_image_dialog(
        				parent,
        				"WLAN",
        				&info,
        				"OK",
        				&file_path,
        				width,
        				heigth
        			);
    			}
    			
    			BarcodeContent::Unknown(_text) => {
    				show_info_button_dialog(
    					parent,
    					"Kein Inhaltstyp",
    					"OK"
    				);
    			}
    		 }       
	    }
	}
    println!("Generiere {} mit Inhalt: {}", if is_qr { "QR" } else { "Barcode" }, content);
}
