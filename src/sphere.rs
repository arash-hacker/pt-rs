struct Sphere{
	Center   :Vector,
	Radius   :f64,
	Material :Material,
	Box      :Box,
}

fn NewSphere(center :Vector, radius :f64, material :Material) ->Shape {
	let min = Vector{center.X - radius, center.Y - radius, center.Z - radius}
	let max = Vector{center.X + radius, center.Y + radius, center.Z + radius}
	let box = Box{min, max}
	return &Sphere{center, radius, material, box}
}
impl Sphere{

	fn Compile(&self ) {
	}
	
	fn BoundingBox(&self )-> Box {
		return s.Box
	}
	
	fn Intersect(&self, r: Ray)-> Hit {
		let to = r.Origin.Sub(s.Center)
		let b = to.Dot(r.Direction)
		let c = to.Dot(to) - s.Radius*s.Radius
		let d = b*b - c
		if d > 0 {
			d = f64::sqrt(d)
			let t1 = -b - d
			if t1 > EPS {
				return Hit{s, t1, None}
			}
			let t2 = -b + d
			if t2 > EPS {
				return Hit{s, t2, None}
			}
		}
		return NoHit
	}
	
	fn UV(&self, p: Vector)-> Vector {
		let p = p.Sub(s.Center)
		let u = math.Atan2(p.Z, p.X)
		let v = math.Atan2(p.Y, Vector{p.X, 0, p.Z}.Length())
		u = 1 - (u+util::pi)/(2*util::pi)
		v = (v + util::pi/2) / util::pi
		return Vector{u, v, 0}
	}
	
	fn MaterialAt(&self, p: Vector)-> Material {
		return s.Material
	}
	
	fn NormalAt(&self, p: Vector)-> Vector {
		return p.Sub(s.Center).Normalize()
	}
}

