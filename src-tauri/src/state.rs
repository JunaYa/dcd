use std::sync::Mutex;

use tauri::{AppHandle, Manager, State};

use crate::AppState;

pub fn get_state(app: &AppHandle) -> [String; 2] {
    let app_state = app.state::<Mutex<AppState>>();
    let state = app_state.lock().unwrap();
    [format!("{}", state.time), format!("{}", state.status)]
}

// start timer
pub fn start_timer(app: &AppHandle) {
    let app_state = app.state::<Mutex<AppState>>();
    let mut state = app_state.lock().unwrap();
    state.time = 0;
    state.status = 1;
}

// pause timer
pub fn pause_timer(app: &AppHandle) {
    let app_state = app.state::<Mutex<AppState>>();
    let mut state = app_state.lock().unwrap();
    state.status = 2;
    state.time = 0;
}

// stop timer
pub fn stop_timer(app: &AppHandle) {
    let app_state = app.state::<Mutex<AppState>>();
    let mut state = app_state.lock().unwrap();
    state.status = 0;
}
