use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindow, WebviewWindowBuilder};

use crate::constants::MAIN_WINDOW;

pub fn get_main_window(app: &AppHandle) -> WebviewWindow {
    if let Some(window) = app.get_webview_window(MAIN_WINDOW) {
        window
    } else {
        let language = app
            .try_state::<crate::eye::EyeState>()
            .and_then(|state| {
                state
                    .0
                    .lock()
                    .ok()
                    .map(|monitor| monitor.settings.language.clone())
            })
            .unwrap_or_else(crate::i18n::system_language);
        let window =
            WebviewWindowBuilder::new(app, MAIN_WINDOW, WebviewUrl::App("main.html".into()))
                .title(crate::i18n::text(&language, "appName"))
                .decorations(true)
                .transparent(false)
                .shadow(true)
                .inner_size(1100.0, 720.0)
                .min_inner_size(760.0, 540.0)
                .center()
                .resizable(true)
                .build()
                .expect("Unable to build main window");
        let hide = window.clone();
        window.on_window_event(move |event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                api.prevent_close();
                let _ = hide.hide();
            }
        });

        window
    }
}
