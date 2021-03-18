use crate::{ray::Ray, triangle::Triangle, vector::*};
use crate::color::*;
use crate::axis::*;
use crate::material::*;
use crate::hit::*;
use crate::axis::*;
use crate::util::{self, *};
use crate::shape::*;

#[derive(Debug)]
pub struct BBox {
   pub Min: Vector,
   pub Max: Vector,
}

pub fn BoxForShapes<T:Shape>(shapes:Vec<Box<T>>) ->BBox {
	if shapes.len() == 0 {
		return BBox::Default();
	}
	let mut bx:BBox = shapes[0].BoundingBox();
	for shape in shapes.iter() {
		bx = bx.Extend(shape.BoundingBox());
	}
	return bx
}

fn BoxForTriangles(shapes:Vec<Triangle> )->BBox {
	if shapes.len() == 0 {
		return BBox::Default();
	}
	let mut bx:BBox = shapes[0].BoundingBox();
	for shape in shapes.iter() {
		bx = bx.Extend(shape.BoundingBox())
	}
	return bx
}

impl BBox {
	pub fn Default()->BBox{
		return BBox{
			Min:Vector{X:0.0,Y:0.0,Z:0.0},
			Max:Vector{X:0.0,Y:0.0,Z:0.0},
		}
	}

	pub fn Anchor(&self,anchor:Vector)->Vector {
		return self.Min.Add(self.Size().Mul(anchor))
	}

	pub fn Center(&self)->Vector {
		return self.Anchor(Vector{X:0.5,Y: 0.5,Z: 0.5})
	}

	pub fn OuterRadius(&self)->f64 {
		return self.Min.Sub(self.Center()).Length()
	}

	pub fn InnerRadius(&self)->f64 {
		return self.Center().Sub(self.Min).MaxComponent()
	}

	pub fn Size(&self)->Vector {
		return self.Max.Sub(self.Min)
	}

	pub fn Extend(&self, b:BBox) -> BBox {
		return BBox{Min:self.Min.Min(b.Min),Max: self.Max.Max(b.Max)}
	}

	pub fn Contains(&self,b:Vector)-> bool {
		return  self.Min.X <= b.X && self.Max.X >= b.X &&
				self.Min.Y <= b.Y && self.Max.Y >= b.Y &&
				self.Min.Z <= b.Z && self.Max.Z >= b.Z
	}

	pub fn Intersects(&self, b:BBox)-> bool {
		return !(self.Min.X > b.Max.X || self.Max.X < b.Min.X || self.Min.Y > b.Max.Y ||
			self.Max.Y < b.Min.Y || self.Min.Z > b.Max.Z || self.Max.Z < b.Min.Z)
	}

	pub fn Intersect(&self, r:Ray) ->(f64, f64) {
		let mut x1 = (self.Min.X - r.Origin.X) / r.Direction.X;
		let mut y1 = (self.Min.Y - r.Origin.Y) / r.Direction.Y;
		let mut z1 = (self.Min.Z - r.Origin.Z) / r.Direction.Z;
		let mut x2 = (self.Max.X - r.Origin.X) / r.Direction.X;
		let mut y2 = (self.Max.Y - r.Origin.Y) / r.Direction.Y;
		let mut z2 = (self.Max.Z - r.Origin.Z) / r.Direction.Z;
		if x1 > x2 {
			let (x1, x2) = (x2, x1);
		}
		if y1 > y2 {
			let (y1, y2) = (y2, y1);
		}
		if z1 > z2 {
			let	(z1, z2) = (z2, z1);
		}
		let t1 = f64::max(f64::max(x1, y1), z1);
		let t2 = f64::min(f64::min(x2, y2), z2);
		return (t1, t2)
	}

	pub fn Partition(&self, axis:Axis, point:f64)-> (bool,bool) {
		return match axis {
			Axis::AxisX=>{(self.Min.X <= point, self.Max.X >= point)},
			Axis::AxisY=>{(self.Min.Y <= point, self.Max.Y >= point)},
			Axis::AxisZ=>{(self.Min.Z <= point, self.Max.Z >= point)},
		    Axis::AxisNone => {(false,false)}
		}
	}

}
