use crate::{bbox::*, common, ray::Ray};
use crate::shape::*;
use crate::material::*;
use crate::matrix::*;
use crate::sdf::*;
use crate::hit::*;
use crate::vector::*;
use crate::triangle::*;
use crate::tree::*;
use crate::axis::*;
pub struct Cylinder {
	pub Radius:f64,
	pub Z0 :f64, 
	pub Z1   :f64,
	pub Material:Material
}

pub fn NewCylinder(radius:f64, z0:f64, z1:f64, material:Material)->Cylinder {
	return Cylinder{Radius:radius, Z0: z0, Z1:z1, Material:material}
}

pub fn NewTransformedCylinder(v0:Vector, v1: Vector, radius :f64, material: Material)-> Box<dyn Shape> {
	let mut up = Vector{X:0.0, Y:0.0, Z:1.0};
	let mut d  = v1.Sub(v0);
	let mut z  = d.Length();
	let mut a  = f64::acos(d.Normalize().Dot(up));
	let mut m  = Translate(v0);
	if a != 0.0 {
		let u = d.Cross(up).Normalize();
		m = Rotate(u, a).Translate(v0);
	}
	let c = NewCylinder(radius, 0.0, z, material);
	return NewTransformedShape(Box::new(c), m);
}
impl Shape for Cylinder{

	fn GetType(&self)->&str{"Cylinder"}

	fn Compile(&self) {}

	fn BoundingBox(&self) -> BBox {
		let r = self.Radius;
		return BBox{Min:Vector{X:-r,Y: -r, Z:self.Z0},Max: Vector{X:r, Y:r, Z:self.Z1}}
	}
	
	fn Intersect(&self,ray :Ray)-> Hit {
		let r = self.Radius;
		let o = ray.Origin;
		let d = ray.Direction;
		let a = d.X*d.X + d.Y*d.Y;
		let b = 2.0*o.X*d.X + 2.0*o.Y*d.Y;
		let c = o.X*o.X + o.Y*o.Y - r*r;
		let q = b*b - 4.0*a*c;
		if q < common::EPS {
			return NoHit
		}
		let s = f64::sqrt(q);
		let t0 = (-b + s) / (2.0 * a);
		let t1 = (-b - s) / (2.0 * a);
		if t0 > t1 {
			let (t0, t1) = (t1, t0);
		}
		let z0 = o.Z + t0*d.Z;
		let z1 = o.Z + t1*d.Z;
		if t0 > common::EPS && self.Z0 < z0 && z0 < self.Z1 {
			return Hit{Shape:Some(Box::new(*self)), T:t0, HitInfo:None}
		}
		if t1 > common::EPS && self.Z0 < z1 && z1 < self.Z1 {
			return Hit{Shape:Some(Box::new(*self)),T: t1,HitInfo: None}
		}
		return NoHit
	
	}
	
	fn UV(&self,p:Vector)-> Vector {
		return Vector::Default()
	}
	
	fn MaterialAt(&self,p:Vector)-> Material {
		return self.Material
	}
	
	fn NormalAt(&self,p :Vector) ->Vector {
		let z=p;
		z.Z = 0.0;
		return z.Normalize()
	}
	
}
