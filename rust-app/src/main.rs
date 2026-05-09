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

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id("io.github.nachtsternbuild.little.barcode")
        .build();

    app.connect_activate(build_ui);
    app.run()
}
