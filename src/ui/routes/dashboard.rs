use dioxus::prelude::*;

use crate::ui::shared::components::navbar::Navbar;

// notes
// create a protected routes

/// Blog page
#[component]
pub fn Dashboard() -> Element {
    rsx! {
        div {
            h1 { "Dashboard Page" }
            Navbar {}
        }
    }
}
