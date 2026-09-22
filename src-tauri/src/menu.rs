use tauri::{
    menu::{Menu, MenuItem, PredefinedMenuItem, Submenu},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
};

pub fn create_tray(app: &mut tauri::App) -> Result<(), tauri::Error> {
    let language = crate::i18n::system_language();
    let menu = localized_menu(app.handle(), &language)?;
    TrayIconBuilder::with_id("main-tray")
        .icon(app.default_window_icon().unwrap().clone())
        .icon_as_template(true)
        .tooltip(crate::i18n::text(&language, "appName"))
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(|app, event| {
            let (action, minutes) = match event.id.as_ref() {
                "eye-main" => ("main", None),
                "eye-break" => ("break", None),
                "eye-settings" => ("settings", None),
                "eye-share" => ("share", None),
                "eye-pause-30" => ("pause", Some(30)),
                "eye-pause-60" => ("pause", Some(60)),
                "eye-pause-tomorrow" => ("pause", Some(0)),
                "eye-resume" => ("resume", None),
                "eye-quit" => ("quit", None),
                _ => return,
            };
            if let Err(error) = crate::eye::eye_action(app.clone(), action.into(), minutes) {
                tracing::error!("Tray action failed: {error}");
            }
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                position,
                ..
            } = event
            {
                if let Err(error) = crate::eye::toggle_popup(tray.app_handle(), position) {
                    tracing::error!("Tray popup failed: {error}");
                }
            }
        })
        .build(app)?;
    Ok(())
}

fn localized_menu(app: &tauri::AppHandle, language: &str) -> Result<Menu<tauri::Wry>, tauri::Error> {
    use crate::i18n::text;
    let pause = Submenu::with_items(app, text(language, "暂停提醒"), true, &[
        &MenuItem::with_id(app, "eye-pause-30", text(language, "30分钟"), true, None::<&str>)?,
        &MenuItem::with_id(app, "eye-pause-60", text(language, "1小时"), true, None::<&str>)?,
        &MenuItem::with_id(app, "eye-pause-tomorrow", text(language, "直到明天"), true, None::<&str>)?,
        &PredefinedMenuItem::separator(app)?,
        &MenuItem::with_id(app, "eye-resume", text(language, "恢复提醒"), true, None::<&str>)?,
    ])?;
    Menu::with_items(app, &[
        &MenuItem::with_id(app, "eye-break", text(language, "休息一下"), true, None::<&str>)?,
        &pause,
        &PredefinedMenuItem::separator(app)?,
        &MenuItem::with_id(app, "eye-main", text(language, "打开主窗口"), true, None::<&str>)?,
        &MenuItem::with_id(app, "eye-settings", text(language, "设置"), true, None::<&str>)?,
        &MenuItem::with_id(app, "eye-share", text(language, "复制今日摘要"), true, None::<&str>)?,
        &PredefinedMenuItem::separator(app)?,
        &MenuItem::with_id(app, "eye-quit", text(language, "退出"), true, None::<&str>)?,
    ])
}

pub fn update_language(app: &tauri::AppHandle, language: &str) -> Result<(), tauri::Error> {
    if let Some(tray) = app.tray_by_id("main-tray") {
        tray.set_menu(Some(localized_menu(app, language)?))?;
        tray.set_tooltip(Some(crate::i18n::text(language, "appName")))?;
    }
    Ok(())
}
