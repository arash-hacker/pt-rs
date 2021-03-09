struct Cube {
	Min   :   Vector,
	Max   :   Vector,
	Material: Material,
	Box     : Box,
}

fn NewCube(min:Vectore, max: Vector, material: Material)->Cube {
	box := Box{min, max}
	return Cube{min, max, material, box}
}


impl Cube{
	fn Compile(&self) {}

	fn BoundingBox(&self)-> Box {
		return self.Box
	}

	fn Intersect(&self,r Ray)-> Hit {
		n := self.Min.Sub(r.Origin).Div(r.Direction)
		f := self.Max.Sub(r.Origin).Div(r.Direction)
		n, f = n.Min(f), n.Max(f)
		t0 := f64::max(f64::max(n.X, n.Y), n.Z)
		t1 := f64::min(f64::min(f.X, f.Y), f.Z)
		if t0 > 0 && t0 < t1 {
			return Hit{c, t0, nil}
		}
		return NoHit
	}

	fn UV(&self,p:Vector) ->Vector {
		p = p.Sub(self.Min).Div(self.Max.Sub(self.Min))
		return Vector{p.X, p.Z, 0}
	}

	fn MaterialAt(&self,p:Vector) ->Material {
		return self.Material
	}

	fn NormalAt(&self,p:Vector) ->Vector {
		match {
		  p.X < self.Min.X+EPS=>return Vector{-1, 0, 0}
		  p.X > self.Max.X-EPS=>return Vector{1, 0, 0}
		  p.Y < self.Min.Y+EPS=>return Vector{0, -1, 0}
		  p.Y > self.Max.Y-EPS=>return Vector{0, 1, 0}
		  p.Z < self.Min.Z+EPS=>return Vector{0, 0, -1}
		  p.Z > self.Max.Z-EPS=>return Vector{0, 0, 1}
		}
		return Vector{0, 1, 0}
	}

	fn Mesh(&self)-> Mesh {
		let mut a = self.Min;
		let mut b = self.Max;
		let mut z = Vector{};
		let mut m = self.Material;
		let mut v000 = Vector{a.X, a.Y, a.Z};
		let mut v001 = Vector{a.X, a.Y, b.Z};
		let mut v010 = Vector{a.X, b.Y, a.Z};
		let mut v011 = Vector{a.X, b.Y, b.Z};
		let mut v100 = Vector{b.X, a.Y, a.Z};
		let mut v101 = Vector{b.X, a.Y, b.Z};
		let mut v110 = Vector{b.X, b.Y, a.Z};
		let mut v111 = Vector{b.X, b.Y, b.Z};
		let mut triangles:[Triangle] ={
			NewTriangle(v000, v100, v110, z, z, z, m),
			NewTriangle(v000, v110, v010, z, z, z, m),
			NewTriangle(v001, v101, v111, z, z, z, m),
			NewTriangle(v001, v111, v011, z, z, z, m),
			NewTriangle(v000, v100, v101, z, z, z, m),
			NewTriangle(v000, v101, v001, z, z, z, m),
			NewTriangle(v010, v110, v111, z, z, z, m),
			NewTriangle(v010, v111, v011, z, z, z, m),
			NewTriangle(v000, v010, v011, z, z, z, m),
			NewTriangle(v000, v011, v001, z, z, z, m),
			NewTriangle(v100, v110, v111, z, z, z, m),
			NewTriangle(v100, v111, v101, z, z, z, m),
		};
		return NewMesh(triangles)
	}
}