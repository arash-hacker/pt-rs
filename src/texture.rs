use crate::color::{self,Color};
use crate::vector::Vector;
use crate::util;
use crate::common;
use std::{collections::HashMap};
use image::*;
trait Texture{
	fn Sample(u:f64, v:f64 )->Color;
	fn NormalSample(u:f64, v :f64)-> Vector ;
	fn BumpSample(u:f64, v :f64)-> Vector ;
	fn MulScalar(a:f64)->  Texture;
	fn Pow(a:f64) -> Texture;
}


pub fn init() {}

fn GetTexture(path:String)->  Texture {
	let textures:HashMap<String,Texture>= HashMap::new();
	if textures.contains_key(&path){
		return textures.get(&path);
	}
	if LoadTexture(path) as bool {
		textures.insert(path, LoadTexture(path));
		return textures.get(path)
	}
	return None
}

fn LoadTexture(path:String) ->Texture {
	println!("Loading IMG: {}\n", path);
	let im = util::LoadImage(path).unwrap();
	return NewTexture(im)
}

pub struct ColorTexture{
	Width  :i32,
	Height :i32,
	Data   :Vec<Color>,
}

pub fn NewTexture(im:image::DynamicImage) -> Texture {
	//TODO: better comparision spread like js
	let size:i32 = i32::max( im.dimensions().0 as i32,im.dimensions().1 as i32);
	let mut data = vec![color::Black(); (size*size) as usize];
	for y in 0..size {
		for x in 0..size {
			let index = y*size + x;
			data[index as usize] = color::NewColor(im.get_pixel(x as u32, y as u32)).Pow(2.2);
		}
	}
	return ColorTexture{
		Width:size,
		Height:size,
		Data: data
	}
}

impl ColorTexture{

	pub fn Pow(&mut self,a :f64)-> Texture {
		for (i,_)  in self.Data.iter().enumerate() {
			self.Data[i] = self.Data[i].Pow(a)
		}
		return self
	}
	
	pub fn MulScalar(&self,a:f64) ->Texture {
		for (i,_) in self.Data.iter().enumerate() {
			self.Data[i] = self.Data[i].MulScalar(a)
		}
		return self
	}
	
	pub fn bilinearSample(&self, u:f64, v:f64)-> Color {
		if u == 1.0 {
			u -= common::EPS as f64;
		}
		if v == 1.0 {
			v -= common::EPS as f64;
		}
		let w = (self.Width as f64) - 1.0;
		let h = (self.Height as f64) - 1.0;
		let (X, x) =util::Modf(u * w);
		let (Y, y) =util::Modf(v * h);
		let x0 = X as i32;
		let y0 = Y as i32;
		let x1 = x0 + 1;
		let y1 = y0 + 1;
		let c00 = self.Data[(y0*self.Width+x0) as usize];
		let c01 = self.Data[(y1*self.Width+x0) as usize];
		let c10 = self.Data[(y0*self.Width+x1) as usize];
		let c11 = self.Data[(y1*self.Width+x1) as usize];
		let mut c = color::Black();
		c = c.Add(c00.MulScalar((1.0 - x) * (1.0 - y)));
		c = c.Add(c10.MulScalar(x * (1.0 - y)));
		c = c.Add(c01.MulScalar((1.0 - x) * y));
		c = c.Add(c11.MulScalar(x * y));
		return c
	}
	
	pub fn Sample(&self, u:f64, v:f64)-> Color {
		u = util::Fract(util::Fract(u) + 1.0);
		v = util::Fract(util::Fract(v) + 1.0);
		return self.bilinearSample(u, 1.0-v)
	}
	
	pub fn NormalSample(&self, u:f64, v:f64) -> Vector {
		let c = self.Sample(u, v);
		return Vector{X:c.R*2.0 - 1.0,Y: c.G*2.0 - 1.0,Z: c.B*2.0 - 1.0}.Normalize()
	}
	
	pub fn BumpSample(&self,u:f64, v:f64) -> Vector {
		let u = util::Fract(util::Fract(u) + 1.0);
		let v = util::Fract(util::Fract(v) + 1.0);
		let v = 1.0 - v;
		let x = (u * (self.Width) as f64) as f64;
		let y = (v * (self.Height) as f64) as f64;
		let (x1, x2 )= (util::Clamp(x-1.0, 0.0, self.Width as f64 -1.0) , util::Clamp(x+1.0, 0.0, self.Width as f64-1.0));
		let (y1, y2 )= (util::Clamp(y-1.0, 0.0, self.Height as f64-1.0), util::Clamp(y+1.0, 0.0, self.Height as f64-1.0));
		let cx = self.Data[(y*self.Width as f64 +x1) as usize].Sub(self.Data[(y*self.Width as f64+x2) as usize]);
		let cy = self.Data[(y1*self.Width as f64+x) as usize].Sub(self.Data[(y2*self.Width as f64+x) as usize]);
		return Vector{X:cx.R,  Y: cy.R, Z: 0.0}
	}
	
}
