use crate::{mesh::Mesh, shape::*};
use crate::material::*;
use crate::hit::*;
use crate::vector::*;
use crate::bbox::*;
use crate::sdf::*;
use crate::ray::*;
use crate::matrix::*;
use std::env;
use std::fs;

pub struct STLHeader{
	pub _s :[u8;80],
	pub Count :u32,
}

pub struct STLTriangle{
	pub N:[f32;3],
	pub V1:[f32;3],
	pub V2:[f32;3],
	pub V3: [f32;3],
	pub _s :u16,
}

//pub fn LoadSTL(path:String, material:Material)-> (Mesh, error) {
// 	println!("Loading STL: {}\n", path);

// 	// open file
// 	let file = fs::read_to_string(path).expect("????");
// 	// get file size
// 	let info = file.Stat();
// 	let size =my_file.metadata().unwrap().len();

// 	// read header, get expected binary size
// 	let header = STLHeader::Default();
// 	binary.Read(file, binary.LittleEndian, &header);
// 	let expectedSize = (header.Count)*50 + 84;
 	
// 	let path_to_read = Path::new("new.txt");
// 	// parse ascii or binary stl
// 	if size == expectedSize {
// 		return loadSTLB(file, material)
// 	} else {
// 		return loadSTLA(file, material)
// 	}
// }

//pub fn loadSTLA(file :File, material: Material)-> (Mesh, ()) {
// 	let mut vertexes: [Vector];
// 	let scanner = BufReader::new(file);;
// 	// bufio.NewScanner(file);
// 	for line in scanner.lines() {
// 		fields = strings.Fields(line);
// 		if fields.len() == 4 && fields[0] == "vertex" {
// 			let f = ParseFloats(fields[1..]);
// 			let v = Vector{X:f[0],Y: f[1],Z: f[2]};
// 			vertexes.append(vertexes, v)
// 		}
// 	}
// 	let triangles :Vec<Triangle>;
// 	for i in 0..len(vertexes).step_by(3) {
// 		let t = Triangle::Default();
// 		t.Material = &material;
// 		t.V1 = vertexes[i+0];
// 		t.V2 = vertexes[i+1];
// 		t.V3 = vertexes[i+2];
// 		t.FixNormals();
// 		triangles.insert(t);
// 	}
// 	return NewMesh(triangles)
// }

//pub fn loadSTLB(file: File, material: Material) ->Mesh {
// 	let header = STLHeader::Default();
// 	let count = header.Count as i32;
// 	let mut triangles = [Triangle; count];
// 	for i in 0..count {
// 		let d = STLTriangle::Default();
// 		let t = Triangle::Default();
// 		t.Material = material;
// 		t.V1 = Vector{X:d.V1[0],Y: d.V1[1],Z: d.V1[2]};
// 		t.V2 = Vector{X:d.V2[0],Y: d.V2[1],Z: d.V2[2]};
// 		t.V3 = Vector{X:d.V3[0],Y: d.V3[1],Z: d.V3[2]};
// 		t.FixNormals();
// 		triangles[i] = t;
// 	}
// 	return NewMesh(triangles)
// }

pub fn SaveSTL(path:String, mesh :Mesh) ->() {
	// let (file, err) = os.Create(path);
	// let header = STLHeader::Default();
	// header.Count = ((mesh.Triangles).len()) as i32;
	// //TODO:>>>
	// //binary.Write(file, binary.LittleEndian, header);
	// for triangle in mesh.Triangles {
	// 	let n = triangle.Normal();
	// 	let d = STLTriangle::Default();
	// 	d.N[0] = (n.X) as f32;
	// 	d.N[1] = (n.Y) as f32;
	// 	d.N[2] = (n.Z) as f32;
	// 	d.V1[0] = (triangle.V1.X) as f32;
	// 	d.V1[1] = (triangle.V1.Y) as f32;
	// 	d.V1[2] = (triangle.V1.Z) as f32;
	// 	d.V2[0] = (triangle.V2.X) as f32;
	// 	d.V2[1] = (triangle.V2.Y) as f32;
	// 	d.V2[2] = (triangle.V2.Z) as f32;
	// 	d.V3[0] = (triangle.V3.X) as f32;
	// 	d.V3[1] = (triangle.V3.Y) as f32;
	// 	d.V3[2] = (triangle.V3.Z) as f32;
	// 	//binary.Write(file, binary.LittleEndian, &d)
	// }
	// return None
}
