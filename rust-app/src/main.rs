//! # Little Barcode - Main Application
//!
//! This module serves as the entry point for the GTK4/Libadwaita application “Little Barcode”.
//! It is responsible for initializing the system, setting up the user interface,
//! and managing the main application window.
//!
//! ## Structure
//! The application uses an `adw::ToolbarView` as the root container, consisting of:
//! * **HeaderBar**: Contains the title and system-specific controls (via `atlbase`).
//! * **Content Area**: A `gtk4::Stack` responsible for navigation between the various
//!   pages (Home, Generator).
//!
//! ## Logging & Initialization
//! Upon startup, a syslog handler is initialized under the domain `little_barcode`,
//! to centrally log error messages and system events.
use adw::prelude::*;
use adw::{
	Application, 
	ApplicationWindow, 
	HeaderBar, 
	ToolbarView
};
use gtk4::{
	Stack, 
	StackTransitionType,
	Label,
	glib 
};

// atl functions
use atlbase::ui_prelude::*;
use atlbase::prelude::init_syslog;

pub mod helper;
pub mod ui;
pub use helper::*;
pub use ui::*;

/// Initializes the user interface and configures the main window.
///
/// This function is called as soon as the `adw::Application` is activated. 
/// It builds the widget hierarchy and sets the initial page.
///
/// # Arguments
///
/// * `app` - The instance of the running Libadwaita application.
///
/// # Flow
/// 1. **Syslog**: Initializes the logging system for the `little_barcode` domain.
/// 2. **UI Layout**: Creates a `ToolbarView` with an integrated `HeaderBar`.
/// 3. **Navigation**: Creates a `gtk4::Stack` with horizontal transition (`SlideLeftRight`).
/// 4. **Initialization**: Loads the home page (`Page::Home`) via the navigation system.
/// 5. **Window**: Creates the `ApplicationWindow` with a default size of 500x500 pixels.
fn build_ui(app: &adw::Application) {
	// init syslog
	let little_barcode_domain = "little_barcode";
    init_syslog(little_barcode_domain).expect("Unable to initialize Syslog");
           
    // add a toolbar
    let toolbar_view = ToolbarView::new();
    toolbar_view.set_vexpand(true);
    toolbar_view.set_hexpand(true);
    let header_bar = HeaderBar::new();
	
	// set the title for the page
    header_bar.set_title_widget(Some(&Label::new(Some("Little Barcode"))));
    let custom_header_content = create_custom_headerbar(app, little_barcode_domain);
    
    // add the custom headerbar
    header_bar.pack_end(&custom_header_content);
    toolbar_view.add_top_bar(&header_bar);
    
    // create a stack
    let stack = Stack::builder()
        .transition_type(StackTransitionType::SlideLeftRight)
        .vexpand(true)
        .hexpand(true)
        .build();

	toolbar_view.set_content(Some(&stack));
	
    // initial load only the 'home' page
    switch_to_page(&stack, Page::Home);
    	
	// create the window
    let window = ApplicationWindow::builder()
        .application(app)
        .content(&toolbar_view)
        .default_width(500)
        .default_height(500)
        .build();
	
	// present the window
    window.present();
}

/// The main entry point of the binary file.
///
/// Initializes the `adw::Application` with the unique ID 
/// `io.github.nachtsternbuild.little.barcode` and starts the event loop.
fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id("io.github.nachtsternbuild.little.barcode")
        .build();

    app.connect_activate(build_ui);
    app.run()
}
