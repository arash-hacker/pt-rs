use crate::shape::*;
use crate::material::*;
use crate::hit::*;
use crate::vector::*;
use crate::bbox::*;
use crate::sdf::*;
use crate::ray::*;
use crate::matrix::*;
use crate::util::{self, *};

#[derive(Copy,Debug,Default,Clone)]
pub struct Sphere{
	pub Center   :Vector,
	pub Radius   :f64,
	pub Material :Material,
	pub Box      :BBox,
}

impl Sphere {
    
}

pub fn NewSphere(center :Vector, radius :f64, material :Material) ->Box<dyn Shape> {
	let min = Vector{X:center.X - radius,Y: center.Y - radius,Z: center.Z - radius};
	let max = Vector{X:center.X + radius,Y: center.Y + radius,Z: center.Z + radius};
	let bx = Box{Min:min, Max:max};
	return Sphere{center, radius, material, bx}
}
impl Shape for Sphere{

	fn GetType(&self)->&str{"Sphere"}

	fn Compile(&self ) {}
	
	fn BoundingBox(&self )-> BBox {
		return self.Box
	}
	
	fn Intersect(&self, r: Ray)-> Hit {
		let to = r.Origin.Sub(self.Center);
		let b = to.Dot(r.Direction);
		let c = to.Dot(to) - self.Radius*self.Radius;
		let d = b*b - c;
		if d > 0 {
			d = f64::sqrt(d);
			let t1 = -b - d;
			if t1 > EPS {
				return Hit{Shape:Box::new(*self), T:t1,HitInfo: None}
			}
			let t2 = -b + d;
			if t2 > EPS {
				return Hit{Shape:Box::new(*self), T:t2, HitInfo:None}
			}
		}
		return NoHit
	}
	
	fn UV(&self, p: Vector)-> Vector {
		let p = p.Sub(self.Center);
		let u = f64::atan2(p.Z, p.X);
		let v = f64::atan2(p.Y, Vector{p.X, 0, p.Z}.Length());
		u = 1 - (u+util::PI)/(2*util::PI);
		v = (v + util::PI/2) / util::PI;
		return Vector{u, v, 0}
	}
	
	fn MaterialAt(&self, p: Vector)-> Material {
		return self.Material
	}
	
	fn NormalAt(&self, p: Vector)-> Vector {
		return p.Sub(self.Center).Normalize()
	}
}

