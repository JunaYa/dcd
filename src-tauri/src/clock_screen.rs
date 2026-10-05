use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};

use crate::i18n::text;

pub fn close(app: &AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("eye-clock") {
        window.destroy().map_err(|error| error.to_string())?;
    }
    Ok(())
}

pub fn toggle(app: &AppHandle, language: &str) -> Result<(), String> {
    if app.get_webview_window("eye-clock").is_some() {
        return close(app);
    }
    let mut builder = WebviewWindowBuilder::new(
        app,
        "eye-clock",
        WebviewUrl::App("main.html?mode=clock".into()),
    )
    .title(text(language, "全屏时钟锁屏"))
    .decorations(false)
    .resizable(false)
    .minimizable(false)
    .maximizable(false)
    .skip_taskbar(true)
    .background_color(tauri::webview::Color(17, 18, 20, 255))
    .visible(false);
    if let Some(monitor) = app
        .get_webview_window("main")
        .and_then(|window| window.current_monitor().ok().flatten())
    {
        let position = monitor.position().to_logical::<f64>(monitor.scale_factor());
        builder = builder.position(position.x, position.y);
    }
    let window = builder.build().map_err(|error| error.to_string())?;
    let result = (|| {
        window
            .set_fullscreen(true)
            .map_err(|error| error.to_string())?;
        window.show().map_err(|error| error.to_string())?;
        window.set_focus().map_err(|error| error.to_string())
    })();
    if result.is_err() {
        let _ = window.destroy();
    }
    result
}
