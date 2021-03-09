extern crate pt;
extern crate math;

use pt::*;

#[derive(Debug)]
struct Box {
    Min: pt::Vector,
    Max: pt::Vector,
}

fn BoxForShapes(shapes:[Shape]) ->Box {
	if shapes.len() == 0 {
		return Box{}
	}
	let mut box:Box = shapes[0].BoundingBox()
	for _, shape in shapes {
		box = box.Extend(shape.BoundingBox())
	}
	return box
}

fn BoxForTriangles(shapes [Triangle] )->Box {
	if shapes.len() == 0 {
		return Box{}
	}
	let mut box:Box = shapes[0].BoundingBox()
	for _, shape in shapes {
		box = box.Extend(shape.BoundingBox())
	}
	return box
}

impl Box {

	fn Anchor(&self,anchor:Vector)->Vector {
		return self.Min.Add(self.Size().Mul(anchor))
	}

	fn Center(&self)->Vector {
		return self.Anchor(Vector{0.5, 0.5, 0.5})
	}

	fn OuterRadius(&self)->f64 {
		return self.Min.Sub(self.Center()).Length()
	}

	fn InnerRadius(&self)->f64 {
		return self.Center().Sub(self.Min).MaxComponent()
	}

	fn Size(&self)->Vector {
		return self.Max.Sub(self.Min)
	}

	fn Extend(&self, b:Box) -> Box {
		return Box{self.Min.Min(b.Min), self.Max.Max(b.Max)}
	}

	fn Contains(&self,b:Vector)-> bool {
		return  self.Min.X <= b.X && self.Max.X >= b.X &&
				self.Min.Y <= b.Y && self.Max.Y >= b.Y &&
				self.Min.Z <= b.Z && self.Max.Z >= b.Z
	}

	fn Intersects(&self, b:Box)-> bool {
		return !(self.Min.X > b.Max.X || self.Max.X < b.Min.X || self.Min.Y > b.Max.Y ||
			a.Max.Y < b.Min.Y || a.Min.Z > b.Max.Z || a.Max.Z < b.Min.Z)
	}

	fn Intersect(&self, r:Ray) (f64, f64) {
		let mut x1 = (self.Min.X - r.Origin.X) / r.Direction.X
		let mut y1 = (self.Min.Y - r.Origin.Y) / r.Direction.Y
		let mut z1 = (self.Min.Z - r.Origin.Z) / r.Direction.Z
		let mut x2 = (self.Max.X - r.Origin.X) / r.Direction.X
		let mut y2 = (self.Max.Y - r.Origin.Y) / r.Direction.Y
		let mut z2 = (self.Max.Z - r.Origin.Z) / r.Direction.Z
		if x1 > x2 {
			x1, x2 = x2, x1
		}
		if y1 > y2 {
			y1, y2 = y2, y1
		}
		if z1 > z2 {
			z1, z2 = z2, z1
		}
		let t1 = f64::max(f64::max(x1, y1), z1)
		let t2 = f64::min(f64::min(x2, y2), z2)
		return t1, t2
	}

	fn Partition(&self, axis:Axis, point:f64)-> (bool,bool) {
		return match axis {
			Axis::AxisX=>{(self.Min.X <= point, self.Max.X >= point)},
			Axis::AxisY=>{(self.Min.Y <= point, self.Max.Y >= point)},
			Axis::AxisZ=>{(self.Min.Z <= point, self.Max.Z >= point)},
				
		}
	}

}
