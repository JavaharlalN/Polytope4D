use std::fs;
use super::error::*;
use crate::save::SData;
use crate::objects::Vec4f;
use std::convert::TryInto;
use crate::objects::Object;
use macroquad::prelude::show_mouse;

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

fn cut(bytes: &mut Vec<u8>, count: u32) -> Vec<u8> {
    let mut res = vec![];
    for _ in 0..count {
        res.push(bytes.remove(0));
    }
    return res;
}

fn deserialize_u64_from_bytes(bytes: &mut Vec<u8>) -> Result<u64, Error> {
    match DData::Uint(cut(bytes, 8)).deserialize()? {
        SData::Ulong(v) => Ok(v),
        _ => return Err(Error::Import(ImportError::FileCorrupted)),
    }
}

/// File structure
/// First 8 bytes: 0x4d4d4d4d (extention)
/// Next 8 bytes: vertices count (a)
/// Next 8 bytes: edges count (b)
/// Next 8 bytes: faces count (c)
/// Next 8 bytes: cells count (d)
/// Next 8 bytes: XW angle
/// Next 8 bytes: YW angle
/// Next 8 bytes: ZW angle
/// Next a * 4 * 8 bytes (vertices): f32, f32, f32, f32 - coordinates
/// Next b * 2 * 8 bytes (edges): u32, u32 - vertices indices
/// Next c * 6 * 8 bytes (faces): u32, u32, u32 - vertices indices and u32, u32, u32 - edges indices
/// Next d * 14 * 8 bytes (cells): u32 (4 times) - vertices indices, u32 (6 times) - edges, u32 (4 times) - faces
pub fn open_4dp() -> Result<Object, Error> {
    let path = match open_with_explorer() {
        Some(p) => p,
        None => return Err(Error::Cancel),
    };
    let mut bytes = match fs::read(path) {
        Ok(b) => b,
        Err(_) => return Err(Error::Import(ImportError::FileNotFound)),
    };
    let extention = cut(&mut bytes, 8);
    if hex::encode(&extention) != "4d4d4d4d" { return Err(Error::Import(ImportError::InvalidExtention)); }
    let vertices_count = deserialize_u64_from_bytes(&mut bytes)?;
    let edges_count = deserialize_u64_from_bytes(&mut bytes)?;
    let faces_count = deserialize_u64_from_bytes(&mut bytes)?;
    let cells_count = deserialize_u64_from_bytes(&mut bytes)?;
    let mut vertices = vec![];
    let mut edges = vec![];
    let mut faces = vec![];
    let mut cells = vec![];
    for _ in 0..vertices_count { vertices.push(DData::Vector(cut(&mut bytes, 32)).deserialize()?.as_vector()?) }
    for _ in 0..edges_count { edges.push(DData::Vector(cut(&mut bytes, 16)).deserialize()?.as_edge()?); }
    for _ in 0..faces_count { faces.push(DData::Vector(cut(&mut bytes, 48)).deserialize()?.as_face()?); }
    for _ in 0..cells_count { cells.push(DData::Vector(cut(&mut bytes, 112)).deserialize()?.as_cell()?); }
    Ok(Object {
        vertices,
        edges,
        faces,
        cells,
        name: None,
    })
}

#[derive(Debug, Clone)]
pub enum DData {
    Vector(Vec<u8>), // 32 bytes
    Edge(Vec<u8>), // 8
    Face(Vec<u8>), // 24
    Cell(Vec<u8>), // 56
    Uint(Vec<u8>), // 8
    Float(Vec<u8>), // 8
}

fn cut_float(bytes: &mut Vec<u8>) -> Result<f32, Error> {
    Ok(f32::from_be_bytes(
        cut(bytes, 4)
            .as_slice()
            .try_into()
            .map_err(|_| Error::Import(ImportError::FileCorrupted))?
    ))
}

fn cut_u64(bytes: &mut Vec<u8>) -> Result<u64, Error> {
    Ok(u64::from_be_bytes(
        cut(bytes, 8)
            .as_slice()
            .try_into()
            .map_err(|_| Error::Import(ImportError::FileCorrupted))?
    ))
}

impl DData {
    fn deserialize(&self) -> Result<SData, Error> {
        match self {
            DData::Vector(b) => {
                let mut bytes = b.clone();
                let x = cut_float(&mut bytes)?;
                let y = cut_float(&mut bytes)?;
                let z = cut_float(&mut bytes)?;
                let w = cut_float(&mut bytes)?;
                return Ok(SData::Vector(Vec4f::new(x, y, z, w)));
            },
            DData::Edge(b) => {
                let mut bytes = b.clone();
                return Ok(SData::Edge(
                    cut_u64(&mut bytes)? as usize,
                    cut_u64(&mut bytes)? as usize
                ));
            },
            DData::Face(b) => todo!(),
            DData::Cell(b) => todo!(),
            DData::Uint(b) => return Ok(SData::Ulong(u64::from_be_bytes(b.as_slice().try_into().unwrap()))),
            DData::Float(b) => return Ok(SData::Float(f32::from_be_bytes(b.as_slice().try_into().unwrap()))),
        }
    }
}