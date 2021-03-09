use crate::shape::*;
use crate::vector::*;
use crate::ray::*;
use crate::material::*;
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

pub static NoHit:Hit = Hit{Shape:None,T:f64::INFINITY ,HitInfo: None};

impl Hit{
	pub fn Ok(&self)-> bool {
		return self.T < f64::INFINITY
	}

	pub fn Info(&self, r:Ray)->HitInfo {
		if self.HitInfo == None {
			return *self.HitInfo
		}
		let mut shape = self.Shape;
		let mut position = r.Position(self.T);
		let mut normal = shape.NormalAt(position);
		let mut material = MaterialAt(shape, position);
		let mut inside = false;
		if normal.Dot(r.Direction) > 0 {
			normal = normal.Negate();
			inside = true;
			//TODO: get_type for this items
			match shape.GetType() {
			 Volume | SDFShape| SphericalHarmonic =>{inside = false;}
			}
		}
		let ray = Ray{Origin:position,Direction: normal};
		return HitInfo{Shape:shape,Position: position,Normal: normal,Ray: ray,Material: material,Inside: inside}
	}
}

