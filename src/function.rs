package pt

type Func =fn(x:f64, y:f64)->f64

struct Function {
	Function:Func,
	Box:Box,
	Material:Material,
}

fn NewFunction(function: Func, box:Box, material:Material)->Shape {
	return Function{function, box, material}
}

impl Function{
	fn  Compile(&self) {}
	
	fn  BoundingBox(&self)-> Box {
		return self.Box
	}
	
	fn  Contains(&self,v: Vector)-> bool {
		return v.Z < self.Function(v.X, v.Y)
	}
	
	fn  Intersect(&self,ray:Ray)-> Hit {
		let step = 1.0 / 32
		let sign = self.Contains(ray.Position(step))

		let t = step
		while t < 12{
			let v = ray.Position(t)
			if self.Contains(v) != sign && self.Box.Contains(v) {
				return Hit{f, t - step, None}
			}
			t+=step
		}
		return NoHit
	}
	
	fn  UV(&self,p:Vector) ->Vector {
		let (x1, x2) = (self.Box.Min.X, self.Box.Max.X)
		let (y1, y2) = (self.Box.Min.Y, self.Box.Max.Y)
		let u = (p.X - x1) / (x2 - x1)
		let v = (p.Y - y1) / (y2 - y1)
		return Vector{u, v, 0}
	}
	
	fn  MaterialAt(&self,p: Vector) -> Material {
		return self.Material
	}
	
	fn  NormalAt(&self,p: Vector) ->Vector {
		let eps = 1e-3
		let v = Vector{
			self.Function(p.X-eps, p.Y) - self.Function(p.X+eps, p.Y),
			self.Function(p.X, p.Y-eps) - self.Function(p.X, p.Y+eps),
			2 * eps,
		}
		return v.Normalize()
	}	
}
