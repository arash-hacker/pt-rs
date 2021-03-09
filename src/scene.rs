use crate::color::*;
use crate::texture::*;
use crate::vector::*;
use crate::shape::*;
use crate::tree::*;
use crate::hit::*;

struct Scene {
	Color        :Color,
	Texture      :Texture,
	TextureAngle :f64,
	Shapes       :Vec<Shape>,
	Lights       :Vec<Shape>,
	tree         :Tree,
	rays         :u64,
}

impl Scene{

	pub fn Compile(&mut self) {
		for shape in self.Shapes {
			shape.Compile();
		}
		if self.tree == None {
			*self.tree = NewTree(self.Shapes);
		}
	}
	
	pub fn Add(&mut self,shape :Shape) {
		self.Shapes.append(shape);
		if shape.MaterialAt(Vector::Default()).Emittance > 0 {
			self.Lights.append(shape) ;
		}
	}
	
	pub fn RayCount(&self) ->u64 {
		self.rays
	}
	
	pub fn Intersect(&mut self,r:Ray)-> Hit {
		*self.rays=*self.rays+1;
		self.tree.Intersect(r)
	}
}

