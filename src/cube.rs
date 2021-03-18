use crate::{bbox::*, mesh::Mesh, ray::Ray};
use crate::shape::*;
use crate::common::{self, *};
use crate::material::*;
use crate::sdf::*;
use crate::hit::*;
use crate::vector::*;
use crate::triangle::*;
use crate::tree::*;
use crate::axis::*;
use crate::mesh::*;
pub struct Cube {
	pub Min   :   Vector,
	pub Max   :   Vector,
	pub Material: Material,
	pub Box     : BBox,
}

pub fn NewCube(min:Vector, max: Vector, material: Material)->Cube {
	let bx = BBox{Min:min,Max: max};
	return Cube{Min:min, Max:max, Material:material,Box: bx}
}

impl Cube{
	pub fn Mesh(&self)-> Mesh {
		let mut a = self.Min;
		let mut b = self.Max;
		let mut z = Vector::Default();
		let mut m = self.Material;
		let mut v000 = Vector{X:a.X,  Y:a.Y,  Z:a.Z};
		let mut v001 = Vector{X:a.X,  Y:a.Y,  Z:b.Z};
		let mut v010 = Vector{X:a.X,  Y:b.Y,  Z:a.Z};
		let mut v011 = Vector{X:a.X,  Y:b.Y,  Z:b.Z};
		let mut v100 = Vector{X:b.X,  Y:a.Y,  Z:a.Z};
		let mut v101 = Vector{X:b.X,  Y:a.Y,  Z:b.Z};
		let mut v110 = Vector{X:b.X,  Y:b.Y,  Z:a.Z};
		let mut v111 = Vector{X:b.X,  Y:b.Y,  Z:b.Z};
		let mut triangles:Vec<Triangle> =vec![
			NewTriangle(v000, v100, v110, z, z, z, m),
			NewTriangle(v000, v110, v010, z, z, z, m),
			NewTriangle(v001, v101, v111, z, z, z, m),
			NewTriangle(v001, v111, v011, z, z, z, m),
			NewTriangle(v000, v100, v101, z, z, z, m),
			NewTriangle(v000, v101, v001, z, z, z, m),
			NewTriangle(v010, v110, v111, z, z, z, m),
			NewTriangle(v010, v111, v011, z, z, z, m),
			NewTriangle(v000, v010, v011, z, z, z, m),
			NewTriangle(v000, v011, v001, z, z, z, m),
			NewTriangle(v100, v110, v111, z, z, z, m),
			NewTriangle(v100, v111, v101, z, z, z, m),
		];
		return NewMesh(triangles)
	}
}

impl Shape for Cube{

	fn GetType(&self) ->&str {"Cube"}

	fn Compile(&self) {}

	fn BoundingBox(&self)-> BBox {
		return self.Box
	}

	 fn Intersect(&self,r:Ray)-> Hit {
		let n = self.Min.Sub(r.Origin).Div(r.Direction);
		let f = self.Max.Sub(r.Origin).Div(r.Direction);
		let (n, f) = (n.Min(f), n.Max(f));
		let t0 = f64::max(f64::max(n.X, n.Y), n.Z);
		let t1 = f64::min(f64::min(f.X, f.Y), f.Z);
		if t0 > 0.0 && t0 < t1 {
			return Hit{Shape:Some(Box::new(*self)), T:t0, HitInfo:None}
		}
		return NoHit
	}

	 fn UV(&self,p:Vector) ->Vector {
		let p = p.Sub(self.Min).Div(self.Max.Sub(self.Min));
		return Vector{X:p.X, Y:p.Z, Z:0.0}
	}

	 fn MaterialAt(&self,p:Vector) ->Material {
		return self.Material
	}

	 fn NormalAt(&self,p:Vector) ->Vector {
		if p.X < self.Min.X+common::EPS{ return Vector{X:-1.0, Y:0.0,    Z:0.0};}
		if p.X > self.Max.X-common::EPS{ return Vector{X:1.0,  Y:0.0,    Z:0.0};}
		if p.Y < self.Min.Y+common::EPS{ return Vector{X:0.0,  Y:-1.0,   Z:0.0};}
		if p.Y > self.Max.Y-common::EPS{ return Vector{X:0.0,  Y:1.0,    Z:0.0};}
		if p.Z < self.Min.Z+common::EPS{ return Vector{X:0.0,  Y:0.0,    Z:-1.0};}
		if p.Z > self.Max.Z-common::EPS{ return Vector{X:0.0,  Y:0.0,    Z:1.0};}
		return Vector{X:0.0, Y:1.0, Z:0.0};
		
		
	}

	
}