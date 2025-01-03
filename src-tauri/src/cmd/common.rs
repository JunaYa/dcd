use tracing::info;

#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// start timer
#[tauri::command]
pub fn start_timer() {
    info!("start timer");
    // chrono
}

// stop timer
#[tauri::command]
pub fn stop_timer() {
    info!("stop timer");
}
