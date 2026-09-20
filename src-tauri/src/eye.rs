use crate::eye_model::{Effect, Monitor, Settings};
use chrono::{Local, Timelike};
use serde_json::json;
use std::{sync::Mutex, time::Duration};
use tauri::{AppHandle, Emitter, Manager, WebviewUrl, WebviewWindowBuilder};
use tauri_plugin_autostart::ManagerExt;
use tauri_plugin_store::StoreExt;

pub struct EyeState(pub Mutex<Monitor>);

#[cfg(target_os = "macos")]
fn is_idle() -> bool {
    #[link(name = "ApplicationServices", kind = "framework")]
    extern "C" {
        fn CGEventSourceSecondsSinceLastEventType(state: i32, event: u32) -> f64;
    }
    unsafe { CGEventSourceSecondsSinceLastEventType(1, u32::MAX) >= 60.0 }
}
#[cfg(not(target_os = "macos"))]
fn is_idle() -> bool {
    false
}

fn save(app: &AppHandle, monitor: &Monitor) -> Result<(), String> {
    let store = app.store("eye-monitor.json").map_err(|e| e.to_string())?;
    store.set(
        "monitor",
        serde_json::to_value(monitor).map_err(|e| e.to_string())?,
    );
    store.save().map_err(|e| e.to_string())
}

pub fn initialize(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let store = app.store("eye-monitor.json")?;
    let mut monitor = match store.get("monitor") {
        Some(value) => serde_json::from_value::<Monitor>(value)?,
        None => Monitor::default(),
    };
    monitor.settings.validate()?;
    monitor.settings.autostart = app.autolaunch().is_enabled()?;
    // A process restart ends a previous break; sleeping time never becomes work time.
    monitor.break_until = 0;
    monitor.work_seconds = 0.0;
    monitor.next_reminder = 0;
    let now = Local::now();
    monitor.tick(
        now.timestamp(),
        &now.format("%Y-%m-%d").to_string(),
        now.hour() * 60 + now.minute(),
        true,
    );
    let settings = monitor.settings.clone();
    app.manage(EyeState(Mutex::new(monitor)));
    apply_shell(app, &settings)?;
    let handle = app.clone();
    std::thread::spawn(move || loop {
        std::thread::sleep(Duration::from_secs(1));
        let now = Local::now();
        let state = handle.state::<EyeState>();
        let mut m = match state.0.lock() {
            Ok(m) => m,
            Err(_) => break,
        };
        let effect = m.tick(
            now.timestamp(),
            &now.format("%Y-%m-%d").to_string(),
            now.hour() * 60 + now.minute(),
            is_idle(),
        );
        if now.second() % 30 == 0 || effect == Effect::EndBreak || effect == Effect::StartBreak {
            m.storage_error = save(&handle, &m).err();
        }
        let fatigue = m.fatigue();
        let seconds = m.days.get(&m.date).map(|d| d.seconds).unwrap_or(0);
        let settings = m.settings.clone();
        drop(m);
        if let Some(tray) = handle.tray_by_id("main-tray") {
            let title = if settings.tray_time {
                format!(
                    "{}% · {}小时{:02}分",
                    fatigue,
                    seconds / 3600,
                    seconds % 3600 / 60
                )
            } else {
                String::new()
            };
            let _ = tray.set_title(Some(title));
        }
        match effect {
            Effect::StartBreak => {
                if let Err(error) = show_break(&handle, &settings) {
                    tracing::error!("Unable to show break: {error}");
                    if let Ok(mut m) = state.0.lock() {
                        m.break_until = 0;
                        m.next_reminder = now.timestamp() + 60;
                        m.storage_error = Some(format!("无法显示休息窗口：{error}"));
                    }
                }
            }
            Effect::EndBreak => {
                if let Some(window) = handle.get_webview_window("eye-break") {
                    let _ = window.destroy();
                }
                #[cfg(target_os = "macos")]
                if settings.sound {
                    let _ = std::process::Command::new("afplay")
                        .arg("/System/Library/Sounds/Glass.aiff")
                        .spawn();
                }
            }
            Effect::Warn => {
                let _ = handle.emit("eye-warning", "10 秒后开始休息");
                #[cfg(target_os = "macos")]
                let _ = std::process::Command::new("osascript").args(["-e", "display notification \"10 秒后开始休息，放松一下双眼\" with title \"Eye Monitor\""]).spawn();
            }
            Effect::None => {}
        }
    });
    Ok(())
}

