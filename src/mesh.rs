use std::collections::HashMap;

use crate::bbox::*;
use crate::triangle::*;
use crate::vector::*;
use crate::matrix::*;
use crate::bbox::*;
use crate::shape::*;
use crate::material::*;
use crate::sdf::*;
use crate::hit::*;
use crate::vector::*;
use crate::triangle::*;
use crate::tree::*;
use crate::axis::*;
use crate::ray::*;
use crate::stl::{self, *};
pub struct Mesh<T:Shape> {
	pub Triangles :Vec<Triangle>,
	pub bx:Option<BBox>,
	pub tree:Option<Tree<T>>,
}

pub fn NewMesh(triangles: Vec<Triangle>)-> Mesh {
	return Mesh{Triangles:triangles,bx: None,tree: None};
}
impl<T> Mesh<T:Shape>{
	
	pub fn dirty(&self) {
		self.bx = None;
		self.tree = None;
	}
	
	pub fn Copy<T>(&self)-> Mesh<T> {

		let mut triangles =vec![Triangle::Default();self.Triangles.len()];
		for (i, t) in self.Triangles.iter().enumerate() {
			triangles[i] = *t;
		}
		return NewMesh(triangles)
	}
	
	pub fn  Compile(&self) {

		if self.tree.is_none() {
			let mut shapes = vec![Box::new(Triangle::Default() ) ;self.Triangles.len()];
			for (i, triangle) in self.Triangles.iter().enumerate() {
				shapes[i] = Box::new(*triangle);
			}
			self.tree = Some(NewTree(shapes));
		}
	}
	
	pub fn Add<T>(&self,b:Mesh<T>) {
		self.Triangles.append(&mut b.Triangles);
		self.dirty();
	}
	
	pub fn BoundingBox(&self) -> BBox {
		if self.bx.is_none() {
			let mut min = self.Triangles[0].V1;
			let mut max = self.Triangles[0].V1;
			for (_, t) in self.Triangles.iter().enumerate() {
				min = min.Min(t.V1).Min(t.V2).Min(t.V3);
				max = max.Max(t.V1).Max(t.V2).Max(t.V3);
			}
			self.bx = Some(BBox{Min:min,Max: max});
		}
		return self.bx.unwrap();
	}
	
	pub fn Intersect(&self, r:Ray) -> Hit {
		return self.tree.unwrap().Intersect(r)
	}
	
	pub fn UV(&self, p:Vector) -> Vector {
		return Vector::Default() // not implemented
	}
	
	pub fn MaterialAt(&self, p:Vector) -> Material {
		return Material::Default() // not implemented
	}
	
	pub fn NormalAt(&self, p:Vector) -> Vector {
		return Vector::Default() // not implemented
	}
	
	pub fn smoothNormalsThreshold(&self, normal:Vector, normals:Vec<Vector>, threshold:f64)-> Vector {
		let mut result = Vector::Default();
		for( _, x) in normals.iter().enumerate() {
			if x.Dot(normal) >= threshold {
				result = result.Add(*x);
			}
		}
		return result.Normalize()
	}
	
	pub fn SmoothNormalsThreshold(&self, radians:f64) {
		let threshold = f64::cos(radians);
		let mut lookup:HashMap<Vector,Vec<Vector>> =HashMap::new(); 
		for (_, t) in self.Triangles.iter().enumerate() {
			(lookup.get_mut(&mut t.V1).unwrap()).push(t.N1);
			(lookup.get_mut(&mut t.V2).unwrap()).push(t.N2);
			(lookup.get_mut(&mut t.V3).unwrap()).push(t.N3);
		}
		for (_, t) in self.Triangles.iter().enumerate() {
			t.N1 = self.smoothNormalsThreshold(t.N1, *lookup.get_mut(&t.V1).unwrap(), threshold);
			t.N2 = self.smoothNormalsThreshold(t.N2, *lookup.get_mut(&t.V2).unwrap(), threshold);
			t.N3 = self.smoothNormalsThreshold(t.N3, *lookup.get_mut(&t.V3).unwrap(), threshold);
		}
	}
	
	pub fn SmoothNormals(&self) {
		let mut lookup :HashMap<Vector,Vector>=HashMap::new();
		for( _, t) in self.Triangles.iter().enumerate() {
			(lookup.get_mut(&mut t.V1).unwrap()).Add(t.N1);
			(lookup.get_mut(&mut t.V2).unwrap()).Add(t.N2);
			(lookup.get_mut(&mut t.V3).unwrap()).Add(t.N3);
		}
		for (k,v)  in lookup {
			let vv=lookup.get_mut(&mut k).unwrap();
			*vv= v.Normalize();
		}
		for (_, t) in self.Triangles.iter_mut().enumerate() {
			(t).N1 = *lookup.get_mut(&mut t.V1).unwrap();
			(t).N2 = *lookup.get_mut(&mut t.V2).unwrap();
			(t).N3 = *lookup.get_mut(&mut t.V3).unwrap();
		}
	}
	
	pub fn UnitCube(&self) {
		self.FitInside(BBox{Min:Vector::Default(),Max:Vector{X:1.0,Y: 1.0,Z: 1.0}}, Vector::Default());
		self.MoveTo(Vector::Default(), Vector{X:0.5,Y: 0.5,Z: 0.5});
	}
	
	pub fn MoveTo(&self, position:Vector, anchor:Vector) {
		let matrix = Translate(position.Sub(self.BoundingBox().Anchor(anchor)));
		self.Transform(matrix);
	}
	
	pub fn FitInside(&self, bx:BBox, anchor:Vector) {
		let scale = bx.Size().Div(self.BoundingBox().Size()).MinComponent();
		let extra = bx.Size().Sub(self.BoundingBox().Size().MulScalar(scale));
		let mut matrix = Identity();
		matrix = matrix.Translate(self.BoundingBox().Min.Negate());
		matrix = matrix.Scale(Vector{X:scale,Y: scale,Z: scale});
		matrix = matrix.Translate(bx.Min.Add(extra.Mul(anchor)));
		self.Transform(matrix);
	}
	
	pub fn Transform(&self, matrix:Matrix) {
		for mut t in self.Triangles.iter() {
			t.V1 = matrix.MulPosition(t.V1) ;
			t.V2 = matrix.MulPosition(t.V2) ;
			t.V3 = matrix.MulPosition(t.V3) ;
			t.N1 = matrix.MulDirection(t.N1);
			t.N2 = matrix.MulDirection(t.N2);
			t.N3 = matrix.MulDirection(t.N3);
		}
		self.dirty()
	}
	
	pub fn etMaterial(&self, material:Material) {
		for mut t in self.Triangles {
			t.Material = Some(material)
		}
	}
	
	pub fn SaveSTL(&self, path:&str) {
		return stl::SaveSTL(String::from(path), *self);
	}
	
}
