/**
 * global home panel, circle max level
 */
use tauri::{AppHandle, WebviewWindow};
use tauri_nspanel::{
    cocoa::appkit::NSWindowCollectionBehavior, panel_delegate, ManagerExt, WebviewWindowExt
};
use crate::constants::MAIN_WINDOW;

pub fn init_home_panel(app_handle: &AppHandle, window: WebviewWindow) {

  let panel = window.to_panel().unwrap();

  let delegate = panel_delegate!(MyPanelDelegate {
      window_did_become_key,
      window_did_resign_key
  });

  let handle = app_handle.to_owned();

  delegate.set_listener(Box::new(move |delegate_name: String| {
      match delegate_name.as_str() {
          "window_did_become_key" => {
              let app_name = handle.package_info().name.to_owned();

              println!("[info]: {:?} panel becomes key window!", app_name);
          }
          "window_did_resign_key" => {
              println!("[info]: panel resigned from key window!");
          }
          _ => (),
      }
  }));

  // Set the window to float level
  #[allow(non_upper_case_globals)]
  const NSFloatWindowLevel: i32 = 1114;
  panel.set_level(NSFloatWindowLevel);

  #[allow(non_upper_case_globals)]
  const NSWindowStyleMaskNonActivatingPanel: i32 = 1 << 7;
  // Ensures the panel cannot activate the app
  panel.set_style_mask(NSWindowStyleMaskNonActivatingPanel);


  #[allow(non_upper_case_globals)]
  panel.set_content_size(60.0, 60.0);

  // Allows the panel to:
  // - display on the same space as the full screen window
  // - join all spaces
  panel.set_collection_behaviour(
      NSWindowCollectionBehavior::NSWindowCollectionBehaviorFullScreenAuxiliary
          | NSWindowCollectionBehavior::NSWindowCollectionBehaviorCanJoinAllSpaces,
  );

  panel.set_delegate(delegate);
}

pub fn show_home_panel(app: &AppHandle) {
    let panel = app.get_webview_panel(MAIN_WINDOW).unwrap();
    panel.show();
}

pub fn hide_home_panel(app: &AppHandle) {
    let panel = app.get_webview_panel(MAIN_WINDOW).unwrap();
    panel.order_out(None);
}

pub fn close_home_panel(handle: AppHandle) {
  let panel = handle.get_webview_panel(MAIN_WINDOW).unwrap();

  panel.set_released_when_closed(true);

  panel.close();
}
