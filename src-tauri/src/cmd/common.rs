use tauri::{AppHandle};

use crate::state;


#[tauri::command]
pub fn get_state(app: AppHandle) -> [String; 2] {
    state::get_state(&app)
}

// start timer
#[tauri::command]
pub fn start_timer(app: AppHandle) {
    state::start_timer(&app);
}

// pause timer
#[tauri::command]
pub fn pause_timer(app: AppHandle) {
    state::pause_timer(&app);
}

// stop timer
#[tauri::command]
pub fn stop_timer(app: AppHandle) {
    state::stop_timer(&app);
}
