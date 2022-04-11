#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use std::sync::{Arc, Mutex};

use overlay::{add_overlay, WindowsOverlayView};
use tauri::Manager;

mod overlay;

struct OverlayState(Arc<Mutex<Option<WindowsOverlayView>>>);
impl OverlayState {
    pub fn new() -> Self {
        OverlayState(Arc::new(Mutex::new(None)))
    }
    pub fn set_overlay(&self, overlay: WindowsOverlayView) {
        *self.0.lock().unwrap() = Some(overlay);
    }
}

fn main() {
    let app = tauri::Builder::default()
        .manage(OverlayState::new())
        .build(tauri::generate_context!())
        .expect("failed to build app");

    app.run(move |handle, event| match event {
        tauri::RunEvent::Ready => {
            let overlay = add_overlay(handle);

            // Storing the overlay in state so that it doesn't immediately get
            // dropped, but, the bug happens whether or not the overlay struct
            // is kept around.
            let state: tauri::State<OverlayState> = handle.state();
            state.set_overlay(overlay);
        }
        _ => {}
    })
}
