use serde::Deserialize;

#[derive(Clone, Deserialize)]
pub struct MaterialRect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub radius: f64,
}

#[derive(Clone, Deserialize)]
pub struct Viewport {
    pub width: f64,
    pub height: f64,
}

#[tauri::command]
pub async fn update_materials(
    window: tauri::WebviewWindow,
    regions: Vec<MaterialRect>,
    dark: bool,
    follow_system: bool,
    viewport: Viewport,
) -> Result<String, String> {
    if window.label() != "main" {
        return Err("Materials are only available in the main window".into());
    }
    if !viewport.width.is_finite()
        || !viewport.height.is_finite()
        || viewport.width <= 0.0
        || viewport.height <= 0.0
    {
        return Err("Invalid material viewport".into());
    }
    if regions.len() > 2
        || regions.iter().any(|r| {
            [r.x, r.y, r.width, r.height, r.radius]
                .iter()
                .any(|n| !n.is_finite())
                || r.width < 0.0
                || r.height < 0.0
                || !(0.0..=32.0).contains(&r.radius)
        })
    {
        return Err("Invalid material regions".into());
    }
    #[cfg(target_os = "macos")]
    {
        let (send, receive) = tokio::sync::oneshot::channel();
        window
            .with_webview(move |webview| {
                let result = unsafe {
                    crate::platform::update_materials(
                        webview.inner(),
                        &regions,
                        dark,
                        follow_system,
                        &viewport,
                    )
                };
                let _ = send.send(result);
            })
            .map_err(|error| error.to_string())?;
        receive.await.map_err(|error| error.to_string())
    }
    #[cfg(not(target_os = "macos"))]
    {
        let _ = (regions, dark, follow_system, viewport);
        Ok("solid".into())
    }
}
