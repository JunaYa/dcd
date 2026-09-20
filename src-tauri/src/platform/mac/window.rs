use std::{thread::sleep, time::Duration};

use crate::window;
use tauri::{PhysicalSize, WebviewWindow};
use tauri_plugin_positioner::{Position, WindowExt};

pub fn show_preview_window(window: &WebviewWindow) {
    let _ = window.show();
    let _ = window.unminimize();
    // window::bottom_right_position(window);
    let _ = window.set_focus();
    let _ = window.set_always_on_top(true);
    if let Some(monitor) = window::find_monitor(window) {
        let screen_size = monitor.size();
        let size = PhysicalSize {
            width: screen_size.width + 100,
            height: screen_size.height + 100,
        };
        let _ = window.set_size(tauri::Size::Physical(size));
        let _ = window.move_window(Position::TopLeft);
    }
}

pub fn update_preview_window(window: &WebviewWindow) {
    if !window.is_visible().unwrap_or_default() {
        show_preview_window(window);
    }
    let _ = window.unminimize();
    let _ = window.set_decorations(true);
    let _ = window.set_focus();
    let _ = window.set_resizable(true);
    let _ = window.set_always_on_top(false);
    if let Some(monitor) = window::find_monitor(window) {
        let screen_size = monitor.size();
        let size = PhysicalSize {
            width: screen_size.width / 2,
            height: screen_size.height / 2,
        };
        let _ = window.set_size(tauri::Size::Physical(size));
        // sleep 0.3
        let window = window.clone();
        tauri::async_runtime::spawn(async move {
            sleep(Duration::from_millis(100));
            let _ = window.move_window(Position::Center);
        });
    }
}

pub fn hide_preview_window(window: &WebviewWindow) {
    let _ = window.minimize();
}

pub fn show_main_window(window: &WebviewWindow) {
    let _ = window.show();
    let _ = window.set_focus();
    let _ = window.set_always_on_top(true);
}

pub fn hide_main_window(window: &WebviewWindow) {
    let _ = window.minimize();
    let _ = window.hide();
}

pub fn show_setting_window(window: &WebviewWindow) {
    let _ = window.show();
    window::center_position(window);
    let _ = window.set_focus();
    let _ = window.unminimize();
}

pub fn hide_setting_window(window: &WebviewWindow) {
    let _ = window.minimize();
}

pub fn show_startup_window(window: &WebviewWindow) {
    let _ = window.show();
    window::center_position(window);
    let _ = window.set_focus();
    let _ = window.unminimize();
}

pub fn hide_startup_window(window: &WebviewWindow) {
    let _ = window.minimize();
}

pub fn show_break_panel(window: &WebviewWindow, new_panel: bool) -> Result<(), String> {
    use tauri_nspanel::{ManagerExt, WebviewWindowExt};

    let handle = window.clone();
    let (sender, receiver) = std::sync::mpsc::sync_channel(1);
    window
        .run_on_main_thread(move || {
            let result = (|| {
                let panel = if new_panel {
                    handle.to_panel().map_err(|error| error.to_string())?
                } else {
                    handle
                        .get_webview_panel(handle.label())
                        .map_err(|error| format!("Unable to find break panel: {error:?}"))?
                };
                // Receive input without activating DCD and raising its main window.
                panel.set_style_mask(1 << 7); // NSWindowStyleMaskNonactivatingPanel
                panel.set_hides_on_deactivate(false);
                panel.set_has_shadow(false);
                panel.show();
                Ok(())
            })();
            let _ = sender.send(result);
        })
        .map_err(|error| error.to_string())?;
    receiver.recv().map_err(|error| error.to_string())?
}
