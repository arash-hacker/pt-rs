use crate::{color::*, volume::Volume};
use crate::scene::*;
use crate::{sphere::*};
use crate::material::*;
use crate::sdf::*;
use crate::ray::*;
use crate::hit::*;
use crate::vector::*;
use crate::bbox::*;
use crate::triangle::*;
use crate::tree::*;
use crate::shape::*;
use crate::util::{self, *};

extern crate rand;
type LightMode =i32;

#[derive(PartialEq)]
pub enum LightModeEnum {
	LightModeRandom = 0,
	LightModeAll=1,
}

pub type SpecularMode =i32;

pub static SpecularModeNaive:i32 = 0;
pub static SpecularModeFirst:i32 = 1;
pub	static SpecularModeAll:i32 = 2;

pub type BounceType =i32;

pub const BounceTypeAny:i32 = 0;
pub const BounceTypeDiffuse:i32 =1;
pub const BounceTypeSpecular:i32 =2;

pub trait Sampler {
	fn Sample(&self,scene :Scene, ray: Ray, rnd:f64)-> Color;
}

pub fn NewSampler(firstHitSamples:i32, maxBounces :i32) -> DefaultSampler {
	return DefaultSampler{
		FirstHitSamples:firstHitSamples,
		MaxBounces:maxBounces,
		DirectLighting:true,
		SoftShadows:true, 
		LightMode:LightModeEnum::LightModeRandom as i32, 
		SpecularMode:SpecularModeNaive
	}
}

pub fn NewDirectSampler() -> DefaultSampler {
	return DefaultSampler{
		FirstHitSamples:1, 
		MaxBounces:0, 
		DirectLighting:true, 
		SoftShadows:false, 
		LightMode:LightModeEnum::LightModeAll as i32, 
		SpecularMode:SpecularModeAll
	}
}

pub struct DefaultSampler {
	pub FirstHitSamples :i32,
	pub MaxBounces      :i32,
	pub DirectLighting  :bool,
	pub SoftShadows     :bool,
	pub LightMode       :LightMode,
	pub SpecularMode    :SpecularMode,
}
impl Sampler for DefaultSampler{
	fn Sample(&self, scene: Scene, ray :Ray, rnd:f64) -> Color {
		return self.sample(scene, ray, true, self.FirstHitSamples, 0, rnd)
	}
}
impl DefaultSampler{
	
	
	pub fn sample(&self, scene: Scene, ray :Ray, emission: bool, samples:i32, depth: i32, rnd:f64)-> Color {
		if depth > self.MaxBounces {
			return Black
		}
		let hit = scene.Intersect(ray);
		if !hit.Ok() {
			return self.sampleEnvironment(scene, ray);
		}
		let info = hit.Info(ray);
		let material = info.Material;
		let mut result = Black;
		if material.Emittance.unwrap() > 0.0 {
			if self.DirectLighting && !emission {
				return Black
			}
			result = result.Add(material.Color.unwrap().MulScalar(material.Emittance.unwrap() * (samples as f64)))
		}
		let n = (f64::sqrt((samples) as f64)) as i32;
		let mut ma:BounceType ;
		let mut mb :BounceType;
		if self.SpecularMode == SpecularModeAll || (depth == 0 && self.SpecularMode == SpecularModeFirst) {
			ma = BounceTypeDiffuse;
			mb = BounceTypeSpecular;
		} else {
			ma = BounceTypeAny;
			mb = BounceTypeAny;
		}
		for u in 0..n {
			for v in 0..n {
				for mode in ma..mb+1 {
					let fu = (u as f64 + rand::random::<f64>()) / (n as f64);
					let fv = (v as f64 + rand::random::<f64>()) / (n as f64);
					let (newRay, reflected, p) = ray.Bounce(info, fu, fv, mode, rnd);
					if mode == BounceTypeAny {
						p = 1.0;
					}
					if p > 0.0 && reflected {
						// specular
						let indirect = self.sample(scene, newRay, reflected, 1, depth+1, rnd);
						let tinted = indirect.Mix(material.Color.unwrap().Mul(indirect), material.Tint.unwrap());
						result = result.Add(tinted.MulScalar(p))
					}
					if p > 0.0 && !reflected {
						// diffuse
						let indirect = self.sample(scene, newRay, reflected, 1, depth+1, rnd);
						let direct = Black;
						if self.DirectLighting {
							direct = self.sampleLights(scene, info.Ray, rnd);
						}
						result = result.Add(material.Color.unwrap().Mul(direct.Add(indirect)).MulScalar(p))
					}
				}
			}
		}
		return result.DivScalar((n * n) as f64)
	}
	
