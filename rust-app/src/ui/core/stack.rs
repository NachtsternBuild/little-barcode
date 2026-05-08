use gtk4::Stack;
use crate::create_home_page;
use crate::create_generator_page;

// define the pages for the stack
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Page {
    Home,
    CreateCode,
}

// implement all the pages and there names
impl Page {
    pub fn to_id(&self) -> &str {
        match self {
            Page::Home => "home",
            Page::CreateCode => "create_code",
        }
    }
}

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
