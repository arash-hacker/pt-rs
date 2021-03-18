use crate::{bbox::*, ray::Ray};
use crate::shape::*;
use crate::material::*;
use crate::sdf::*;
use crate::hit::*;
use crate::vector::*;
use crate::triangle::*;
use crate::tree::*;
use crate::axis::*;
use crate::material;
use crate::shape::*;


pub type Func =fn(x:f64, y:f64)->f64;

pub struct Function {
	pub Function:Func,
	pub Box:BBox,
	pub Material:Material,
}

pub fn NewFunction(function: Func, bx:BBox, material:Material)->Box<dyn Shape> {
	return Box::new(Function{Function:function,Box: bx,Material: material})
}

impl Function{
	pub fn  Contains(&self,v: Vector)-> bool {
		return v.Z < (self.Function)(v.X, v.Y)
	}
}
 impl Shape for Function{

	 fn GetType(&self)->&str{"function"}

	 fn  Compile(&self) {}
	
	 fn  BoundingBox(&self)-> BBox {
		return self.Box
	}
	
	 fn  Intersect(&self,ray:Ray)-> Hit {
		let step = 1.0 / 32.0;
		let sign = self.Contains(ray.Position(step));

		let t = step;
		while t < 12.0{
			let v = ray.Position(t);
			if self.Contains(v) != sign && self.Box.Contains(v) {
				return Hit{Shape:Some(Box::new(*self)), T:t - step, HitInfo:None};
			}
			t+=step
		}
		return NoHit
	}
	
	 fn  UV(&self,p:Vector) ->Vector {
		let (x1, x2) = (self.Box.Min.X, self.Box.Max.X);
		let (y1, y2) = (self.Box.Min.Y, self.Box.Max.Y);
		let u = (p.X - x1) / (x2 - x1);
		let v = (p.Y - y1) / (y2 - y1);
		return Vector{X:u, Y:v, Z:0.0}
	}
	
	 fn  MaterialAt(&self,p: Vector) -> Material {
		return self.Material
	}
	
	 fn  NormalAt(&self,p: Vector) ->Vector {
		let eps = 1e-3;
		let v = Vector{
			X:((*self).Function)(p.X-eps, p.Y) -( (*self).Function)(p.X+eps, p.Y),
			Y:((*self).Function)(p.X, p.Y-eps) -( (*self).Function)(p.X, p.Y+eps),
			Z:2.0 * eps,
		};
		return v.Normalize()
	}	
}
