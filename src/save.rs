use macroquad::prelude::show_mouse;
use crate::error::Error;
use crate::objects::Edge;
use crate::objects::Vec4f;
use crate::objects::Object;

fn save_with_explorer() -> Option<String> {
	show_mouse(true);
	let path = rfd::FileDialog::new()
		.add_filter("4D object", &["p4d"])
		.set_directory("/")
		.save_file();
	show_mouse(false);
	match path {
		Some(p) => p.as_os_str().to_str().map(|s| s.to_string()),
		None => None,
	}
}

fn choose_dir() -> Option<String> {
	show_mouse(true);
	let path = rfd::FileDialog::new()
		.set_directory("/")
		.pick_folder();
	show_mouse(false);
	match path {
		Some(p) => p.as_os_str().to_str().map(|s| s.to_string()),
		None => None,
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
pub fn save(objects: &Vec<Object>) {
	let extention = "4dp";
	let filename = save_with_explorer().map(|s| format!("{}.{}", s, extention));
	let path = match filename {
		Some(dir) => dir,
		None => return,
	};
	let mut vertices = vec![];
	let mut edges = vec![];
	let mut faces = vec![];
	let mut cells = vec![];
	for obj in objects.iter() {
		for v in &obj.vertices {
			vertices.push(SData::Vector(v.clone()));
		}
		for e in &obj.edges {
			edges.push(SData::Edge(e.a, e.b));
		}
		for f in &obj.faces {
			faces.push(SData::Face(f.0, f.1));
		}
		for c in &obj.cells {
			cells.push(SData::Cell(c.0, c.1, c.2))
		}
	}
	let mut data = vec![];
	let extention = SData::Uint(0x4d4d4d4d);
	let vertices_count = SData::Uint(vertices.len() as u32);
	let edges_count = SData::Uint(edges.len() as u32);
	let faces_count = SData::Uint(faces.len() as u32);
	let cells_count = SData::Uint(cells.len() as u32);
	data.append(&mut extention.serialize());
	data.append(&mut vertices_count.serialize());
	data.append(&mut edges_count.serialize());
	data.append(&mut faces_count.serialize());
	data.append(&mut cells_count.serialize());
	for v in vertices { data.append(&mut v.serialize()); }
	for e in edges { data.append(&mut e.serialize()); }
	for f in faces { data.append(&mut f.serialize()); }
	for c in cells { data.append(&mut c.serialize()); }
	let res = std::fs::write(&path, data);
	match res {
		Ok(_) => println!("{}", path),
		Err(e) => println!("failed to save, reason: {}", e),
	}
}

pub enum SData {
	Vector(Vec4f),
	Edge(usize, usize),
	Face(
		(usize, usize, usize),
		(usize, usize, usize)
	),
	Cell(
		(usize, usize, usize, usize),
		(usize, usize, usize, usize, usize, usize),
		(usize, usize, usize, usize)
	),
	Uint(u32),
	Ulong(u64),
	Float(f32),
}

impl SData {
	pub fn as_vector(&self) -> Result<Vec4f, Error> {
		if let Self::Vector(v) = self { Ok(v.clone()) }
		else { Err(Error::Unknown) }
	}

	pub fn as_edge(&self) -> Result<Edge, Error> {
		if let Self::Edge(a, b) = self { Ok(Edge::new(*a, *b)) }
		else { Err(Error::Unknown) }
	}

	pub fn as_face(&self) -> Result<(
		(usize, usize, usize),
		(usize, usize, usize),
		bool
	), Error> {
		if let Self::Face((v0, v1, v2), (a, b, c)) = self {
			Ok(((*v0, *v1, *v2), (*a, *b, *c), false))
		} else { Err(Error::Unknown) }
	}

	pub fn as_cell(&self) -> Result<(
		(usize, usize, usize, usize),
		(usize, usize, usize, usize, usize, usize),
		(usize, usize, usize, usize),
		bool,
	), Error> {
		if let Self::Cell(vertices, edges, faces) = *self {
			Ok((vertices, edges, faces, false))
		} else { Err(Error::Unknown) }
	}

	fn serialize(&self) -> Vec<u8> {
		let mut res = vec![];
		match self {
			SData::Vector(v) => {
				let coords = v.as_vec();
				for bytes in coords {
					res.append(&mut SData::Float(bytes).serialize());
				}
			},
			SData::Edge(a, b) => {
				res.append(&mut SData::Uint(*a as u32).serialize());
				res.append(&mut SData::Uint(*b as u32).serialize());
			},
			SData::Face(v, e) => {
				for uint in vec![v.0, v.1, v.2, e.0, e.1, e.2] {
					res.append(&mut SData::Uint(uint as u32).serialize());
				}
			},
			SData::Cell(v, e, f) => {
				for uint in vec![
					v.0, v.1, v.2, v.3,
					e.0, e.1, e.2, e.3, e.4, e.5,
					f.0, f.1, f.2, f.3,
				] {
					res.append(&mut SData::Uint(uint as u32).serialize());
				}
			},
			SData::Uint(u) => {
				res = (*u).to_be_bytes().into();
			},
			SData::Ulong(u) => {
				res = (*u).to_be_bytes().into();
			}
			SData::Float(f) => {
				res = (*f).to_be_bytes().into();
			},
		}
		return res
	}
}