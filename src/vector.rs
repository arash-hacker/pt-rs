use std::hash::{Hash, Hasher};
use std::cmp::Eq;
use crate::shape::*;
use crate::material::*;
use crate::sdf::*;
use crate::hit::*;
use crate::bbox::*;
use crate::triangle::*;
use crate::tree::*;
extern crate rand;

#[derive(Debug,Clone, Copy,PartialEq)]
pub struct Vector {
	pub X:f64,
	pub Y:f64,
	pub Z:f64,
}

impl Hash for Vector {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (self.X as i32).hash(state);
        (self.Y as i32).hash(state);
        (self.Z as i32).hash(state);
    }
}
impl Eq for Vector {
    
}


pub fn V(x:f64, y:f64, z:f64 )-> Vector {
	return Vector{X:x, Y:y, Z:z}
}

pub fn RandomUnitVector()-> Vector {
	loop {

		let x = rand::random::<f64>()*2.0 - 1.0;
		let y = rand::random::<f64>()*2.0 - 1.0;
		let z = rand::random::<f64>()*2.0 - 1.0;

		if x*x+y*y+z*z > 1.0 {
			continue
		}
		return Vector{X:x, Y:y, Z:z}.Normalize();
	}
}
impl Vector{
	pub fn Default()->Vector{
		Vector{
			X:0.0,
			Y:0.0,
			Z:0.0,
		}
	}
	pub fn Length(&self) ->f64 {
		f64::sqrt(self.X*self.X + self.Y*self.Y + self.Z*self.Z)
	}
	
	pub fn LengthN(&mut self,n:f64)->f64 {
		if n == 2.0 {
			return self.Length();
		}
		*self = self.Abs();
		f64::powf(
		f64::powf(self.X, n)+
			f64::powf(self.Y, n)+
			f64::powf(self.Z, n),1.0/n) 
	}
	
	pub fn Dot(&self,b:Vector)-> f64 {
		self.X*b.X + self.Y*b.Y + self.Z*b.Z
	}
	
	pub fn Cross(&self, b:Vector)-> Vector {
		let x = self.Y*b.Z - self.Z*b.Y;
		let y = self.Z*b.X - self.X*b.Z;
		let z = self.X*b.Y - self.Y*b.X;
		Vector{X:x, Y:y, Z:z}
	}
	
	pub fn Normalize(&self) -> Vector {
		let d = self.Length();
		Vector{
			X:self.X / d,
			Y: self.Y / d,
			Z: self.Z / d
		}
	}
	
	pub fn Negate(&self) -> Vector {
		Vector {
			X:-self.X,
			Y: -self.Y,
			Z: -self.Z,
		}
	}
	
	pub fn Abs(&self)-> Vector {
		Vector{ 
			X:f64::abs(self.X),
			Y:f64::abs(self.Y),
			Z:f64::abs(self.Z),
		}
	}
	
	pub fn Add(&self,b:Vector) ->Vector {
		Vector{
			X:self.X + b.X,
			Y: self.Y + b.Y,
			Z: self.Z + b.Z
		}
	}
	
	pub fn Sub(&self,b:Vector)-> Vector {
		Vector{
			X:self.X - b.X,
			Y: self.Y - b.Y,
			Z: self.Z - b.Z
		}
	}
	
	pub fn Mul(&self,b :Vector) -> Vector {
		Vector{
			X:self.X * b.X,
			Y:self.Y * b.Y,
			Z:self.Z * b.Z
		}
	}
	
	pub fn Div(&self,b :Vector) -> Vector {
		Vector{
			X:self.X / b.X, 
			Y:self.Y / b.Y, 
			Z:self.Z / b.Z
		}
	}
	
	pub fn Mod(&self,b:Vector)-> Vector {
		// as implemented in GLSL
		
		let x = self.X - b.X*f64::floor(self.X/b.X);
		let y = self.Y - b.Y*f64::floor(self.Y/b.Y);
		let z = self.Z - b.Z*f64::floor(self.Z/b.Z);
		Vector{X:x,Y:y,Z:z}
	}
	
	pub fn AddScalar(&self,b:f64)-> Vector {
		 Vector{
			X:self.X + b,
			Y: self.Y + b,
			Z:  self.Z + b,
			}
	}
	
	pub fn SubScalar(&self,b:f64) -> Vector {
		 Vector{
			X:self.X - b,
			Y:self.Y - b,
			Z:self.Z - b
		}
	}
	
	pub fn MulScalar(&self,b:f64) -> Vector {
		 Vector{
			X:self.X * b,
			Y:self.Y * b,
			Z:self.Z * b
		}
	}
	
	pub fn DivScalar(&self,b:f64)-> Vector {
		Vector{
			X:self.X / b,
			Y: self.Y / b,
			Z: self.Z / b
		}
	}
	
	pub fn Min(&self,b:Vector) ->Vector {
		Vector{
			X:f64::min(self.X, b.X), 
			Y:f64::min(self.Y, b.Y), 
			Z:f64::min(self.Z, b.Z),
		}
	}
	
	pub fn Max(&self,b:Vector) ->Vector {
		Vector{
			X:f64::max(self.X, b.X),
			Y:f64::max(self.Y, b.Y), 
			Z:f64::max(self.Z, b.Z),
		}
	}
	
	pub fn MinAxis(&self) -> Vector {
		let (x, y, z) =(
			f64::abs(self.X),
			f64::abs(self.Y),
			f64::abs(self.Z)
		);
		if x <= y && x <= z { return Vector{X:1.0, Y:0.0, Z:0.0} }
		if y <= x && y <= z { return Vector{X:0.0, Y:1.0, Z:0.0} }
		return Vector{X:0.0, Y:0.0, Z:1.0}
	}
	
	pub fn MinComponent(&self) ->f64 {
		f64::min(f64::min(self.X, self.Y), self.Z)
	}
	
	pub fn MaxComponent(&self) ->f64 {
		f64::max(f64::max(self.X, self.Y), self.Z)
	}
	
	pub fn Reflect(&self,i :Vector)-> Vector {
		i.Sub(self.MulScalar(2.0 * self.Dot(i)))
	}
	
	pub fn Refract(&self, i: Vector, n1:f64, n2:f64)-> Vector {
		let nr = n1 / n2;
		let cosI = -self.Dot(i);
		let sinT2 = nr * nr * (1.0 - cosI*cosI);
		if sinT2 > 1.0 {
			return Vector{X:0.0,Y:0.0,Z:0.0}
		}
		let cosT = f64::sqrt(1.0 - sinT2);
		i.MulScalar(nr).Add(self.MulScalar(nr*cosI - cosT))
	}
	
	pub fn Reflectance(&self,i:Vector, n1:f64, n2:f64)->f64 {
		let nr = n1 / n2;
		let cosI = -self.Dot(i);
		let sinT2 = nr * nr * (1.0 - cosI*cosI);
		if sinT2 > 1.0 {
			return 1.0;
		}
		let cosT = f64::sqrt(1.0 - sinT2);
		let rOrth = (n1*cosI - n2*cosT) / (n1*cosI + n2*cosT);
		let rPar = (n2*cosI - n1*cosT) / (n2*cosI + n1*cosT);
		return (rOrth*rOrth + rPar*rPar) / 2.0
	}	
}