fn apply_shell(app: &AppHandle, settings: &Settings) -> Result<(), String> {
    if let Some(popup) = app.get_webview_window("eye-tray") {
        popup.set_size(tauri::LogicalSize::new(520.0, if settings.tray_show_chart { 530.0 } else { 285.0 }))
            .map_err(|e| e.to_string())?;
    }
    if let Some(tray) = app.tray_by_id("main-tray") {
        tray.set_visible(settings.tray_icon)
            .map_err(|e| e.to_string())?;
    }
    #[cfg(target_os = "macos")]
    app.set_dock_visibility(settings.dock_icon)
        .map_err(|e| e.to_string())?;
    Ok(())
}

pub fn show_main(app: &AppHandle, page: &str) -> Result<(), String> {
    let window = crate::window::get_main_window(app);
    window.show().map_err(|e| e.to_string())?;
    window.unminimize().map_err(|e| e.to_string())?;
    window.set_focus().map_err(|e| e.to_string())?;
    window
        .emit("eye-navigate", page)
        .map_err(|e| e.to_string())?;
    if let Some(popup) = app.get_webview_window("eye-tray") {
        let _ = popup.hide();
    }
    Ok(())
}

pub fn toggle_popup(app: &AppHandle, position: tauri::PhysicalPosition<f64>) -> Result<(), String> {
    let show_chart = app.state::<EyeState>().0.lock().map_err(|e| e.to_string())?.settings.tray_show_chart;
    let popup = if let Some(w) = app.get_webview_window("eye-tray") {
        if w.is_visible().unwrap_or(false) {
            return w.hide().map_err(|e| e.to_string());
        }
        w
    } else {
        let w = WebviewWindowBuilder::new(
            app,
            "eye-tray",
            WebviewUrl::App("main.html?mode=tray".into()),
        )
        .title("Eye Monitor")
        .decorations(false)
        .transparent(true)
        .shadow(false)
        .inner_size(520.0, if show_chart { 530.0 } else { 285.0 })
        .resizable(false)
        .skip_taskbar(true)
        .always_on_top(true)
        .visible(false)
        .build()
        .map_err(|e| e.to_string())?;
        let hide = w.clone();
        w.on_window_event(move |event| {
            if matches!(event, tauri::WindowEvent::Focused(false)) {
                let _ = hide.hide();
            }
        });
        w
    };
    let scale = popup.scale_factor().map_err(|e| e.to_string())?;
    let mut x = position.x - 260.0 * scale;
    if let Some(monitor) = popup.current_monitor().map_err(|e| e.to_string())? {
        let left = monitor.position().x as f64;
        let right = left + monitor.size().width as f64 - 520.0 * scale;
        x = x.clamp(left, right.max(left));
    }
    popup
        .set_position(tauri::PhysicalPosition::new(x, position.y + 14.0 * scale))
        .map_err(|e| e.to_string())?;
    popup.show().map_err(|e| e.to_string())?;
    popup.set_focus().map_err(|e| e.to_string())
}

fn show_break(app: &AppHandle, settings: &Settings) -> Result<(), String> {
    let window = if let Some(w) = app.get_webview_window("eye-break") {
        w
    } else {
        let w = WebviewWindowBuilder::new(
            app,
            "eye-break",
            WebviewUrl::App("main.html?mode=break".into()),
        )
        .title("休息一下")
        .decorations(false)
        .transparent(true)
        .shadow(false)
        .inner_size(800.0, 560.0)
        .resizable(false)
        .maximizable(false)
        .minimizable(false)
        .center()
        .skip_taskbar(true)
        .always_on_top(true)
        .visible(false)
        .build()
        .map_err(|e| e.to_string())?;
        w.on_window_event(|event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                api.prevent_close();
            }
        });
        w
    };
    if settings.reminder_style == "fullscreen" {
        let monitor = app
            .get_webview_window("main")
            .and_then(|main| main.current_monitor().ok().flatten())
            .or(window.current_monitor().map_err(|e| e.to_string())?)
            .ok_or("无法确定休息浮层所在的显示器")?;
        window.set_size(*monitor.size()).map_err(|e| e.to_string())?;
        window.set_position(*monitor.position()).map_err(|e| e.to_string())?;
        #[cfg(target_os = "macos")]
        {
            use tauri_nspanel::cocoa::appkit::{NSScreen, NSWindow, NSWindowCollectionBehavior};
            use tauri_nspanel::cocoa::base::{id, nil, NO, YES};
            let native = window.ns_window().map_err(|e| e.to_string())? as usize;
            let keep_alive = window.clone();
            window.run_on_main_thread(move || unsafe {
                let _window = keep_alive;
                let ns_window = native as id;
                ns_window.setCollectionBehavior_(
                    NSWindowCollectionBehavior::NSWindowCollectionBehaviorCanJoinAllSpaces
                        | NSWindowCollectionBehavior::NSWindowCollectionBehaviorFullScreenAuxiliary
                        | NSWindowCollectionBehavior::NSWindowCollectionBehaviorStationary
                        | NSWindowCollectionBehavior::NSWindowCollectionBehaviorIgnoresCycle,
                );
                // Screen-saver level covers the menu bar without creating a fullscreen Space.
                ns_window.setLevel_(1000);
                ns_window.setMovable_(NO);
                ns_window.setHidesOnDeactivate_(NO);
                let screen = ns_window.screen();
                if screen != nil {
                    ns_window.setFrame_display_(NSScreen::frame(screen), YES);
                }
            }).map_err(|e| e.to_string())?;
        }
    }
    window.show().map_err(|e| e.to_string())?;
    window.set_focus().map_err(|e| e.to_string())?;
    if let Some(popup) = app.get_webview_window("eye-tray") {
        let _ = popup.hide();
    }
    Ok(())
}

