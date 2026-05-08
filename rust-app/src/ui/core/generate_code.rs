use gtk4::Widget;
use adw::prelude::*;
use atlbase::design::dialogs::*;
use dirs_next::download_dir;
use std::path::PathBuf;
use crate::BarcodeHandler;
use crate::Detector;
use crate::BarcodeContent;

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
						600,
						400
					);
    			}
    			
    			BarcodeContent::Ean13(ean) => {
    				show_image_dialog(
				    	parent,
						"EAN13",
						&ean,
						"OK",
						&file_path,
						600,
						400
					);
        			
    			}
    			
    			BarcodeContent::Isbn(isbn) => {
        			show_image_dialog(
        				parent,
        				"ISBN",
        				&isbn,
        				"OK",
        				&file_path,
        				600,
        				400
        			);
    			}
    			
    			BarcodeContent::VCard(vcard_data) => {
    				show_image_dialog(
        				parent,
        				"Visitenkarte",
        				&vcard_data,
        				"OK",
        				&file_path,
        				600,
        				400
        			);
    			}
    			
    			BarcodeContent::Wifi { ssid, pass } => {
        			let info = format!("SSID: {}\nPasswort: {}", ssid, pass);
        			show_image_dialog(
        				parent,
        				"WLAN",
        				&info,
        				"OK",
        				&file_name,
        				600,
        				400
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
