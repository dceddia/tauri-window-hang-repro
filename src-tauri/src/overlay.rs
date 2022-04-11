use std::sync::Arc;
use tao::platform::windows::WindowBuilderExtWindows;
use tauri::{AppHandle, Manager};

pub struct WindowsOverlayView {
    overlay: Arc<tao::window::Window>,
}

impl WindowsOverlayView {
    pub fn new(overlay: Arc<tao::window::Window>) -> Self {
        WindowsOverlayView { overlay }
    }
}

pub fn add_overlay(app_handle: &AppHandle) -> WindowsOverlayView {
    let window = app_handle
        .get_window("main")
        .expect("failed to get main window");

    let hwnd = window.hwnd().expect("failed to get HWND");
    let overlay = app_handle
        .create_tao_window(move || {
            let window_builder = tao::window::WindowBuilder::new()
                .with_always_on_top(false)
                .with_decorations(false)
                .with_resizable(false)
                .with_visible(true)
                .with_position(tao::dpi::LogicalPosition::<u32>::new(30, 30))
                .with_owner_window(hwnd)
                .with_inner_size(tao::dpi::LogicalSize::<u32>::new(200, 200));

            ("Overlay".to_string(), window_builder)
        })
        .expect("failed to create overlay window");

    // This was an attempt to get the overlay to ignore events.
    // Enabling this code has no effect on the panic though.

    // make_window_passthrough_events(
    //     overlay
    //         .upgrade()
    //         .expect("failed to get Arc<Window>")
    //         .as_ref(),
    // );

    // I've tried storing the Weak<Window> as well as storing an Arc<Window> and
    // it has no effect on the panic.
    WindowsOverlayView::new(overlay.upgrade().expect("upgrade failed"))
}

// /// Make it so that mouse events pass through the window and it's excluded from tab order
// fn make_window_passthrough_events(window: &tao::window::Window) {
//     use tao::platform::windows::WindowExtWindows;
//     use windows::Win32::{
//         Foundation::HWND,
//         UI::WindowsAndMessaging::{
//             GetWindowLongW, SetWindowLongW, GWL_EXSTYLE, WS_EX_LAYERED, WS_EX_NOACTIVATE,
//             WS_EX_TRANSPARENT,
//         },
//     };
//     let hwnd = HWND(window.hwnd() as _);
//     unsafe {
//         // Based on https://stackoverflow.com/a/50245502
//         let cur_style = GetWindowLongW(hwnd, GWL_EXSTYLE) as u32;
//         SetWindowLongW(
//             hwnd,
//             GWL_EXSTYLE,
//             (cur_style | WS_EX_TRANSPARENT | WS_EX_LAYERED | WS_EX_NOACTIVATE) as i32,
//         );
//     }
// }