#[tauri::command]
pub fn eye_snapshot(app: AppHandle) -> Result<serde_json::Value, String> {
    let state = app.state::<EyeState>();
    let m = state.0.lock().map_err(|e| e.to_string())?;
    let mut value = serde_json::to_value(&*m).map_err(|e| e.to_string())?;
    value["fatigue"] = json!(m.fatigue());
    value["now"] = json!(Local::now().timestamp());
    value["storageError"] = json!(m.storage_error);
    Ok(value)
}

pub fn save_on_exit(app: &AppHandle) {
    if let Some(state) = app.try_state::<EyeState>() {
        if let Ok(m) = state.0.lock() {
            if let Err(error) = save(app, &m) {
                tracing::error!("Unable to save usage on exit: {error}");
            }
        }
    }
}

fn apply_settings(app: &AppHandle, settings: &Settings) -> Result<(), String> {
    let enabled = app.autolaunch().is_enabled().map_err(|e| e.to_string())?;
    if enabled != settings.autostart {
        if settings.autostart {
            app.autolaunch().enable()
        } else {
            app.autolaunch().disable()
        }
        .map_err(|e| e.to_string())?;
    }
    apply_shell(app, settings)
}

#[tauri::command]
pub fn eye_save_settings(app: AppHandle, settings: Settings) -> Result<(), String> {
    settings.validate()?;
    let state = app.state::<EyeState>();
    let mut m = state.0.lock().map_err(|e| e.to_string())?;
    let old = m.settings.clone();
    let result = apply_settings(&app, &settings).and_then(|_| {
        m.settings = settings;
        save(&app, &m)
    });
    if let Err(error) = result {
        m.settings = old.clone();
        let rollback = apply_settings(&app, &old);
        // Restore the store's in-memory value too, so its later autosave cannot persist a failed edit.
        let persisted = save(&app, &m);
        return Err(format!(
            "{error}{}{}",
            rollback
                .err()
                .map(|e| format!("；系统设置恢复失败：{e}"))
                .unwrap_or_default(),
            persisted
                .err()
                .map(|e| format!("；本地保存失败：{e}"))
                .unwrap_or_default()
        ));
    }
    Ok(())
}

#[tauri::command]
pub fn eye_action(app: AppHandle, action: String, minutes: Option<u32>) -> Result<(), String> {
    if action == "main" || action == "settings" {
        return show_main(
            &app,
            if action == "settings" {
                "settings"
            } else {
                "today"
            },
        );
    }
    let state = app.state::<EyeState>();
    let mut m = state.0.lock().map_err(|e| e.to_string())?;
    let now = Local::now().timestamp();
    match action.as_str() {
        "break" => {
            let old = m.break_until;
            m.start_break(now);
            if let Err(error) = show_break(&app, &m.settings) {
                m.break_until = old;
                return Err(error);
            }
        }
        "skip" => {
            if !m.settings.allow_skip {
                return Err("当前规则不允许跳过休息".into());
            }
            m.break_until = 0;
            m.next_reminder = now + m.settings.repeat_minutes as i64 * 60;
            if let Some(w) = app.get_webview_window("eye-break") {
                w.destroy().map_err(|e| e.to_string())?;
            }
        }
        "pause" => {
            let mins = minutes.unwrap_or(30);
            if ![30, 60, 0].contains(&mins) {
                return Err("暂停时长无效".into());
            }
            m.paused_until = if mins == 0 {
                let local = Local::now();
                now + (24 * 3600 - local.num_seconds_from_midnight()) as i64
            } else {
                now + mins as i64 * 60
            };
        }
        "resume" => {
            m.paused_until = 0;
        }
        "quit" => {
            save(&app, &m)?;
            drop(m);
            app.exit(0);
            return Ok(());
        }
        _ => return Err("未知操作".into()),
    }
    save(&app, &m)
}
