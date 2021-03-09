
struct Scene {
	Color        :Color,
	Texture      :Texture,
	TextureAngle :f64,
	Shapes       :[Shape],
	Lights       :[Shape],
	tree         :Tree,
	rays         :u64,
}

impl Scene{

	fn Compile(&self) {
		for shape in self.Shapes {
			shape.Compile()
		}
		if self.tree == None {
			self.tree = NewTree(self.Shapes)
		}
	}
	
	fn Add(&self,shape :Shape) {
		self.Shapes = append(self.Shapes, shape)
		if shape.MaterialAt(Vector{}).Emittance > 0 {
			self.Lights = append(self.Lights, shape)
		}
	}
	
	fn RayCount(&self) ->u64 {
		return atomic.LoadUint64(&self.rays)
	}
	
	fn Intersect(&self,r:Ray)-> Hit {
		atomic.AddUint64(&self.rays, 1)
		return self.tree.Intersect(r)
	}
}

