#![windows_subsystem = "windows"]
use dioxus::desktop::tao::window::Fullscreen;
use dioxus::desktop::{tao, use_window, Config, WindowBuilder};
use dioxus::prelude::*;
use std::sync::mpsc;
use tao::event::{Event, WindowEvent};
use tracing::info;

mod shared;
mod ui;

use crate::shared::constants::{PKG_VERSION, PROJECTNAME};
use crate::shared::utils::logging::init_tracing;
use crate::ui::shared::utils::setup_tray::{
    init_tray_channel, tray_init, tray_receiver, TrayAction,
};
use crate::ui::shared::utils::webview::setup_webview2_user_data;

#[derive(Debug, Clone, Routable, PartialEq)]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

const ICON: Asset = asset!("/icons/icon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    init_tracing();
    info!("Initializing tracing completed");

    setup_webview2_user_data();
    info!("WebView2 user data folder setup completed");

    // Initialize a channel so the tray can signal actions
    let (tx, rx) = mpsc::channel::<TrayAction>();
    init_tray_channel(tx, rx);
    let _tray_icon = tray_init();
    info!("System tray initialization completed");

    info!("Starting Dioxus main app");

    LaunchBuilder::new()
        .with_cfg(
            Config::new()
                .with_exits_when_last_window_closes(false)
                .with_window(
                    WindowBuilder::new()
                        .with_title("Nex Cafe Client")
                        .with_always_on_top(false) //  true
                        .with_decorations(false)
                        .with_content_protection(true)
                        .with_closable(false)
                        .with_resizable(true)
                        .with_maximizable(true)
                        .with_fullscreen(Some(Fullscreen::Borderless(None))),
                ),
        )
        .launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: ICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}

#[component]
pub fn Hero() -> Element {
    // tray event handler to show window and handle close-to-tray
    let window = use_window();
    let window_clone = window.clone();
    let _ = use_hook(move || {
        // Intercept events: drain tray actions and handle close-to-hide
        let tray_rx = tray_receiver();
        window.create_wry_event_handler(move |event, _| {
            // Drain any pending tray actions
            if let Some(mtx) = &tray_rx {
                if let Ok(rx) = mtx.lock() {
                    while let Ok(action) = rx.try_recv() {
                        match action {
                            TrayAction::Show => {
                                window_clone.set_visible(true);
                                window_clone.set_focus();
                            }
                        }
                    }
                }
            }

            if let Event::WindowEvent { event, .. } = event {
                if let WindowEvent::CloseRequested = event {
                    window_clone.set_visible(false);
                }
            }
        })
    });

    rsx! {
        div {
            p { class: "text-red-900", "{PROJECTNAME}" }
            p { "{PKG_VERSION}" }
        }
    }
}

/// Home page
#[component]
fn Home() -> Element {
    rsx! {
        Hero {}

    }
}

/// Blog page
#[component]
pub fn Blog(id: i32) -> Element {
    rsx! {
        div { id: "blog",

            // Content
            h1 { "This is blog #{id}!" }
            p {
                "In blog #{id}, we show how the Dioxus router works and how URL parameters can be passed as props to our route components."
            }

            // Navigation links
            Link { to: Route::Blog { id: id - 1 }, "Previous" }
            span { " <---> " }
            Link { to: Route::Blog { id: id + 1 }, "Next" }
        }
    }
}

/// Shared navbar component.
#[component]
fn Navbar() -> Element {
    rsx! {
        div { id: "navbar",
            Link { to: Route::Home {}, "Home" }
            Link { to: Route::Blog { id: 1 }, "Blog" }
        }

        Outlet::<Route> {}
    }
}
