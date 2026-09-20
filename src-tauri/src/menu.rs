use tauri::{
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
};

pub fn create_tray(app: &mut tauri::App) -> Result<(), tauri::Error> {
    let menu = Menu::with_items(
        app,
        &[
            &MenuItem::with_id(app, "eye-main", "打开主窗口", true, None::<&str>)?,
            &MenuItem::with_id(app, "eye-break", "手动开始一次休息", true, None::<&str>)?,
            &MenuItem::with_id(app, "eye-quit", "退出 Eye Monitor", true, None::<&str>)?,
        ],
    )?;
    TrayIconBuilder::with_id("main-tray")
        .icon(app.default_window_icon().unwrap().clone())
        .icon_as_template(true)
        .tooltip("Eye Monitor · 关照双眼")
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(|app, event| {
            let action = match event.id.as_ref() {
                "eye-main" => "main",
                "eye-break" => "break",
                "eye-quit" => "quit",
                _ => return,
            };
            if let Err(error) = crate::eye::eye_action(app.clone(), action.into(), None) {
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
