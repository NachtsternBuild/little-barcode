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
            
            match content_type {
   				BarcodeContent::Url(url) => {
   					let content_box = GtkBox::builder()
    					.orientation(Orientation::Vertical)
    					.spacing(10)
	    				.build();
 					
 					let url_clone = url.clone();
					let content_button = create_special_button::create_button(
						"Im Brower öffnen",
						move |_| {
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
        			show_alert_dialog(
        				&container_ref,
        				"WLAN",
        				&info,
        				"OK"
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
