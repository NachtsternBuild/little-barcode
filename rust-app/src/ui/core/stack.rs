//! # Navigation and Page Management
//!
//! This module controls page navigation within the application using
//! a `gtk4::Stack`. It implements a “lazy loading” pattern, in which pages
//! are only initialized and added to the stack when they are actually
//! needed.
//!
//! ## Concept
//! Instead of loading all UI components at program startup, this module checks
//! whether a requested page already exists in the stack. If not, the
//! corresponding constructor function is called.
use gtk4::Stack;
use crate::create_home_page;
use crate::create_generator_page;

/// Represents the available main pages of the application.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Page {
    /// The application's home page (dashboard).
    Home,
    /// The page for generating barcodes and QR codes.
    CreateCode,
}

impl Page {
	/// Returns the unique identifier (ID) of the page.
    ///
    /// This ID is used internally by `gtk4::Stack` to address the widgets
    ///
    pub fn to_id(&self) -> &str {
        match self {
            Page::Home => "home",
            Page::CreateCode => "create_code",
        }
    }
}

/// Changes the visible page in the passed stack.
///
/// If the target page does not yet exist in the stack, it is automatically
/// created and added. This saves resources when the application starts.
///
/// # Arguments
///
/// * `stack` - The GtkStack that manages the pages.
/// * `target` - The target page of type [`Page`] to which the view should be switched.
///
/// # How it works
///
/// 1. Determines the ID of the target page via [`Page::to_id`].
/// 2. Checks using `stack.child_by_name` whether the widget has already been loaded.
/// 3. If `None`: Calls the specific creation function (e.g., `create_home_page`).
/// 4. Adds the new widget to the stack under the ID.
/// 5. Sets the target page as `visible_child`.
pub fn switch_to_page(stack: &Stack, target: Page) {
    let id = target.to_id();

    // Check if page loaded
    if stack.child_by_name(id).is_none() {
        println!("Load page: {}...", id);
        
        // create the pages
        let widget = match target {
            Page::Home => create_home_page(stack),
            Page::CreateCode => create_generator_page(stack),
        };
		
		// add the pages to the stack
        stack.add_named(&widget, Some(id));
    }

    // switch the page
    stack.set_visible_child_name(id);
}
