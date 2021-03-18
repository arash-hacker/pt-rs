
use image::{Pixel, Rgba};

use crate::shape::*;
use crate::material::*;
use crate::sdf::*;
use crate::hit::*;
use crate::vector::*;
use crate::bbox::*;
use crate::triangle::*;
use crate::tree::*;
use crate::ray::*;

pub struct Volume{
	pub W:i32, 
	pub H:i32,
	pub D :i32,
	pub ZScale  :f64,
	pub Data    :Vec<f64>,
	pub Windows :Vec<VolumeWindow>,
	pub Box     :BBox,
}

pub struct VolumeWindow{
	pub Lo:f64, 
	pub Hi:f64,
	pub Material :Material
}

pub fn NewVolume(bx :BBox, images: Vec<image::ImageBuffer<image::Rgb<u8>, Vec<u8>>>, sliceSpacing :f64, windows: Vec<VolumeWindow>) ->Volume {
	let w = images[0].width();
	let h = images[0].height();
	let d = images.len() as u32;
	// TODO: w/h aspect ratio
	let zs = (sliceSpacing * (d as f64)) / (w as f64);
	let data = vec![0.0;( w*h*d) as usize];
	for (z, im) in images.iter().enumerate() {
		for y in 0..h {
			for x in 0..w {
				let r = im.get_pixel_mut(x, y).to_rgb();
				let f = (r.0[0]) as f64 / 65535.0;
				data[(x+y*w + (z as u32)*w*h )as usize] = f;
			}
		}
	}
	return Volume{
		W:w as i32, 
		H:h as i32, 
		D:d as i32,
		ZScale:zs,
		Data:data,
		Windows: windows,
		Box:bx
	}
}

impl Volume{

	pub fn Get(&self, x:i32, y:i32, z: i32) ->f64 {
		if x < 0 || y < 0 || z < 0 || x >= self.W || y >= self.H || z >= self.D {
			return 0.0
		}
		return self.Data[(x+y*self.W+z*self.W*self.H) as usize]
	}
	
	pub fn Sample(&self, x:f64, y:f64, z :f64) ->f64 { 
		z /= self.ZScale;
		x = ((x + 1.0) / 2.0) * (self.W) as f64;
		y = ((y + 1.0) / 2.0) * (self.H) as f64;
		z = ((z + 1.0) / 2.0) * (self.D) as f64;
		let x0 = x as i32;
		let y0 = y as i32;
		let z0 = z as i32;
		let x1 = x0 + 1;
		let y1 = y0 + 1;
		let z1 = z0 + 1;
		let v000 = self.Get(x0, y0, z0);
		let v001 = self.Get(x0, y0, z1);
		let v010 = self.Get(x0, y1, z0);
		let v011 = self.Get(x0, y1, z1);
		let v100 = self.Get(x1, y0, z0);
		let v101 = self.Get(x1, y0, z1);
		let v110 = self.Get(x1, y1, z0);
		let v111 = self.Get(x1, y1, z1);
		x -= (x0) as f64;
		y -= (y0) as f64;
		z -= (z0) as f64;
		let c00 = v000*(1.0-x) + v100*x;
		let c01 = v001*(1.0-x) + v101*x;
		let c10 = v010*(1.0-x) + v110*x;
		let c11 = v011*(1.0-x) + v111*x;
		let c0 = c00*(1.0-y) + c10*y;
		let c1 = c01*(1.0-y) + c11*y;
		let c = c0*(1.0-z) + c1*z;
		return c
	}
	
	
	pub fn Sign(&self, a: Vector) -> i32 {
		let s = self.Sample(a.X, a.Y, a.Z);
		for (i, window) in self.Windows.iter().enumerate() {
			if s < window.Lo {
				return (i + 1) as i32
			}
			if s > window.Hi {
				continue
			}
			return 0
		}
		return (self.Windows.len() as i32) + 1;
	}
	
}


impl Shape for Volume{

	fn GetType(&self) ->&str {"Volume"}
	
	fn BoundingBox(&self ) -> BBox {
		return self.Box
	}

	fn Compile(&self) {}
	
	fn Intersect(&self, ray: Ray)-> Hit {
		let (tmin, tmax) = self.Box.Intersect(ray);
		let step = 1.0 / 512.0;
		let start = f64::max(step, tmin);
		let sign = -1;
		for t in ((start as usize)..(tmax as usize)+1).step_by(step as usize) {
			let p = ray.Position(t as f64);
			let s = self.Sign(p);
			if s == 0 || (sign >= 0 && s != sign) {
				let mut t = t as f64- step;
				step /= 64.0;
				t += step;
				for i in 0..64 {
					if self.Sign(ray.Position(t)) == 0 {
						return Hit{Shape:Some(Box::new(*self)), T:t - step,HitInfo: None}
					}
					t += step;
				}
			}
			sign = s;
		}
		return NoHit
	}
	
	fn UV(&self, p:Vector) ->Vector {
		return Vector::Default() // not implemented
	}
	
	fn MaterialAt(&self, p:Vector)-> Material {
		let be = 1e9;
		let bm = Material::Default();
		let s = self.Sample(p.X, p.Y, p.Z);
		for window in self.Windows {
			if s >= window.Lo && s <= window.Hi {
				return window.Material;
			}
			let e = f64::min(f64::abs(s-window.Lo),f64::abs(s-window.Hi));
			if e < be {
				be = e;
				bm = window.Material;
			}
		}
		return bm
	}
	
	fn NormalAt(&self, p:Vector) ->Vector {
		let eps = 0.001;
		let n = Vector{
			X:self.Sample(p.X-eps, p.Y, p.Z) - self.Sample(p.X+eps, p.Y, p.Z),
			Y:self.Sample(p.X, p.Y-eps, p.Z) - self.Sample(p.X, p.Y+eps, p.Z),
			Z:self.Sample(p.X, p.Y, p.Z-eps) - self.Sample(p.X, p.Y, p.Z+eps),
		};
		return n.Normalize()
	}
	
}
