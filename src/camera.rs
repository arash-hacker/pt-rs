
use crate::{bbox::*, ray::Ray};
use crate::shape::*;
use crate::material::*;
use crate::sdf::*;
use crate::hit::*;
use crate::vector::*;
use crate::triangle::*;
use crate::tree::*;
use crate::axis::*;

use crate::common::{self, *};
use crate::util::{self, *};

extern crate rand;
pub struct  Camera{
	pub p:Vector,
	pub u:Vector,
	pub v:Vector, 
	pub w:Vector,
	pub m:f64 ,
	pub focalDistance :f64,
	pub apertureRadius:f64
}

pub fn LookAt(eye:Vector, center:Vector, up:Vector, fovy: f64) -> Camera {
	let mut c = Camera::Default();
	c.p = eye;
	c.w = center.Sub(eye).Normalize();
	c.u = up.Cross(c.w).Normalize();
	c.v = c.w.Cross(c.u).Normalize();
	c.m = 1.0 / f64::tan(fovy*util::PI/360.0);
	return c
}

impl Camera{
	pub fn Default()->Camera{
		return Camera{
			p:Vector::Default(),
			u:Vector::Default(),
			v:Vector::Default(), 
			w:Vector::Default(),
			m:0.0 ,
			focalDistance :0.0,
			apertureRadius:0.0,
		}
	}
	pub fn SetFocus(&self,focalPoint: Vector, apertureRadius :f64) {
		self.focalDistance = focalPoint.Sub(self.p).Length();
		self.apertureRadius = apertureRadius;
	}
	
	pub fn CastRay(&self,x:i32, y:i32, w:i32, h:i32, u:f64, v:f64, rnd:f64) ->Ray {
		let aspect = w as f64 / h as f64;
		let px = ((x as f64+u-0.5)/(w as f64-1.0))*2.0 - 1.0;
		let py = ((y as f64+v-0.5)/(h as f64-1.0))*2.0 - 1.0;
		let mut  d = Vector::Default();
		d = d.Add(self.u.MulScalar(-px * aspect));
		d = d.Add(self.v.MulScalar(-py));
		d = d.Add(self.w.MulScalar(self.m));
		d = d.Normalize();
		let mut p = self.p;
		if self.apertureRadius > 0.0 {
			let focalPoint = self.p.Add(d.MulScalar(self.focalDistance));
			let angle = rand::random::<f64>() * 2.0 * util::PI;
			let radius = rand::random::<f64>() * self.apertureRadius;
			p = p.Add(self.u.MulScalar(f64::cos(angle) * radius));
			p = p.Add(self.v.MulScalar(f64::sin(angle) * radius));
			d = focalPoint.Sub(p).Normalize();
		}
		return Ray{Origin:p,Direction: d}
	}
}

