
use crate::{bbox::*, ray::Ray, util};
use crate::shape::*;
use crate::material::*;
use crate::sdf::*;
use crate::hit::*;
use crate::vector::*;
use crate::triangle::*;
use crate::tree::*;
use crate::axis::*;
use crate::common::{self, *};
pub struct Plane{
	pub Point    :Vector,
	pub Normal   :Vector,
	pub Material :Material,
}

pub fn NewPlane(point:Vector, normal :Vector, material: Material)-> Plane {
	let normal = normal.Normalize();
	return Plane{Point:point,Normal: normal,Material: material}
}
impl Shape for Plane{
	 fn GetType(&self) ->&str {"Plane"}
	 
	 fn Compile(&self) {
	}
	
	 fn BoundingBox(&self)-> BBox {
		return BBox{
			Min:Vector{X:f64::NEG_INFINITY,Y: f64::NEG_INFINITY, Z:f64::NEG_INFINITY},
			Max: Vector{X:f64::INFINITY, Y:f64::INFINITY, Z:f64::INFINITY}
		}
	}
	
	 fn Intersect(&self, ray: Ray)-> Hit {
		let d = self.Normal.Dot(ray.Direction);
		if f64::abs(d) < common::EPS {
			return NoHit
		}
		let a = self.Point.Sub(ray.Origin);
		let t = a.Dot(self.Normal) / d;
		if t < common::EPS {
			return NoHit
		}
		return Hit{Shape:Some(Box::new(*self)), T:t, HitInfo:None}
	}
	
	 fn UV(&self, a: Vector) -> Vector {
		return Vector::Default()
	}
	
	 fn MaterialAt(&self, a: Vector)-> Material {
		return self.Material
	}
	
	 fn NormalAt(&self, a: Vector) -> Vector {
		return self.Normal
	}
	
}
