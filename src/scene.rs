use crate::{color::*, ray::Ray, tree};
use crate::texture::*;
use crate::vector::*;
use crate::shape::*;
use crate::tree::*;
use crate::hit::*;

pub struct Scene {
	pub Color        :Color,
	pub Texture      :Option<Texture>,
	pub TextureAngle :f64,
	pub Shapes       :Vec<Box<dyn Shape>>,
	pub Lights       :Vec<Box<dyn Shape>>,
	pub tree         :Option<Tree>,
	pub rays         :u64,
}

impl Scene{

	pub fn Compile(&mut self) {
		for shape in self.Shapes {
			shape.Compile();
		}
		//TODO: check with None Some
		match self.tree {
			None=>{
				let m=(self).tree.unwrap();
				m= NewTree(self.Shapes);
			}
			_=>{}
		}
	}
	
	pub fn Add(&mut self,shape :Box<dyn Shape>) {
		self.Shapes.push(shape);
		if shape.MaterialAt(Vector::Default()).Emittance.unwrap() > 0.0 {
			self.Lights.push(shape);
		}
	}
	
	pub fn RayCount(&self) ->u64 {
		self.rays
	}
	
	pub fn Intersect(&mut self,r:Ray)-> Hit {
		(*self).rays=(*self).rays+1;
		self.tree.unwrap().Intersect(r)
	}
}

