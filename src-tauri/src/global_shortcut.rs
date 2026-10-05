use std::{collections::HashSet, str::FromStr, sync::Mutex};
use tauri::{plugin::TauriPlugin, AppHandle, Emitter, Manager};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

use crate::{eye, eye_model::Settings, i18n::text};

#[derive(Default)]
pub struct RegisteredShortcuts(Mutex<Vec<Shortcut>>);

pub fn bindings(settings: &Settings) -> Result<Vec<(Shortcut, &'static str)>, String> {
    let mut bindings = Vec::new();
    for (value, action) in [
        (&settings.shortcut_main, "main"),
        (&settings.shortcut_break, "break"),
        (&settings.shortcut_skip, "skip"),
        (&settings.shortcut_pause, "toggle-pause"),
        (&settings.shortcut_clock, "toggle-clock"),
    ] {
        if value.trim().is_empty() {
            continue;
        }
        let shortcut = Shortcut::from_str(value.trim())
            .map_err(|_| text(&settings.language, "快捷键格式无效"))?;
        if !shortcut
            .mods
            .intersects(Modifiers::CONTROL | Modifiers::SUPER | Modifiers::ALT)
        {
            return Err(text(&settings.language, "快捷键格式无效"));
        }
        if bindings.iter().any(|(key, _)| key == &shortcut) {
            return Err(text(&settings.language, "快捷键不能重复"));
        }
        bindings.push((shortcut, action));
    }
    Ok(bindings)
}

fn reconcile(
    active: &mut Vec<Shortcut>,
    desired: &[Shortcut],
    mut register: impl FnMut(Shortcut) -> Result<(), String>,
    mut unregister: impl FnMut(Shortcut) -> Result<(), String>,
) -> Result<(), String> {
    // Register additions first so a conflict leaves existing shortcuts available.
    for shortcut in desired {
        if !active.contains(shortcut) {
            register(*shortcut)?;
            active.push(*shortcut);
        }
    }
    for shortcut in active.clone() {
        if !desired.contains(&shortcut) {
            unregister(shortcut)?;
            active.retain(|key| key != &shortcut);
        }
    }
    Ok(())
}

pub fn apply(app: &AppHandle, settings: &Settings) -> Result<(), String> {
    let desired: Vec<_> = bindings(settings)?
        .into_iter()
        .map(|(key, _)| key)
        .collect();
    let state = app.state::<RegisteredShortcuts>();
    let mut active = state.0.lock().map_err(|e| e.to_string())?;
    let manager = app.global_shortcut();
    let previous = active.clone();
    let mut sync = |desired: &[Shortcut]| {
        reconcile(
            &mut active,
            desired,
            |key| {
                manager.register(key).map_err(|e| {
                    format!(
                        "{} ({key}): {e}",
                        text(&settings.language, "快捷键注册失败")
                    )
                })
            },
            |key| {
                manager.unregister(key).map_err(|e| {
                    format!(
                        "{} ({key}): {e}",
                        text(&settings.language, "快捷键注销失败")
                    )
                })
            },
        )
    };
    if let Err(error) = sync(&desired) {
        return match sync(&previous) {
            Ok(()) => Err(error),
            Err(rollback) => Err(format!("{error}; {rollback}")),
        };
    }
    Ok(())
}

pub fn tauri_plugin_global_shortcut() -> TauriPlugin<tauri::Wry> {
    let pressed = Mutex::new(HashSet::new());
    tauri_plugin_global_shortcut::Builder::new()
        .with_handler(move |app, shortcut, event| {
            {
                let Ok(mut pressed) = pressed.lock() else {
                    return;
                };
                if event.state() == ShortcutState::Released {
                    pressed.remove(&shortcut.id());
                    return;
                }
                if !pressed.insert(shortcut.id()) {
                    return;
                }
            }
            let app = app.clone();
            let shortcut = *shortcut;
            // The plugin holds its registry lock during callbacks. Dispatch outside it.
            tauri::async_runtime::spawn_blocking(move || {
                let handle = app.clone();
                if let Err(error) = app.run_on_main_thread(move || dispatch(handle, shortcut)) {
                    tracing::error!("Unable to dispatch global shortcut: {error}");
                    let _ = app.emit("eye-warning", error.to_string());
                }
            });
        })
        .build()
}

fn dispatch(app: AppHandle, shortcut: Shortcut) {
    let action = {
        let Some(state) = app.try_state::<eye::EyeState>() else {
            return;
        };
        let Ok(monitor) = state.0.lock() else { return };
        bindings(&monitor.settings).ok().and_then(|bindings| {
            bindings
                .into_iter()
                .find(|(key, _)| key == &shortcut)
                .map(|(_, action)| action)
        })
    };
    if let Some(action) = action {
        if let Err(error) = eye::eye_action(app.clone(), action.into(), None) {
            tracing::error!("Global shortcut failed: {error}");
            let _ = app.emit("eye-warning", error);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_defaults_aliases_and_disabled_bindings() {
        let mut settings: Settings = serde_json::from_str("{}").unwrap();
        assert_eq!(bindings(&settings).unwrap().len(), 5);
        settings.shortcut_main.clear();
        assert_eq!(bindings(&settings).unwrap().len(), 4);
        let restored: Settings =
            serde_json::from_str(&serde_json::to_string(&settings).unwrap()).unwrap();
        assert!(restored.shortcut_main.is_empty());
        assert_eq!(restored.shortcut_pause, settings.shortcut_pause);
        assert_eq!(restored.shortcut_clock, "CmdOrCtrl+Shift+L");
        assert!(bindings(&restored).unwrap().iter().any(|(_, action)| *action == "toggle-clock"));
        settings.shortcut_main = settings.shortcut_break.clone();
        assert!(bindings(&settings).is_err());
        settings.shortcut_main = "Shift+A".into();
        assert!(bindings(&settings).is_err());
        settings.shortcut_main = "not a shortcut".into();
        assert!(bindings(&settings).is_err());
        settings.shortcut_main = "Ctrl+Alt+X".into();
        settings.shortcut_break = "Control+Alt+KeyX".into();
        assert!(bindings(&settings).is_err());
    }

    #[test]
    fn conflict_preserves_old_bindings_and_rollback_removes_partial_additions() {
        let old = Shortcut::from_str("Ctrl+Shift+A").unwrap();
        let added = Shortcut::from_str("Ctrl+Shift+B").unwrap();
        let conflict = Shortcut::from_str("Ctrl+Shift+C").unwrap();
        let mut active = vec![old];
        assert!(reconcile(
            &mut active,
            &[added, conflict],
            |key| {
                if key == conflict {
                    Err("occupied".into())
                } else {
                    Ok(())
                }
            },
            |_| panic!("must retain existing shortcut on conflict")
        )
        .is_err());
        assert_eq!(active, vec![old, added]);
        reconcile(
            &mut active,
            &[old],
            |_| panic!("old shortcut is still registered"),
            |key| {
                assert_eq!(key, added);
                Ok(())
            },
        )
        .unwrap();
        assert_eq!(active, vec![old]);
        reconcile(&mut active, &[], |_| unreachable!(), |_| Ok(())).unwrap();
        assert!(active.is_empty());
    }
}
