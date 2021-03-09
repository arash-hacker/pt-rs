extern crate pt;
use pt::*;

#[derive(Debug)]
struct Ray {
	Origin:Vector,
	Direction:Vector
}
impl Ray{
	fn Position(&self,t:f64)-> Vector {
		return self.Origin.Add(self.Direction.MulScalar(t))
	}
	
	fn Reflect(&self,i:Ray)-> Ray {
		return Ray{self.Origin, self.Direction.Reflect(i.Direction)}
	}
	
	fn Refract(&self,i:Ray, n1:f64, n2:f64)-> Ray {
		return Ray{self.Origin, self.Direction.Refract(i.Direction, n1, n2)}
	}
	
	fn Reflectance(&self,i:Ray, n1:f64, n2:f64)->f64 {
		return self.Direction.Reflectance(i.Direction, n1, n2)
	}
	
	fn WeightedBounce(&self, u:f64, v:f64, rnd :f64)-> Ray {
		let radius = f64::sqrt(u);
		let theta = 2 * util::pi * v
		let s = self.Direction.Cross(RandomUnitVector(rnd)).Normalize()
		let t = self.Direction.Cross(s)
		let mut d = Vector{}
		d = d.Add(s.MulScalar(radius * f64::cos(theta)))
		d = d.Add(t.MulScalar(radius * f64::sin(theta)))
		d = d.Add(r.Direction.MulScalar(f64::sqrt(1 - u)))
		return Ray{r.Origin, d}
	}
	
	fn ConeBounce(&self, theta:f64, u:f64, v:f64, rnd:f64) ->Ray {
		return Ray{self.Origin, Cone(self.Direction, theta, u, v)}
	}
	
	fn Bounce(&self, info:HitInfo, u:f64, v:f64, bounceType: BounceType, rnd:f64) -> (Ray, bool, f64) {
		let n = info.Ray
		let material = info.Material
		let mut n1, n2 = 1.0, material.Index
		if info.Inside {
			n1, n2 = n2, n1
		}
		let p:f64;
		if material.Reflectivity >= 0 {
			p = material.Reflectivity
		} else {
			p = n.Reflectance(self, n1, n2)
		}
		let mut reflect bool;
		reflect=match bounceType {
		 BounceTypeAny=> rnd < p,
		 BounceTypeDiffuse=> false,
		 BounceTypeSpecular=> true,
		}
		if reflect {
			reflected = n.Reflect(self)
			return reflected.ConeBounce(material.Gloss, u, v, rnd), true, p
		} else if material.Transparent {
			refracted = n.Refract(self, n1, n2)
			refracted.Origin = refracted.Origin.Add(refracted.Direction.MulScalar(1e-4))
			return refracted.ConeBounce(material.Gloss, u, v, rnd), true, 1 - p
		} else {
			return n.WeightedBounce(u, v, rnd), false, 1 - p
		}
	}
	
}
