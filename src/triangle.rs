
struct Triangle{
	Material :  Material,
	V1:Vector, V2:Vector, V3 :Vector,
	N1:Vector, N2:Vector, N3: Vector,
	T1:Vector, T2:Vector, T3: Vector,
}

fn NewTriangle(v1:Vector, v2:Vector, v3:Vector, t1:Vector, t2:Vector, t3: Vector, material:Material) *Triangle {
	let t = Triangle{}
	t.V1 = v1
	t.V2 = v2
	t.V3 = v3
	t.T1 = t1
	t.T2 = t2
	t.T3 = t3
	t.Material = &material
	t.FixNormals()
	return &t
}

impl Triangle{
	fn Vertices(&self ) (Vector, Vector, Vector) {
		return self.V1, self.V2, self.V3
	}
	
	fn Compile(&self ) {
	}
	
	fn BoundingBox(&self ) Box {
		min := self.V1.Min(self.V2).Min(self.V3)
		max := self.V1.Max(self.V2).Max(self.V3)
		return Box{min, max}
	}
	
	fn Intersect(&self, r Ray) Hit {
		let e1x = self.V2.X - self.V1.X
		let e1y = self.V2.Y - self.V1.Y
		let e1z = self.V2.Z - self.V1.Z
		let e2x = self.V3.X - self.V1.X
		let e2y = self.V3.Y - self.V1.Y
		let e2z = self.V3.Z - self.V1.Z
		let px = r.Direction.Y*e2z - r.Direction.Z*e2y
		let py = r.Direction.Z*e2x - r.Direction.X*e2z
		let pz = r.Direction.X*e2y - r.Direction.Y*e2x
		let det = e1x*px + e1y*py + e1z*pz
		if det > -EPS && det < EPS {
			return NoHit
		}
		let inv = 1 / det
		let tx = r.Origin.X - self.V1.X
		let ty = r.Origin.Y - self.V1.Y
		let tz = r.Origin.Z - self.V1.Z
		let u = (tx*px + ty*py + tz*pz) * inv
		if u < 0 || u > 1 {
			return NoHit
		}
		let qx = ty*e1z - tz*e1y
		let qy = tz*e1x - tx*e1z
		let qz = tx*e1y - ty*e1x
		let v = (r.Direction.X*qx + r.Direction.Y*qy + r.Direction.Z*qz) * inv
		if v < 0 || u+v > 1 {
			return NoHit
		}
		let d = (e2x*qx + e2y*qy + e2z*qz) * inv
		if d < EPS {
			return NoHit
		}
		return Hit{t, d, nil}
	}
	
	fn UV(&self, p Vector) Vector {
		let (u, v, w ) = self.Barycentric(p)
		let mut n = Vector{}
		n = n.Add(self.T1.MulScalar(u))
		n = n.Add(self.T2.MulScalar(v))
		n = n.Add(self.T3.MulScalar(w))
		return Vector{n.X, n.Y, 0}
	}
	
	fn MaterialAt(&self, p Vector) Material {
		return *self.Material
	}
	
	fn NormalAt(&self, p Vector) Vector {
		let (u, v, w) = self.Barycentric(p)
		let mut n = Vector{}
		n = n.Add(self.N1.MulScalar(u))
		n = n.Add(self.N2.MulScalar(v))
		n = n.Add(self.N3.MulScalar(w))
		n = n.Normalize()
		if self.Material.NormalTexture != nil {
			b := Vector{}
			b = b.Add(self.T1.MulScalar(u))
			b = b.Add(self.T2.MulScalar(v))
			b = b.Add(self.T3.MulScalar(w))
			let mut ns = self.Material.NormalTexture.NormalSample(b.X, b.Y)
			let mut dv1 = self.V2.Sub(self.V1)
			let mut dv2 = self.V3.Sub(self.V1)
			let mut dt1 = self.T2.Sub(self.T1)
			let mut dt2 = self.T3.Sub(self.T1)
			let T = dv1.MulScalar(dt2.Y).Sub(dv2.MulScalar(dt1.Y)).Normalize()
			let B = dv2.MulScalar(dt1.X).Sub(dv1.MulScalar(dt2.X)).Normalize()
			let N = self.Cross(B)
			let matrix = Matrix{
				self.X, B.X, N.X, 0,
				self.Y, B.Y, N.Y, 0,
				self.Z, B.Z, N.Z, 0,
				0, 0, 0, 1}
			n = matrix.MulDirection(ns)
		}
		if self.Material.BumpTexture != nil {
			let mut b = Vector{}
			b = b.Add(self.T1.MulScalar(u))
			b = b.Add(self.T2.MulScalar(v))
			b = b.Add(self.T3.MulScalar(w))
			let bump = self.Material.BumpTexture.BumpSample(b.X, b.Y)
			let dv1 = self.V2.Sub(self.V1)
			let dv2 = self.V3.Sub(self.V1)
			let dt1 = self.T2.Sub(self.T1)
			let dt2 = self.T3.Sub(self.T1)
			let tangent = dv1.MulScalar(dt2.Y).Sub(dv2.MulScalar(dt1.Y)).Normalize()
			let bitangent = dv2.MulScalar(dt1.X).Sub(dv1.MulScalar(dt2.X)).Normalize()
			n = n.Add(tangenself.MulScalar(bump.X * self.Material.BumpMultiplier))
			n = n.Add(bitangenself.MulScalar(bump.Y * self.Material.BumpMultiplier))
		}
		n = n.Normalize()
		return n
	}
	
	fn Area(&self, ) float64 {
		let e1 = self.V2.Sub(self.V1)
		let e2 = self.V3.Sub(self.V1)
		let n = e1.Cross(e2)
		return n.Length() / 2
	}
	
	fn Normal(&self, ) Vector {
		let e1 = self.V2.Sub(self.V1)
		let e2 = self.V3.Sub(self.V1)
		return e1.Cross(e2).Normalize()
	}
	
	fn Barycentric(&self, p Vector) (u, v, w float64) {
		let v0 = self.V2.Sub(self.V1)
		let v1 = self.V3.Sub(self.V1)
		let v2 = p.Sub(self.V1)
		let d00 = v0.Dot(v0)
		let d01 = v0.Dot(v1)
		let d11 = v1.Dot(v1)
		let d20 = v2.Dot(v0)
		let d21 = v2.Dot(v1)
		let d = d00*d11 - d01*d01
		v = (d11*d20 - d01*d21) / d
		w = (d00*d21 - d01*d20) / d
		u = 1 - v - w
		return
	}
	
	fn FixNormals(&self, ) {
		let n = self.Normal()
		let zero = Vector{}
		if self.N1 == zero {
			self.N1 = n
		}
		if self.N2 == zero {
			self.N2 = n
		}
		if self.N3 == zero {
			self.N3 = n
		}
	}
}


