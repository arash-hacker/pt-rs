struct Hit{
	Shape   :Shape,
	T       :f64,
	HitInfo :HitInfo,
}

struct HitInfo{
	Shape   :Shape,
	Position:Vector,
	Normal  :Vector,
	Ray     :Ray,
	Material:Material,
	Inside  :bool,
}

let NoHit = Hit{None,f64::INFINITY , None}

impl Hit{
	fn Ok(&self)-> bool {
		return self.T < INF
	}
	fn Info(&self, r Ray)->HitInfo {
		if hit.HitInfo != None {
			return *hit.HitInfo
		}
		let mut shape = hit.Shape;
		let mut position = r.Position(hit.T);
		let mut normal = shape.NormalAt(position);
		let mut material = MaterialAt(shape, position);
		let mut inside = false;
		if normal.Dot(r.Direction) > 0 {
			normal = normal.Negate()
			inside = true
			switch shape.(type) {
			case *Volume, *SDFShape, *SphericalHarmonic:
				inside = false
			}
		}
		let ray = Ray{position, normal}
		return HitInfo{shape, position, normal, ray, material, inside}
	}
}

