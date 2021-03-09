struct Cylinder {
	Radius:f64,
	Z0 :f64, 
	Z1   :f64,
	Material:Material
}

fn NewCylinder(radius, z0, z1 float64, material Material)->Cylinder {
	return Cylinder{radius, z0, z1, material}
}

fn NewTransformedCylinder(v0:Vector, v1: Vector, radius :f64, material: Material)-> Shape {
	let mut up = Vector{0, 0, 1};
	let mut d  = v1.Sub(v0);
	let mut z  = d.Length();
	let mut a  = math.Acos(d.Normalize().Dot(up));
	let mut m  = Translate(v0);
	if a != 0 {
		u := d.Cross(up).Normalize()
		m = Rotate(u, a).Translate(v0)
	}
	let c = NewCylinder(radius, 0, z, material)
	return NewTransformedShape(c, m)
}
impl Cylinder{
	fn Compile(&self) {}

	fn BoundingBox(&self) -> Box {
		r := self.Radius
		return Box{Vector{-r, -r, self.Z0}, Vector{r, r, self.Z1}}
	}
	
	fn Intersect(&self,ray :Ray)-> Hit {
		let r = self.Radius
		let o = ray.Origin
		let d = ray.Direction
		let a = d.X*d.X + d.Y*d.Y
		let b = 2*o.X*d.X + 2*o.Y*d.Y
		let c = o.X*o.X + o.Y*o.Y - r*r
		let q = b*b - 4*a*c
		if q < EPS {
			return NoHit
		}
		let s = f64::sqrt(q)
		let t0 = (-b + s) / (2 * a)
		let t1 = (-b - s) / (2 * a)
		if t0 > t1 {
			t0, t1 = t1, t0
		}
		let z0 = o.Z + t0*d.Z
		let z1 = o.Z + t1*d.Z
		if t0 > EPS && self.Z0 < z0 && z0 < self.Z1 {
			return Hit{shape, t0, None}
		}
		if t1 > EPS && self.Z0 < z1 && z1 < self.Z1 {
			return Hit{shape, t1, None}
		}
		return NoHit
	
	}
	
	fn UV(&self,p:Vector)-> Vector {
		return Vector{}
	}
	
	fn MaterialAt(&self,p:Vector)-> Material {
		return self.Material
	}
	
	fn NormalAt(&self,p :Vector) ->Vector {
		p.Z = 0
		return p.Normalize()
	}
	
}
