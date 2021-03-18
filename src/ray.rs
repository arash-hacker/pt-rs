use rand::random;
use crate::{vector::*};
use crate::color::*;
use crate::axis::*;
use crate::material::*;
use crate::hit::*;
use crate::util::{self, *};
use crate::sampler::{self,*};

#[derive(Debug)]
pub struct Ray {
	pub Origin:Vector,
	pub Direction:Vector
}
impl Ray{
	pub fn Position(&self,t:f64)-> Vector {
		return self.Origin.Add(self.Direction.MulScalar(t))
	}
	
	pub fn Reflect(&self,i:Ray)-> Ray {
		return Ray{Origin:self.Origin,Direction: self.Direction.Reflect(i.Direction)}
	}
	
	pub fn Refract(&self,i:Ray, n1:f64, n2:f64)-> Ray {
		return Ray{Origin:self.Origin,Direction: self.Direction.Refract(i.Direction, n1, n2)}
	}
	
	pub fn Reflectance(&self,i:Ray, n1:f64, n2:f64)->f64 {
		return self.Direction.Reflectance(i.Direction, n1, n2)
	}
	
	pub fn WeightedBounce(&self, u:f64, v:f64, rnd :f64)-> Ray {
		let radius = f64::sqrt(u);
		let theta = 2.0 * PI * v;
		let s = self.Direction.Cross(RandomUnitVector()).Normalize();
		let t = self.Direction.Cross(s);
		let mut d = Vector::Default();
		d = d.Add(s.MulScalar(radius * f64::cos(theta)));
		d = d.Add(t.MulScalar(radius * f64::sin(theta)));
		d = d.Add(self.Direction.MulScalar(f64::sqrt(1.0 - u)));
		return Ray{Origin:self.Origin,Direction: d}
	}
	
	pub fn ConeBounce(&self, theta:f64, u:f64, v:f64, rnd:f64) ->Ray {
		return Ray{Origin:self.Origin,Direction: Cone(self.Direction, theta, u, v)}
	}
	
	pub fn Bounce(&self, info:HitInfo, u:f64, v:f64, bounceType: BounceType, rnd:f64) -> (Ray, bool, f64) {
		let n = info.Ray;
		let material = info.Material;
		let  (n1, n2) = (1.0, material.Index);
		if info.Inside {
			let (n1, n2) = (n2, n1);
		}
		let p:f64;
		if material.Reflectivity.unwrap() >= 0.0 {
			p = material.Reflectivity.unwrap();
		} else {
			p = n.Reflectance(*self, n1, n2.unwrap());
		}
		let mut reflect: bool;
		let mut reflect=match bounceType {
		 	sampler::BounceTypeAny=> random::<f64>() < p,
		 	sampler::BounceTypeDiffuse=> false,
		 	sampler::BounceTypeSpecular=> true,
		};
		if reflect {
			let reflected = n.Reflect(*self);
			return (reflected.ConeBounce(material.Gloss.unwrap(), u, v, rnd), true, p)
		} else if material.Transparent.unwrap() {
			let mut refracted = n.Refract(*self, n1, n2.unwrap());
			refracted.Origin = refracted.Origin.Add(refracted.Direction.MulScalar(1e-4));
			return (refracted.ConeBounce(material.Gloss.unwrap(), u, v, rnd), true, 1.0 - p);
		} else {
			return (n.WeightedBounce(u, v, rnd), false, 1.0 - p)
		}
	}
	
}
