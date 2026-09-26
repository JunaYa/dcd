use base64::{engine::general_purpose::STANDARD, Engine};
use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Pack {
    version: u8,
    name: String,
    frame_width: u32,
    frame_height: u32,
    states: BTreeMap<String, Animation>,
}
#[derive(Deserialize)]
struct Animation { image: String, frames: u32, fps: u32 }

pub fn validate_pack(source: &str) -> Result<(), ()> {
    if source.is_empty() { return Ok(()); }
    if source.len() > 4 * 1024 * 1024 { return Err(()); }
    let pack: Pack = serde_json::from_str(source).map_err(|_| ())?;
    if pack.version != 1 || pack.name.trim().is_empty() || pack.name.chars().count() > 40
        || !(8..=128).contains(&pack.frame_width) || !(8..=128).contains(&pack.frame_height) { return Err(()); }
    for state in ["ready", "working", "tired", "resting", "celebrate"] {
        let animation = pack.states.get(state).ok_or(())?;
        if !(1..=32).contains(&animation.frames) || !(1..=24).contains(&animation.fps) { return Err(()); }
        let bytes = STANDARD.decode(animation.image.strip_prefix("data:image/png;base64,").ok_or(())?).map_err(|_| ())?;
        let decoder = image::codecs::png::PngDecoder::new(std::io::Cursor::new(bytes)).map_err(|_| ())?;
        use image::ImageDecoder;
        if decoder.dimensions() != (pack.frame_width * animation.frames, pack.frame_height) { return Err(()); }
        let mut pixels = vec![0; decoder.total_bytes() as usize];
        decoder.read_image(&mut pixels).map_err(|_| ())?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn validates_builtin_and_rejects_external_or_malformed_images() {
        let source = include_str!("../../src/pet/default.json");
        assert!(validate_pack(source).is_ok());
        let mut pack: serde_json::Value = serde_json::from_str(source).unwrap();
        pack["states"]["ready"]["image"] = "https://example.com/pet.png".into();
        assert!(validate_pack(&pack.to_string()).is_err());
        assert!(validate_pack("{}").is_err());
    }
}
