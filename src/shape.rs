use crate::vector::*;
use crate::ray::*;
use crate::bbox::*;
use crate::color::*;
use crate::axis::*;
use crate::material::*;
use crate::matrix::*;
use crate::hit::*;

pub trait Shape  {
	fn GetType(&self)->&str;
	fn Compile(&self)->();
	fn BoundingBox(&self) ->BBox;
	fn Intersect(&self,r:Ray) ->Hit;
	fn UV(&self,v:Vector) ->Vector;
	fn NormalAt(&self,v:Vector) ->Vector;
	fn MaterialAt(&self,v:Vector) ->Material;
}
pub struct TransformedShape {
	pub shape:Box<dyn Shape>,
	pub matrix:Matrix,
	pub inverse:Matrix,
}

pub fn NewTransformedShape(s:Box<dyn Shape>, m:Matrix) ->Box<dyn Shape> {
	return Box::new(TransformedShape{shape:s, matrix:m, inverse:m.Inverse()})
}

impl Shape for TransformedShape {

	fn Compile(&self)->(){()}
	fn UV(&self,v:Vector) ->Vector{Vector::Default()}
	fn NormalAt(&self,v:Vector) ->Vector{Vector::Default()}
	fn MaterialAt(&self,v:Vector) ->Material{Material::Default()}

	fn GetType(&self)-> &str {
		"TransformedShape"
	}

	fn BoundingBox(&self)-> BBox {
		return self.matrix.MulBox(self.shape.BoundingBox())
	}
	
	fn Intersect(&self, r:Ray)->Hit {
		let shapeRay = self.inverse.MulRay(r);
		let mut hit = self.shape.Intersect(shapeRay);
		if !hit.Ok() {
			return hit;
		}
		let shape = hit.Shape.unwrap();
		let shapePosition = shapeRay.Position(hit.T);
		let shapeNormal = shape.NormalAt(shapePosition);
		let position = self.matrix.MulPosition(shapePosition);
		let mut normal = self.inverse.Transpose().MulDirection(shapeNormal);
		let material = MaterialAt(shape, shapePosition);
		let mut inside = false;
		if shapeNormal.Dot(shapeRay.Direction) > 0.0 {
			normal = normal.Negate();
			inside = true;
		}
		let ray = Ray{ Origin:position,Direction: normal};
		let info = HitInfo{
			Shape:Some(shape),
			Position: position,
			Normal:normal,
			Ray:ray,
			Material:material,
			Inside:inside
		};
		hit.T = position.Sub(r.Origin).Length();
		hit.HitInfo = Some(info);
		return hit;
	}
}

