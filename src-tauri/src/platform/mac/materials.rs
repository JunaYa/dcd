use crate::materials::{MaterialRect, Viewport};
use cocoa::base::{id, nil, NO, YES};
use cocoa::foundation::{NSPoint, NSRect, NSSize, NSString};
use objc::runtime::Class;
use objc::{class, msg_send, sel, sel_impl};
use std::ffi::c_void;

static SIDEBAR: u8 = 1;
static HEADER: u8 = 2;

#[link(name = "objc")]
extern "C" {
    fn objc_getAssociatedObject(object: id, key: *const c_void) -> id;
    fn objc_setAssociatedObject(object: id, key: *const c_void, value: id, policy: usize);
}

// Called by with_webview on the AppKit thread. Associated objects tie the material
// views to the webview lifetime; no native pointers escape onto worker threads.
pub unsafe fn update_materials(
    webview: *mut c_void,
    regions: &[MaterialRect],
    dark: bool,
    follow_system: bool,
    viewport: &Viewport,
) -> String {
    let webview = webview as id;
    let parent: id = msg_send![webview, superview];
    if parent == nil {
        return "solid".into();
    }
    let workspace: id = msg_send![class!(NSWorkspace), sharedWorkspace];
    let reduced: bool = msg_send![workspace, accessibilityDisplayShouldReduceTransparency];
    let contrast: bool = msg_send![workspace, accessibilityDisplayShouldIncreaseContrast];
    let glass = Class::get("NSGlassEffectView");
    let mode = if reduced || contrast {
        "solid"
    } else if glass.is_some() {
        "glass"
    } else {
        "vibrancy"
    };
    let name = NSString::alloc(nil).init_str(if dark {
        "NSAppearanceNameDarkAqua"
    } else {
        "NSAppearanceNameAqua"
    });
    let appearance: id = msg_send![class!(NSAppearance), appearanceNamed: name];
    let _: () = msg_send![name, release];
    let appearance = if follow_system { nil } else { appearance };
    let window: id = msg_send![webview, window];
    let _: () = msg_send![window, setAppearance: appearance];
    let background: id = msg_send![class!(NSColor), windowBackgroundColor];
    let _: () = msg_send![window, setBackgroundColor: background];
    let bounds: NSRect = msg_send![webview, frame];
    let flipped: bool = msg_send![parent, isFlipped];
    for (index, key) in [&SIDEBAR, &HEADER].iter().enumerate() {
        let key = *key as *const u8 as *const c_void;
        let mut view = objc_getAssociatedObject(webview, key);
        if mode == "solid" || index >= regions.len() || regions[index].height == 0.0 {
            if view != nil {
                let _: () = msg_send![view, setHidden: YES];
            }
            continue;
        }
        let region = &regions[index];
        if view == nil {
            let view_class = glass.unwrap_or_else(|| class!(NSVisualEffectView));
            let allocated: id = msg_send![view_class, alloc];
            view = msg_send![allocated, initWithFrame: NSRect::new(NSPoint::new(0.0, 0.0), NSSize::new(0.0, 0.0))];
            if glass.is_some() {
                let _: () = msg_send![view, setStyle: 0isize];
            } else {
                let _: () = msg_send![view, setMaterial: if index == 0 { 7isize } else { 10isize }];
                let _: () = msg_send![view, setBlendingMode: 0isize];
                let _: () = msg_send![view, setState: 0isize];
                let _: () = msg_send![view, setWantsLayer: YES];
            }
            let _: () = msg_send![parent, addSubview: view positioned: -1isize relativeTo: webview];
            objc_setAssociatedObject(webview, key, view, 1); // retain, nonatomic
            let _: () = msg_send![view, release];
        }
        let rect = material_frame(region, bounds, viewport, flipped);
        let _: () = msg_send![view, setFrame: rect];
        let _: () = msg_send![view, setAppearance: appearance];
        let radius = region.radius * bounds.size.width / viewport.width;
        if glass.is_some() {
            let _: () = msg_send![view, setCornerRadius: radius];
        } else {
            let layer: id = msg_send![view, layer];
            let _: () = msg_send![layer, setCornerRadius: radius];
            let _: () = msg_send![layer, setMasksToBounds: YES];
        }
        let _: () = msg_send![view, setHidden: NO];
    }
    mode.into()
}

fn material_frame(
    region: &MaterialRect,
    frame: NSRect,
    viewport: &Viewport,
    flipped: bool,
) -> NSRect {
    let scale = frame.size.width / viewport.width;
    let content_height = viewport.height * scale;
    // WKWebView's frame can include the titlebar while the DOM viewport excludes it.
    let y = if flipped {
        frame.size.height - content_height + region.y * scale
    } else {
        content_height - (region.y + region.height) * scale
    };
    NSRect::new(
        NSPoint::new(frame.origin.x + region.x * scale, frame.origin.y + y),
        NSSize::new(region.width * scale, region.height * scale),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn material_frame_accounts_for_titlebar_and_zoom() {
        let frame = NSRect::new(NSPoint::new(0.0, 0.0), NSSize::new(960.0, 540.0));
        let viewport = Viewport {
            width: 480.0,
            height: 254.0,
        };
        let region = MaterialRect {
            x: 108.0,
            y: 16.0,
            width: 356.0,
            height: 43.0,
            radius: 14.0,
        };
        let rect = material_frame(&region, frame, &viewport, false);
        assert_eq!((rect.origin.x, rect.origin.y), (216.0, 390.0));
        assert_eq!((rect.size.width, rect.size.height), (712.0, 86.0));
        let flipped = material_frame(&region, frame, &viewport, true);
        assert_eq!(flipped.origin.y, 64.0);
    }
}