	pub fn sampleEnvironment(&self, scene: Scene, ray:Ray) -> Color {
		if !scene.Texture.is_none() {
			let mut d = ray.Direction;
			let mut u = f64::atan2(d.Z, d.X) + scene.TextureAngle;
			let mut v = f64::atan2(d.Y, Vector{X:d.X,Y: 0.0,Z: d.Z}.Length());
			u = (u + util::PI) / (2.0 * util::PI);
			v = (v + util::PI/2.0) / util::PI;
			return scene.Texture.unwrap().Sample(u, v)
		}
		return scene.Color
	}
	
	pub fn sampleLights(&self, scene: Scene, n: Ray, rnd:f64)-> Color {
		let nLights = (scene.Lights).len();
		if nLights == 0 {
			return Black
		}
	
		if self.LightMode == LightModeEnum::LightModeAll as i32{
			let result :Color;
			for light in scene.Lights {
				result = result.Add(self.sampleLight(scene, n, rnd, light))
			}
			return result
		} else {
			// pick a random light
			let light = scene.Lights[(rand::random::<f64>()*(nLights as f64)) as usize];
			return self.sampleLight(scene, n, rnd, light).MulScalar((nLights as f64) as f64)
		}
	}
	
	pub fn sampleLight(&self, scene: Scene, n :Ray, rnd :f64, light:Box<dyn Shape>)-> Color {
		// get bounding sphere center and radius
		let mut center :Vector;
		let mut radius :f64;
		match (*light).GetType() {
		 	"Sphere"=>{
			 radius = (light).Radius;
			 center = (light).Center;
		 	},
			_ =>{
			// get bounding sphere from bounding box
				let bx = light.BoundingBox();
				radius = bx.OuterRadius();
				center = bx.Center();
			}
		}
	
		// get random point in disk
		let point = center;
		if self.SoftShadows {
			loop {
				let x = rand::random::<f64>()*2.0 - 1.0;
				let y = rand::random::<f64>()*2.0 - 1.0;
				if x*x+y*y <= 1.0 {
					let l = center.Sub(n.Origin).Normalize();
					let u = l.Cross(RandomUnitVector()).Normalize();
					let v = l.Cross(u);
					point = Vector::Default();
					point = point.Add(u.MulScalar(x * radius));
					point = point.Add(v.MulScalar(y * radius));
					point = point.Add(center);
					break;
				}
			}
		}
	
		// construct ray toward light point
		let ray = Ray{Origin:n.Origin,Direction: point.Sub(n.Origin).Normalize()};
	
		// get cosine term
		let diffuse = ray.Direction.Dot(n.Direction);
		if diffuse <= 0.0 {
			return Black
		}
	
		// check for light visibility
		let hit = scene.Intersect(ray);
		if !hit.Ok() || hit.Shape.unwrap().GetType() != light.GetType() {
			return Black
		}
	
		// compute solid angle (hemisphere coverage)
		let hyp = center.Sub(n.Origin).Length();
		let opp = radius;
		let theta = f64::asin(opp / hyp);
		let adj = opp / f64::tan(theta);
		let d = f64::cos(theta) * adj;
		let r = f64::sin(theta) * adj;
		let coverage = (r * r) / (d * d);
	
		// TODO: fix issue where hyp < opp (point inside sphere)
		if hyp < opp {
			coverage = 1.0
		}
		coverage = f64::min(coverage, 1.0);
	
		// get material properties from light
		let material = MaterialAt(light, point);
	
		// combine factors
		let m = material.Emittance.unwrap() * diffuse * coverage;
		return material.Color.unwrap().MulScalar(m)
	}
	
}
