trait Shape  {
	fn Compile(&self)->();
	fn BoundingBox(&self) ->Box;
	fn Intersect(&self,r:Ray) ->Hit;
	fn UV(&self,v:Vector) ->Vector;
	fn NormalAt(&self,v:Vector) ->Vector;
	fn MaterialAt(&self,v:Vector) ->Material;
}

struct TransformedShape {
	shape:Shape,
	matrix:Matrix,
	inverse:Matrix,
}

fn NewTransformedShape(s:Shape, m:Matrix) -> Shape {
	return &TransformedShape{s, m, m.Inverse()}
}

impl TransformedShape {

	fn BoundingBox(&self) Box {
		return self.matrix.MulBox(self.shape.BoundingBox())
	}
	
	fn Intersect(r:Ray)->Hit {
		let shapeRay = self.inverse.MulRay(r)
		let hit = self.shape.Intersect(shapeRay)
		if !hit.Ok() {
			return hit
		}
		let shape = hit.shape
		let shapePosition = shapeRay.Position(hit.T)
		let shapeNormal = shape.NormalAt(shapePosition)
		let position = self.matrix.MulPosition(shapePosition)
		let mut normal = self.inverse.Transpose().MulDirection(shapeNormal)
		let material = MaterialAt(shape, shapePosition)
		let mut inside = false
		if shapeNormal.Dot(shapeRay.Direction) > 0 {
			normal = normal.Negate()
			inside = true
		}
		let ray = Ray{position, normal}
		let info = HitInfo{shape, position, normal, ray, material, inside}
		hit.T = position.Sub(r.Origin).Length()
		hit.HitInfo = &info
		return hit
	}
}

