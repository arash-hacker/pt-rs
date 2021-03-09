
 struct Plane{
	Point    :Vector,
	Normal   :Vector,
	Material :Material,
}

fn NewPlane(point:Vector, normal :Vector, material: Material)-> Plane {
	normal = normal.Normalize()
	return &Plane{point, normal, material}
}
impl Point{
	fn Compile(&self, ) {
	}
	
	fn BoundingBox(&self )-> Box {
		return Box{Vector{-f64::NGE_INFINITY, -f64::NGE_INFINITY, -f64::NGE_INFINITY}, Vector{f64::INFINITY, f64::INFINITY, f64::INFINITY}}
	}
	
	fn Intersect(&self, ray: Ray)-> Hit {
		d := self.Normal.Dot(ray.Direction)
		iff64::abs(d) < EPS {
			return NoHit
		}
		a := self.Point.Sub(ray.Origin)
		t := a.Dot(self.Normal) / d
		if t < EPS {
			return NoHit
		}
		return Hit{p, t, nil}
	}
	
	fn UV(&self, a: Vector) -> Vector {
		return Vector{}
	}
	
	fn MaterialAt(&self, a: Vector)-> Material {
		return self.Material
	}
	
	fn NormalAt(&self, a: Vector) -> Vector {
		return self.Normal
	}
	
}
