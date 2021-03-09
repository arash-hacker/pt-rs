use std::env;
use std::fs;

struct STLHeader{
	_ :[u8;80],
	Count :u32,
}

struct STLTriangle{
	N:[3]f32,
	V1:[3]f32,
	V2:[3]f32,
	V3: [3]f32,
	_ :u16,
}

fn LoadSTL(path:String, material:Material)-> (Mesh, error) {
	println!("Loading STL: {}\n", path)

	// open file
	let file = fs::read_to_string(path).expect("????");
	// get file size
	info, err := file.Stat()
	if err != nil {
		return nil, err
	}

	let size =my_file.metadata().unwrap().len();

	// read header, get expected binary size
	let header = STLHeader{}
	if err = binary.Read(file, binary.LittleEndian, &header); err != nil {
		return nil, err
	}
	expectedSize := int64(header.Count)*50 + 84
 	
	let path_to_read = Path::new("new.txt");
	// parse ascii or binary stl
	if size == expectedSize {
		return loadSTLB(file, material)
	} else {
		return loadSTLA(file, material)
	}
}

fn loadSTLA(file *os.File, material: Material)-> (*Mesh, ()) {
	var vertexes [Vector]
	scanner := bufio.NewScanner(file)
	for scanner.Scan() {
		line := scanner.Text()
		fields := strings.Fields(line)
		if len(fields) == 4 && fields[0] == "vertex" {
			f := ParseFloats(fields[1:])
			v := Vector{f[0], f[1], f[2]}
			vertexes = append(vertexes, v)
		}
	}
	let triangles Vec<Triangle>;
	for i = 0..len(vertexes).step_by(3) {
		let t = Triangle{}
		t.Material = &material
		t.V1 = vertexes[i+0]
		t.V2 = vertexes[i+1]
		t.V3 = vertexes[i+2]
		t.FixNormals()
		triangles.insert(t)
	}
	return NewMesh(triangles), scanner.Err()
}

fn loadSTLB(file *os.File, material Material) (*Mesh, error) {
	let header = STLHeader{}
	let count = header.Count as i32
	let mut triangles = [Triangle; count]
	for i in 0..count {
		let d = STLTriangle{}
		let t = Triangle{}
		t.Material = &material
		t.V1 = Vector{float64(d.V1[0]), float64(d.V1[1]), float64(d.V1[2])}
		t.V2 = Vector{float64(d.V2[0]), float64(d.V2[1]), float64(d.V2[2])}
		t.V3 = Vector{float64(d.V3[0]), float64(d.V3[1]), float64(d.V3[2])}
		t.FixNormals()
		triangles[i] = t
	}
	return NewMesh(triangles), None
}

fn SaveSTL(path:String, mesh *Mesh) error {
	file, err := os.Create(path)
	if err != nil {
		return err
	}
	header = STLHeader{}
	header.Count = ((mesh.Triangles).len) as i32
	if err := binary.Write(file, binary.LittleEndian, &header); err != nil {
		return err
	}
	for triangle in mesh.Triangles {
		let n = triangle.Normal()
		let d = STLTriangle{}
		d.N[0] = float32(n.X)
		d.N[1] = float32(n.Y)
		d.N[2] = float32(n.Z)
		d.V1[0] = float32(triangle.V1.X)
		d.V1[1] = float32(triangle.V1.Y)
		d.V1[2] = float32(triangle.V1.Z)
		d.V2[0] = float32(triangle.V2.X)
		d.V2[1] = float32(triangle.V2.Y)
		d.V2[2] = float32(triangle.V2.Z)
		d.V3[0] = float32(triangle.V3.X)
		d.V3[1] = float32(triangle.V3.Y)
		d.V3[2] = float32(triangle.V3.Z)
		if err := binary.Write(file, binary.LittleEndian, &d); err != nil {
			return err
		}
	}
	return None
}
