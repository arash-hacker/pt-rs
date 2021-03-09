struct Mesh {
	Triangles :[Triangle],
	box:Box,
	tree:Tree,
}

fn NewMesh(triangles [Triangle])-> Mesh {
	return Mesh{triangles, None, None}
}
impl Mesh{
	fn dirty(&self) {
		self.box = None
		self.tree = None
	}
	
	fn Copy(&self)-> Mesh {
		let mut triangles = [Triangle;self.Triangles.len()], )
		for i, t in self.Triangles {
			a = *t
			triangles[i] = &a
		}
		return NewMesh(triangles)
	}
	
	fn  Compile(&self) {
		if self.tree == None {
			let mut shapes = [Shape; self.Triangles.len()]
			for i, triangle in self.Triangles {
				shapes[i] = triangle
			}
			self.tree = NewTree(shapes)
		}
	}
	
	fn Add(&self,b *Mesh) {
		self.Triangles = append(a.Triangles, b.Triangles...)
		self.dirty()
	}
	
	fn BoundingBox(&self) -> Box {
		if self.box == None {
			let mut min = self.Triangles[0].V1
			let mut max = self.Triangles[0].V1
			for _, t in self.Triangles.iter().enumerate() {
				min = min.Min(t.V1).Min(t.V2).Min(t.V3)
				max = max.Max(t.V1).Max(t.V2).Max(t.V3)
			}
			self.box = Box{min, max}
		}
		return self.box
	}
	
	fn Intersect(&self, r Ray) -> Hit {
		return self.tree.Intersect(r)
	}
	
	fn UV(&self, p Vector) -> Vector {
		return Vector{} // not implemented
	}
	
	fn MaterialAt(&self, p Vector) -> Material {
		return Material{} // not implemented
	}
	
	fn NormalAt(&self, p Vector) -> Vector {
		return Vector{} // not implemented
	}
	
	fn smoothNormalsThreshold(&self, normal:Vector, normals:Vec<Vector>, threshold:f64)-> Vector {
		let mut result = Vector{}
		for _, x in normals {
			if x.Dot(normal) >= threshold {
				result = result.Add(x)
			}
		}
		return result.Normalize()
	}
	
	fn SmoothNormalsThreshold(&self, radians float64) {
		let threshold = f64::cos(radians)
		let lookup = make(map[Vector][]Vector)
		for _, t := range self.Triangles {
			lookup[t.V1] = append(lookup[t.V1], t.N1)
			lookup[t.V2] = append(lookup[t.V2], t.N2)
			lookup[t.V3] = append(lookup[t.V3], t.N3)
		}
		for _, t in self.Triangles.iter().enumerate() {
			t.N1 = smoothNormalsThreshold(t.N1, lookup[t.V1], threshold)
			t.N2 = smoothNormalsThreshold(t.N2, lookup[t.V2], threshold)
			t.N3 = smoothNormalsThreshold(t.N3, lookup[t.V3], threshold)
		}
	}
	
	fn SmoothNormals(&self) {
		let mut lookup :HashMap<Vector,Vector>=HashMap::new()
		for _, t in self.Triangles.iter().enumerate() {
			lookup[t.V1] = lookup[t.V1].Add(t.N1)
			lookup[t.V2] = lookup[t.V2].Add(t.N2)
			lookup[t.V3] = lookup[t.V3].Add(t.N3)
		}
		for k, v in lookup.iter().enumerate() {
			lookup[k] = v.Normalize()
		}
		for _, t in self.Triangles.iter_mut().enumerate() {
			t.N1 = lookup[t.V1]
			t.N2 = lookup[t.V2]
			t.N3 = lookup[t.V3]
		}
	}
	
	fn UnitCube(&self) {
		self.FitInside(Box{Vector{}, Vector{1, 1, 1}}, Vector{})
		self.MoveTo(Vector{}, Vector{0.5, 0.5, 0.5})
	}
	
	fn MoveTo(&self, position:Vector, anchor:Vector) {
		let matrix = Translate(position.Sub(self.BoundingBox().Anchor(anchor)))
		self.Transform(matrix)
	}
	
	fn FitInside(&self, box:Box, anchor:Vector) {
		let scale = box.Size().Div(self.BoundingBox().Size()).MinComponent()
		let extra = box.Size().Sub(self.BoundingBox().Size().MulScalar(scale))
		let mut matrix = Identity()
		matrix = matrix.Translate(self.BoundingBox().Min.Negate())
		matrix = matrix.Scale(Vector{scale, scale, scale})
		matrix = matrix.Translate(box.Min.Add(extra.Mul(anchor)))
		self.Transform(matrix)
	}
	
	fn Transform(&self, matrix:Matrix) {
		for _, t in self.Triangles.iter().enumerate() {
			t.V1 = matrix.MulPosition(t.V1)
			t.V2 = matrix.MulPosition(t.V2)
			t.V3 = matrix.MulPosition(t.V3)
			t.N1 = matrix.MulDirection(t.N1)
			t.N2 = matrix.MulDirection(t.N2)
			t.N3 = matrix.MulDirection(t.N3)
		}
		self.dirty()
	}
	
	fn etMaterial(&self, material:Material) {
		for _, t in self.Triangles {
			t.Material = &material
		}
	}
	
	fn SaveSTL(&self, path:&str) {
		return SaveSTL(path, self)
	}
	
}
