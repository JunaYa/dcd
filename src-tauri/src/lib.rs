use std::sync::Mutex;

use serde_json::json;
use tauri::{ActivationPolicy, Manager};
use tauri_plugin_store::StoreExt;
use tauri_nspanel;

mod cmd;
mod common;
mod constants;
mod global_shortcut;
mod menu;
mod platform;
mod window;
mod panel;
mod state;

#[derive(Default)]
struct AppState {
    status: u8, // 0: 休息, 1: 工作, 2: 暂停
    time: u32, // 当前时间
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_sql::Builder::new().build())
        .plugin(tauri_plugin_window_state::Builder::new().build())
        .plugin(tauri_nspanel::init())
        .plugin(tauri_plugin_positioner::init())
        .plugin(tauri_plugin_os::init())
        .setup(|app| {
            #[cfg(desktop)]
            configure_autostart(app);

            #[cfg(desktop)]
            let _ = global_shortcut::register_global_shortcut(app);

            app.set_activation_policy(ActivationPolicy::Accessory);

            menu::create_tray(app)?;

            app.manage(Mutex::new(AppState::default()));

            let app_local_data = app
                .path()
                .app_local_data_dir()
                .expect("could not resolve app local data path");
            let store = app.store("settings.json")?;
            store.set(
                "screenshot_path".to_string(),
                json!({ "value": app_local_data.to_string_lossy() }),
            );

            // check if first run
            let value = store
                .get("first_run")
                .unwrap_or_else(|| json!({ "value": false }));
            if value.is_null() {
                store.set("first_run".to_string(), json!({ "value": true }));
                window::show_startup_window(&app.handle());
            }

            Ok(())
        })
        // .menu(menu::get_app_menu)
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(global_shortcut::tauri_plugin_global_shortcut())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            cmd::show_preview_window, 
            cmd::hide_preview_window,
            cmd::update_preview_window,
            cmd::show_main_window,
            cmd::hide_main_window,
            cmd::show_setting_window,
            cmd::hide_setting_window,
            cmd::show_task_window,
            cmd::hide_task_window,
            cmd::close_task_window,
            cmd::start_timer,
            cmd::pause_timer,
            cmd::stop_timer,
            cmd::get_state,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(desktop)]
fn configure_autostart(app: &tauri::App) {
    use tauri_plugin_autostart::MacosLauncher;
    use tauri_plugin_autostart::ManagerExt;

    let _ = app.handle().plugin(tauri_plugin_autostart::init(
        MacosLauncher::LaunchAgent,
        Some(vec!["--flag1", "--flag2"]),
    ));

    // Get the autostart manager
    let autostart_manager = app.autolaunch();
    // Enable autostart
    let _ = autostart_manager.enable();
}
