use crate::vector::*;
use crate::ray::*;
use crate::bbox::*;

trait Shape  {
	fn GetType(&self)->&str;
	fn Compile(&self)->();
	fn BoundingBox(&self) ->BBox;
	fn Intersect(&self,r:Ray) ->Hit;
	fn UV(&self,v:Vector) ->Vector;
	fn NormalAt(&self,v:Vector) ->Vector;
	fn MaterialAt(&self,v:Vector) ->Material;
}
#[derive(Debug,Clone, Copy)]
struct TransformedShape {
	shape:Shape,
	matrix:Matrix,
	inverse:Matrix,
}

fn NewTransformedShape(s:Shape, m:Matrix) ->dyn Shape {
	TransformedShape{shape:s,matrix: m,inverse: m.Inverse()}
}

impl TransformedShape {

	fn BoundingBox(&self)-> BBox {
		return self.matrix.MulBox(self.shape.BoundingBox())
	}
	
	fn Intersect(r:Ray)->Hit {
		let shapeRay = self.inverse.MulRay(r);
		let mut hit = self.shape.Intersect(shapeRay);
		if !hit.Ok() {
			return hit;
		}
		let shape = hit.shape;
		let shapePosition = shapeRay.Position(hit.T);
		let shapeNormal = shape.NormalAt(shapePosition);
		let position = self.matrix.MulPosition(shapePosition);
		let mut normal = self.inverse.Transpose().MulDirection(shapeNormal);
		let material = MaterialAt(shape, shapePosition);
		let mut inside = false;
		if shapeNormal.Dot(shapeRay.Direction) > 0 {
			normal = normal.Negate();
			inside = true;
		}
		let ray = Ray{ Origin:position,Direction: normal};
		let info = HitInfo{
			Shape:shape,
			Position: position,
			Normal:  normal,
			Ray:   ray,
			Material:    material,
			Inside:	 inside
		};
		hit.T = position.Sub(r.Origin).Length();
		hit.HitInfo = info;
		return hit;
	}
}

