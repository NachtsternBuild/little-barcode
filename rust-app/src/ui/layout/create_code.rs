use gtk4::prelude::*;
use gtk4::{
    Box as GtkBox, 
    Orientation, 
    DropDown,       // Ersatz für ComboBoxText (Modernes GTK4.10+)
    Button, 
    Stack,
    Align,
    StringObject    // Nötig für das Auslesen des DropDown-Werts
};
use atlbase::design::utils::create_entry::*;
use atlbase::design::dialogs::*;
use atlbase::design::create_special_button;
use crate::Page;
use crate::switch_to_page;
use crate::execute_generation;

pub fn create_generator_page(stack: &Stack) -> GtkBox {
    let container = GtkBox::new(Orientation::Vertical, 18);
    container.set_valign(Align::Center);
    container.set_hexpand(true);
    container.set_vexpand(true);
    container.set_margin_top(24);
    container.set_margin_bottom(24);
    container.set_margin_start(24);
    container.set_margin_end(24);

    // dropdown qr/barcode
    let type_options = ["QR-Code", "Barcode (EAN-13)"];
    let type_dropdown = DropDown::from_strings(&type_options);
    type_dropdown.set_selected(0);
    container.append(&type_dropdown);

    // dropdown content
    let content_options = ["Unbekannt", "URL", "ISBN", "EAN-13", "WiFi"];
    let content_dropdown = DropDown::from_strings(&content_options);
    content_dropdown.set_selected(0);
    container.append(&content_dropdown);

    // entry 
    let (entry_box, main_entry) = create_entry("Inhalt:", Some("Hier Text eingeben..."));
    container.append(&entry_box);

    let generate_btn = Button::with_label("Code Generieren");
    // two css classes to highlight the button
    generate_btn.add_css_class("suggested-action");
    generate_btn.add_css_class("destructive-action");
    generate_btn.add_css_class("pill");
    container.append(&generate_btn);
    
    // clone the event handler
    let container_clone = container.clone();        
    let stack_clone = stack.clone();

    generate_btn.connect_clicked(move |_| {
        let is_qr = type_dropdown.selected() == 0; 
        
        let content_type = content_dropdown
            .selected_item()
            .and_downcast::<StringObject>()
            .map(|obj| obj.string().to_string())
            .unwrap_or_else(|| "URL".to_string());

        let input_value = main_entry.text().to_string();

        if input_value.is_empty() { 
            return; 
        }
        
        let container_ref = container_clone.clone();
        let stack_ref = stack_clone.clone();
        
        if content_type == "WiFi" {
            let ssid = input_value.clone();
            
            show_entry_dialog(
                &stack_ref,
                "WiFi",
                &format!("Bitte Passwort für SSID '{}' eingeben:", ssid),
                "Generieren",
                "Abbrechen",
                "Passwort:",
                "********",
                move |password| {
                    let wifi_string = format!("WIFI:S:{};P:{};;", ssid, password);
                    execute_generation(&container_ref, wifi_string, is_qr);
                },
            );
        } 
        
        else {
            execute_generation(&container_ref, input_value, is_qr);
        }
    });
    
    //  button back to home_page
    let stack_back_clone = stack.clone();
    let back_button = create_special_button::create_button_icon_position(
        "pan-start-symbolic",
        "Zurück",
        Align::Center,
        move |_| {
            switch_to_page(&stack_back_clone, Page::Home);
        }
    );

    container.append(&back_button);

    container
}
