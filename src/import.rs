use macroquad::prelude::show_mouse;
use crate::objects::Object;
use std::fs;
use super::error::*;

fn open_with_explorer() -> Option<String> {
	show_mouse(true);
	let path = rfd::FileDialog::new()
		.add_filter("4D object", &["4dp"])
		.add_filter("4D object", &["*"])
		.set_directory("/")
        .set_title("Открыть как")
		.pick_file();
	show_mouse(false);
	match path {
		Some(p) => p.as_os_str().to_str().map(|s| s.to_string()),
		None => None,
	}
}

/// File structure
/// First 8 bytes: 0x4d4d4d4d (extention)
/// Next 4 bytes (a): vertices count
/// Next 4 bytes (b): edges count
/// Next 4 bytes (c): faces count
/// Next 4 bytes (d): cells count
/// Next 4 bytes: XW angle
/// Next 4 bytes: YW angle
/// Next 4 bytes: ZW angle
/// Next a * 4 bytes (vertices): f32, f32, f32, f32 - coordinates
/// Next b * 2 bytes (edges): u32, u32 - vertices indices
/// Next c * 6 bytes (faces): u32, u32, u32 - vertices indices and u32, u32, u32 - edges indices
/// Next d * 14 bytes (cells): u32 (4 times) - vertices indices, u32 (6 times) - edges, u32 (4 times) - faces
pub fn open_4dp() -> Result<Option<Vec<Object>>, Error> {
    let path = match open_with_explorer() {
        Some(p) => p,
        None => return Ok(None),
    };
    let bytes = match fs::read(path) {
        Ok(b) => b,
        Err(_) => return Err(Error::Import(ImportError::FileNotFound)),
    };
    if hex::encode(bytes) != "4d4d4d4d" { return Err(Error::Import(ImportError::InvalidExtention)); }
    Ok(None)
}

#[derive(Debug, Clone)]
enum Data {
    Vector([u8; 4]),
    Edge([u8; 8]),
    Face([u8; 24]),
    Cell([u8; 56]),
    Uint([u8; 8]),
    Float([u8; 8]),
}