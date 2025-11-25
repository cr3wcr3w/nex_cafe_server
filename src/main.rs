#![windows_subsystem = "windows"]
use dioxus::desktop::tao::window::Fullscreen;
use dioxus::desktop::{tao, use_window, Config, WindowBuilder};
use dioxus::prelude::*;
use std::sync::mpsc::channel;
use tao::event::{Event, WindowEvent};
use tracing::info;

mod api;
mod shared;
mod ui;

use crate::shared::utils::logging::init_tracing;
use crate::ui::routes::dashboard::Dashboard;
use crate::ui::routes::home::Home;
use crate::ui::shared::utils::setup_tray::{TrayAction, TrayChannel};
use crate::ui::shared::utils::webview::setup_webview2_user_data;

#[derive(Debug, Clone, Routable, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/dashboard")]
    Dashboard {},
}

const ICON: Asset = asset!("/icons/icon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    init_tracing();
    info!("Initializing tracing completed");

    let os = std::env::consts::OS;

    if os == "windows" {
        setup_webview2_user_data();
        info!("WebView2 user data folder setup completed");
    }

    // Initialize a channel so the tray can signal actions
    let (tx, rx) = channel::<TrayAction>();
    TrayChannel::init(tx, rx);
    let _tray_icon = TrayChannel::init_tray();
    info!("System tray initialization completed");

    api::start_server();
    info!("API server started");

    info!("Starting Dioxus main app");

    LaunchBuilder::new()
        .with_cfg(
            Config::new()
                .with_exits_when_last_window_closes(false)
                .with_window(
                    WindowBuilder::new()
                        .with_title("Nex Cafe Server")
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
    // tray event handler to show window and handle close-to-tray
    let window = use_window();
    let window_clone = window.clone();
    let _ = use_hook(move || {
        // Intercept events: drain tray actions and handle close-to-hide
        let tray_rx = TrayChannel::receiver();
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
        document::Link { rel: "icon", href: ICON }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<Route> {}
    }
}
