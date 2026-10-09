use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};

use crate::i18n::text;

pub fn close(app: &AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("eye-clock") {
        window.hide().map_err(|error| error.to_string())?;
    }
    #[cfg(target_os = "macos")]
    app.state::<crate::platform::ClockKeepAwake>().release();
    crate::eye::clock_visibility(app, false)
}

pub fn toggle(app: &AppHandle, language: &str) -> Result<(), String> {
    let existing = app.get_webview_window("eye-clock");
    if let Some(window) = &existing {
        if window.is_visible().map_err(|error| error.to_string())? {
            return close(app);
        }
    }
    let new_window = existing.is_none();
    let window = if let Some(window) = existing {
        window
    } else {
        let window = WebviewWindowBuilder::new(
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
        .shadow(false)
        .always_on_top(true)
        .visible(false)
        .build()
        .map_err(|error| error.to_string())?;
        let handle = app.clone();
        window.on_window_event(move |event| {
            #[cfg(target_os = "macos")]
            if matches!(event, tauri::WindowEvent::Destroyed) {
                handle.state::<crate::platform::ClockKeepAwake>().release();
            }
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                api.prevent_close();
                if let Err(error) = close(&handle) {
                    tracing::error!("Unable to close clock: {error}");
                }
            }
        });
        window
    };
    let result = (|| {
        #[cfg(target_os = "macos")]
        app.state::<crate::platform::ClockKeepAwake>()
            .acquire()
            .map_err(|error| format!("{}: {error}", text(language, "无法保持时钟屏幕常亮")))?;
        crate::eye::clock_visibility(app, true)?;
        let monitor = app
            .get_webview_window("main")
            .and_then(|main| main.current_monitor().ok().flatten())
            .or(window
                .current_monitor()
                .map_err(|error| error.to_string())?)
            .ok_or_else(|| text(language, "无法确定休息浮层所在的显示器"))?;
        window
            .set_size(*monitor.size())
            .map_err(|error| error.to_string())?;
        window
            .set_position(*monitor.position())
            .map_err(|error| error.to_string())?;
        window
            .set_title(&text(language, "全屏时钟锁屏"))
            .map_err(|error| error.to_string())?;
        #[cfg(target_os = "macos")]
        {
            use tauri_nspanel::cocoa::appkit::{NSScreen, NSWindow, NSWindowCollectionBehavior};
            use tauri_nspanel::cocoa::base::{id, nil, NO, YES};
            let native = window.ns_window().map_err(|error| error.to_string())? as usize;
            let keep_alive = window.clone();
            window
                .run_on_main_thread(move || unsafe {
                    let _window = keep_alive;
                    let ns_window = native as id;
                    ns_window.setCollectionBehavior_(
                    NSWindowCollectionBehavior::NSWindowCollectionBehaviorCanJoinAllSpaces
                        | NSWindowCollectionBehavior::NSWindowCollectionBehaviorFullScreenAuxiliary
                        | NSWindowCollectionBehavior::NSWindowCollectionBehaviorStationary
                        | NSWindowCollectionBehavior::NSWindowCollectionBehaviorIgnoresCycle,
                );
                    // Cover the desktop and menu bar without entering a fullscreen Space.
                    ns_window.setLevel_(1000);
                    ns_window.setMovable_(NO);
                    ns_window.setHidesOnDeactivate_(NO);
                    let screen = ns_window.screen();
                    if screen != nil {
                        ns_window.setFrame_display_(NSScreen::frame(screen), YES);
                    }
                })
                .map_err(|error| error.to_string())?;
            crate::platform::show_overlay_panel(&window, new_window)?;
        }
        #[cfg(not(target_os = "macos"))]
        {
            window.show().map_err(|error| error.to_string())?;
            window.set_focus().map_err(|error| error.to_string())?;
        }
        Ok(())
    })();
    if result.is_err() {
        #[cfg(target_os = "macos")]
        app.state::<crate::platform::ClockKeepAwake>().release();
        if new_window {
            let _ = window.destroy();
        } else {
            let _ = window.hide();
        }
        if let Err(error) = crate::eye::clock_visibility(app, false) {
            tracing::error!("Unable to restore timer after clock failure: {error}");
        }
    }
    result
}
