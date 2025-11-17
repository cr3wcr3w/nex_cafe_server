use std::sync::{
    mpsc::{Receiver, Sender},
    Mutex, OnceLock,
};
use tray_item::{IconSource, TrayItem};

// Simple action enum to communicate tray intents to the UI
#[derive(Debug, Clone)]
pub enum TrayAction {
    Show,
}

// One-time initialized channel endpoints to bridge tray events to the app
static TRAY_SENDER: OnceLock<Sender<TrayAction>> = OnceLock::new();
static TRAY_RECEIVER: OnceLock<Mutex<Receiver<TrayAction>>> = OnceLock::new();

pub struct TrayChannel;

impl TrayChannel {
    pub fn init(sender: Sender<TrayAction>, receiver: Receiver<TrayAction>) {
        let _ = TRAY_SENDER.set(sender);
        let _ = TRAY_RECEIVER.set(Mutex::new(receiver));
    }

    pub fn receiver() -> Option<&'static Mutex<Receiver<TrayAction>>> {
        TRAY_RECEIVER.get()
    }

    pub fn init_tray() -> TrayItem {
        // Create tray icon using a Windows resource name embedded via tray.rc
        let mut tray = TrayItem::new("Nex Cafe", IconSource::Resource("app-icon"))
            .expect("Failed to create tray icon");

        // Show menu
        let show_tx = TRAY_SENDER.get().expect("TRAY_SENDER not initialized");
        let show_tx = show_tx.clone();
        tray.add_menu_item("Show", move || {
            let _ = show_tx.send(TrayAction::Show);
        })
        .expect("Failed to add Show item");

        tray
    }
}
