use std::str::FromStr;
use std::{thread::sleep, time::Duration};

use strum_macros::{Display, EnumString};
use tauri::{
    menu::{Menu, MenuEvent, MenuItem, PredefinedMenuItem, Submenu},
    tray::{TrayIcon, TrayIconBuilder, TrayIconEvent},
    AppHandle,
};
use tracing::info;

use crate::window;

#[derive(Debug, Display, EnumString)]
#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
enum MenuID {
    RIGHT_NOW_TAKE_A_BREAK,
    START_TIMER,
    STOP_TIMER,
    SHOW_SETTING_WINDOW,
    HELP,
    FEEDBACK,
    EXIT,
}

pub fn create_tray(app: &mut tauri::App) -> Result<(), tauri::Error> {
    let _ = TrayIconBuilder::with_id("main-tray")
        .menu(&get_tray_menu(app.handle())?)
        .icon(app.default_window_icon().unwrap().clone())
        .icon_as_template(true)
        .menu_on_left_click(true)
        .on_menu_event(handle_tray_menu_events)
        .on_tray_icon_event(handle_tray_icon_events)
        .build(app)?;
    Ok(())
}

pub fn get_tray_menu(app: &AppHandle) -> Result<Menu<tauri::Wry>, tauri::Error> {
    let tray = Menu::with_id(app, "tray_menu")?;

    tray.append_items(&[
        &MenuItem::with_id(
            app,
            MenuID::RIGHT_NOW_TAKE_A_BREAK.to_string(),
            "Right Now Take A Break",
            true,
            None::<&str>,
        )?,
        &PredefinedMenuItem::separator(app)?,
        &MenuItem::with_id(
            app,
            MenuID::START_TIMER.to_string(),
            "Start Timer",
            true,
            None::<&str>,
        )?,
        &MenuItem::with_id(
            app,
            MenuID::STOP_TIMER.to_string(),
            "Stop Timer",
            true,
            None::<&str>,
        )?,
        &PredefinedMenuItem::separator(app)?,
        &MenuItem::with_id(
            app,
            MenuID::SHOW_SETTING_WINDOW.to_string(),
            "Setting Manager",
            true,
            None::<&str>,
        )?,
        &MenuItem::with_id(app, MenuID::EXIT.to_string(), "Exit", true, None::<&str>)?,
    ])?;

    Ok(tray)
}

pub fn get_app_menu(app: &AppHandle) -> Result<Menu<tauri::Wry>, tauri::Error> {
    let app_menu = Menu::new(app)?;
    let menu = Submenu::with_items(
        app,
        "Descktop Countdown",
        true,
        &[
            &PredefinedMenuItem::about(app, Some("about"), Default::default())?,
            &PredefinedMenuItem::separator(app)?,
            &MenuItem::with_id(
                app,
                MenuID::SHOW_SETTING_WINDOW.to_string(),
                "Settings",
                true,
                None::<&str>,
            )?,
            &PredefinedMenuItem::separator(app)?,
            &PredefinedMenuItem::hide(app, None)?,
            &PredefinedMenuItem::hide_others(app, None)?,
            &PredefinedMenuItem::show_all(app, None)?,
            &PredefinedMenuItem::quit(app, None)?,
        ],
    )?;
    app_menu.append(&menu)?;

    // edit menu
    let edit_menu = Submenu::with_items(
        app,
        "Edit",
        true,
        &[
            &PredefinedMenuItem::cut(app, None)?,
            &PredefinedMenuItem::copy(app, None)?,
            &PredefinedMenuItem::paste(app, None)?,
            &PredefinedMenuItem::separator(app)?,
            &PredefinedMenuItem::select_all(app, None)?,
            &PredefinedMenuItem::separator(app)?,
            &PredefinedMenuItem::undo(app, None)?,
            &PredefinedMenuItem::redo(app, None)?,
        ],
    )?;
    app_menu.append(&edit_menu)?;

    // help menu
    let help_menu = Submenu::with_items(
        app,
        "Help",
        true,
        &[
            &MenuItem::with_id(app, MenuID::HELP.to_string(), "Help", true, None::<&str>)?,
            &PredefinedMenuItem::separator(app)?,
            &MenuItem::with_id(
                app,
                MenuID::FEEDBACK.to_string(),
                "Feedback",
                true,
                None::<&str>,
            )?,
        ],
    )?;
    app_menu.append(&help_menu)?;

    Ok(app_menu)
}

fn handle_tray_icon_events(_tray: &TrayIcon, event: TrayIconEvent) {
    tauri_plugin_positioner::on_tray_event(_tray.app_handle(), &event);
    if let TrayIconEvent::DoubleClick { .. } = event {
        info!("Double click");
    }
}

fn handle_tray_menu_events(app: &AppHandle, event: MenuEvent) {
    let menu_id = if let Ok(menu_id) = MenuID::from_str(event.id.as_ref()) {
        menu_id
    } else {
        return;
    };

    match menu_id {
        MenuID::RIGHT_NOW_TAKE_A_BREAK => {
            info!("Right Now Take A Break");
            let app_handle = app.clone();
            tauri::async_runtime::spawn(async move {
                sleep(Duration::from_millis(2000));
                window::show_main_window(&app_handle);
            });
            
        }
        MenuID::START_TIMER => {
            info!("Start Timer");
            window::show_preview_window(&app);
        }
        MenuID::STOP_TIMER => {
            info!("Stop Timer");
        }
        MenuID::SHOW_SETTING_WINDOW => {
            info!("Setting Manager");
            window::show_setting_window(&app);
        }
        MenuID::EXIT => {
            info!("Exit");
            app.exit(0)
        }
        MenuID::HELP => {
            info!("Help");
        }
        MenuID::FEEDBACK => {
            info!("Feedback");
        }
    }
}
