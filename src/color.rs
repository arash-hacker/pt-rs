use crate::bbox::*;
use crate::shape::*;
use crate::material::*;
use crate::sdf::*;
use crate::hit::*;
use crate::vector::*;
use crate::triangle::*;
use crate::tree::*;
use crate::axis::*;
#[derive(Debug,Clone, Copy)]
pub struct Color {
	pub R:f64,
	pub G:f64,
	pub B :f64,
}
pub struct RGBA{
	pub R:f64,
	pub G:f64,
	pub B:f64,
	pub A:f64,
}
pub static  Black:Color= Color{R:0.0, G:0.0, B:0.0};
pub static  White:Color= Color{R:1.0, G:1.0, B:1.0};
// pub fn Black()->Color{
// 	Color{R:0.0, G:0.0, B:0.0}
// }
// pub fn White()->Color{
// 	Color{R:1.0, G:1.0, B:1.0}
// }
pub fn HexColor(x:i32)-> Color {
	let r = ((x>>16)&0xff) as f64 / 255.0;
	let g = ((x>>8)&0xff) as f64  / 255.0;
	let b = ((x>>0)&0xff) as f64  / 255.0;
	Color{R:r,G: g, B:b}.Pow(2.2)
}

pub fn NewColor(c:Color) ->Color {
	Color{R:((c.R as f64) / 65535.0),G: (c.G as f64) / 65535.0,B: (c.B as f64) / 65535.0}
}
impl Color{
	pub fn RGBA(&self) -> RGBA {
		let r = (f64::max(0.0, f64::min(255.0, self.R*255.0))) as f64;
		let g = (f64::max(0.0, f64::min(255.0, self.G*255.0))) as f64;
		let b = (f64::max(0.0, f64::min(255.0, self.B*255.0))) as f64;
		RGBA{R:r, G:g, B:b, A:255.0}
	}

	pub fn Add(&self,b:Color) ->Color {
		Color{
			R:self.R + b.R,
			G: self.G + b.G,
			B: self.B + b.B
		}
	}
	
	pub fn Sub(&self,b:Color)-> Color {
		Color{
			R:self.R - b.R,
			G:self.G - b.G,
			B:self.B - b.B
		}
	}
	
	pub fn Mul(&self,b :Color)-> Color {
		Color{
			R:self.R * b.R,
			G: self.G * b.G,
			B: self.B * b.B
		}
	}
	
	pub fn MulScalar(&self,b:f64)-> Color {
		Color{
			R:self.R * b,
			G:self.G * b,
			B:self.B * b}
	}
	
	pub fn DivScalar(&self,b :f64)-> Color {
		Color{
			R:self.R / b,
			G:self.G / b,
			B:self.B / b
		}
	}
	
	pub fn Min(&self,b:Color)-> Color {
		Color{
			R:f64::min(self.R, b.R),
			G:f64::min(self.G, b.G),
			B:f64::min(self.B, b.B)
		}
	}
	
	pub fn Max(&self,b:Color)->Color {
		Color{
			R:f64::max(self.R, b.R),
			G:f64::max(self.G, b.G),
			B:f64::max(self.B, b.B)
		}
	}
	
	pub fn MinComponent(&self)->f64 {
		f64::min(f64::min(self.R, self.G), self.B)
	}
	
	pub fn MaxComponent(&self)->f64 {
		f64::max(f64::max(self.R, self.G), self.B)
	}
	
	pub fn Pow(&self,b:f64)-> Color {
		Color{
			R:f64::powf(self.R, b),
			G:f64::powf(self.G, b),
			B:f64::powf(self.B, b),
		}
	}
	
	pub fn Mix(&mut self,b :Color, pct:f64) ->Color {
		*self = self.MulScalar(1.0 - pct);
		let b = b.MulScalar(pct);
		self.Add(b)
	}
	
}
