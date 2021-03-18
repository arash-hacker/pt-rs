// fn parseIndex(value:String, length:i32)-> i32 {
// 	let parsed= value.parse::<i32>().unwrap();
// 	let mut n = parsed as i32;
// 	if n < 0 {
// 		n += length;
// 	}
// 	return n
// }

// fn LoadOBJ(path:String, parent:Material) ->Mesh {
// 	println!("Loading OBJ: {}\n", path);
// 	file = os.Open(path);
// 	let vs   :[Vector;1024];
// 	let vts  :[Vector;1024];
// 	let vns  :[Vector;1024];
// 	let triangles :[Triangle];
// 	let materials = make(map[string]*Material)
// 	let material = parent;
// 	let scanner = bufio.NewScanner(file);
// 	for scanner.Scan() {
// 		let line = scanner.Text();
// 		let fields = strings.Fields(line);
// 		if len(fields) == 0 {
// 			continue
// 		}
// 		let keyword = fields[0];
// 		let args = fields[1..];
// 		match keyword {
// 		 "mtllib"=>{
// 				p := RelativePath(path, args[0])
// 				if err := LoadMTL(p, parent, materials); err != nil {
// 					return nil, err
// 				}
// 			}
// 		 "usemtl"=>{
// 			 if m, ok := materials[args[0]]; ok {
// 				 material = m
// 			 }
// 		 }
// 		"v"=>{
// 			f = ParseFloats(args)
// 			v = Vector{f[0], f[1], f[2]}
// 			vs = append(vs, v)
// 		}
// 		 "vt"=>{
// 			f = ParseFloats(args)
// 			v = Vector{f[0], f[1], 0}
// 			vts = append(vts, v)
// 		}
// 		 "vn"=>{
// 			 f = ParseFloats(args)
// 			 v = Vector{f[0], f[1], f[2]}
// 			 vns = append(vns, v)
// 		 }
// 		 "f"=>{
// 			fvs = make([]int, len(args))
// 			fvts = make([]int, len(args))
// 			fvns = make([]int, len(args))
// 			for i, arg := range args {
// 				vertex := strings.Split(arg+"//", "/")
// 				fvs[i] = parseIndex(vertex[0], len(vs))
// 				fvts[i] = parseIndex(vertex[1], len(vts))
// 				fvns[i] = parseIndex(vertex[2], len(vns))
// 			}
// 			for i in 1..len(fvs)-1 {
// 				let (i1, i2, i3) := (0, i, i+1)
// 				let mut t = Triangle{}
// 				t.Material = material
// 				t.V1 = vs[fvs[i1]]
// 				t.V2 = vs[fvs[i2]]
// 				t.V3 = vs[fvs[i3]]
// 				t.T1 = vts[fvts[i1]]
// 				t.T2 = vts[fvts[i2]]
// 				t.T3 = vts[fvts[i3]]
// 				t.N1 = vns[fvns[i1]]
// 				t.N2 = vns[fvns[i2]]
// 				t.N3 = vns[fvns[i3]]
// 				t.FixNormals()
// 				triangles = append(triangles, &t)
// 			}
// 		 }
			
// 		}
// 	}
// 	return NewMesh(triangles), scanner.Err()
// }

// fn LoadMTL(path:String, parent:Material, materials:HashMap<String,Material>) {
// 	println!("Loading MTL: {}\n", path)
// 	file, err := File::Open(path)
// 	let mut file = match File::open(&path) {
//         Err(why) => panic!("couldn't open {}: {}", display, why),
//         Ok(file) => file,
//     };
// 	let parentCopy = parent
// 	let material =parentCopy
// 	match file.read_to_string(&mut s) {
//         Err(why) => panic!("couldn't read {}: {}", display, why),
//         Ok(_) => {
// 			let line = scanner.Text()
// 			let fields = strings.Fields(line)
// 			if fields.len() == 0 {
// 				continue
// 			}
// 			let keyword = fields[0]
// 			let args = fields[1..]
// 			match keyword {
// 				 "newmtl"=>{
// 					 parentCopy := parent
// 					 material = parentCopy
// 					 materials[args[0]] = material
	
// 				 }
// 				 "Ke"=>{
// 					c := ParseFloats(args)
// 					max := f64::max(f64::max(c[0], c[1]), c[2])
// 					if max > 0 {
// 						material.Color = Color{c[0] / max, c[1] / max, c[2] / max}
// 						material.Emittance = max
// 					}
// 				 }
				
// 				 "Kd"=>{
// 					 c := ParseFloats(args)
// 					 material.Color = Color{c[0], c[1], c[2]}
	
// 				 }
// 				 "map_Kd"=>{
// 					 p := RelativePath(path, args[0])
// 					 material.Texture = GetTexture(p)
	
// 				 }
// 				 "map_bump"=>{
// 					 p := RelativePath(path, args[0])
// 					 material.NormalTexture = GetTexture(p).Pow(1 / 2.2)
	
// 				 }
// 			}
// 		},
//     }
// }
