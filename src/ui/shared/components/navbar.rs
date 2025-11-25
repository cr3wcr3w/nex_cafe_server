use dioxus::prelude::*;

use crate::Route;

/// Shared navbar component.
#[component]
pub fn Navbar() -> Element {
    rsx! {
        div { id: "navbar",
            Link { to: Route::Home {}, "Home" }
            Link { to: Route::Dashboard {}, "Dashboard" }
        }

        Outlet::<Route> {}
    }
}
