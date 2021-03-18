use crate::shape::*;
use crate::vector::*;
use crate::ray::*;
use crate::material::*;

use crate::volume::*;
use crate::sdf::*;
use crate::shape::*;
use crate::ray::*;


pub struct Hit{
	pub Shape   :Option<Box<dyn Shape>>,
	pub T       :f64,
	pub HitInfo :Option<HitInfo>,
}

pub struct HitInfo{
	pub Shape   :Option<Box<dyn Shape>>,
	pub Position:Vector,
	pub Normal  :Vector,
	pub Ray     :Ray,
	pub Material:Material,
	pub Inside  :bool,
}

pub static NoHit:Hit = Hit{Shape:None ,T:f64::INFINITY, HitInfo: None};

impl Hit{
	pub fn Default()->Hit{
		return Hit{
			Shape   :None,
			T       :0.0,
			HitInfo :None,
		}
	}
	pub fn Ok(&self)-> bool {
		return self.T < f64::INFINITY
	}

	pub fn Info(&self, r:Ray)->HitInfo {
		

		if !self.HitInfo.is_none() {
			return self.HitInfo.unwrap();
		}
		let mut shape = self.Shape.unwrap();
		let mut position = r.Position(self.T);
		let mut normal = shape.NormalAt(position);
		let mut material = MaterialAt(shape, position);
		let mut inside = false;
		if normal.Dot(r.Direction) > 0.0 {
			normal = normal.Negate();
			inside = true;
			//TODO: get_type for this items
			match shape.GetType() {
			 "Volume" | "SDFShape"| "SphericalHarmonic" =>{inside = false;}
			}
		}
		let ray = Ray{Origin:position,Direction: normal};
		return HitInfo{
			Shape:Some(shape),
			Position: position,
			Normal: normal,
			Ray: ray,
			Material: material,
			Inside: inside
		}
	}
}

