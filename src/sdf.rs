// use crate::shape::*;
// use crate::material::*;
// use crate::hit::*;
// use crate::vector::*;
// use crate::bbox::*;
// use crate::ray::*;
// use crate::matrix::*;

//  pub struct SDFShape{
// 	pub SDF:Box<dyn SDF>,
// 	pub Material: Material,
// }

// pub fn NewSDFShape(sdf :Box<dyn SDF>, material: Material) ->Box<dyn Shape> {
// 	return Box::new(SDFShape{SDF:sdf,Material: material})
// }

// impl Shape for SDFShape{

// 	pub fn Compile(&self) {
// 	}
	
// 	pub fn Intersect(&self,ray:Ray) ->Hit {
// 		let epsilon = 0.00001;
// 		let start = 0.0001;
// 		let jumpSize = 0.001;
// 		let bx = (*self).BoundingBox();
// 		let (t1, t2) = bx.Intersect(ray);
// 		if t2 < t1 || t2 < 0 {
// 			return NoHit
// 		}
// 		let t = f64::max(start, t1);
// 		let jump = true;
// 		for i in 0..1000 {
// 			let d = (*self).Evaluate(ray.Position(t));
// 			if jump && d < 0 {
// 				t -= jumpSize;
// 				jump = false;
// 				continue
// 			}
// 			if d < epsilon {
// 				return Hit{Shape:Box::new(*self), T:t, HitInfo:None}
// 			}
// 			if jump && d < jumpSize {
// 				d = jumpSize
// 			}
// 			t += d;
// 			if t > t2 {
// 				return NoHit
// 			}
// 		}
// 		return NoHit
// 	}
	
// 	pub fn UV(&self,p:Vector) ->Vector {
// 		return Vector::Default()
// 	}
	
// 	pub fn NormalAt(&self,p:Vector) ->Vector {
// 		let e = 0.0001;
// 		let (x, y, z) =( p.X, p.Y, p.Z);
// 		let n = Vector{
// 			X:self.Evaluate(Vector{X:x - e,Y: y,Z: z}) - self.Evaluate(Vector{X:x + e,Y: y,Z: z}),
// 			Y:self.Evaluate(Vector{X:x,Y: y - e,Z: z}) - self.Evaluate(Vector{X:x,Y: y + e,Z: z}),
// 			Z:self.Evaluate(Vector{X:x,Y: y, Z:z - e}) - self.Evaluate(Vector{X:x, Y:y,Z: z + e}),
// 		};
// 		return n.Normalize()
// 	}
	
// 	pub fn MaterialAt(&self,p:Vector) ->Material {
// 		return self.Material
// 	}
// }


// // SDF

// pub trait SDF  {
// 	fn Evaluate(p: Vector)-> f64 where Self: Sized;
// 	fn BoundingBox()-> BBox where Self: Sized;
// }

// // SphereSDF

// pub struct SphereSDF {
// 	pub Radius   : f64,
// 	pub Exponent : f64,
// }

// pub fn NewSphereSDF(radius :f64) -> Box<dyn SDF> {
// 	return SphereSDF{Radius:radius,Exponent: 2}
// }
// impl SphereSDF{
// 	pub  fn Evaluate(&self, p: Vector) -> f64 {
// 		return p.LengthN(self.Exponent) - self.Radius
// 	}
	
// 	pub  fn BoundingBox(&self) ->BBox {
// 		let r = self.Radius;
// 		return BBox{Min: Vector{X:-r,Y: -r,Z: -r},Max: Vector{X:r,Y:r,Z: r}};
// 	}
// }


// // CubeSDF

// pub struct CubeSDF {
// 	pub Size :Vector
// }

// pub fn NewCubeSDF(size: Vector) ->Box<dyn SDF> {
// 	return CubeSDF{Size:size}
// }
// impl CubeSDF{
// 	pub fn Evaluate(&self, p:Vector) -> f64 {
// 		let x = p.X;
// 		let y = p.Y;
// 		let z = p.Z;
// 		if x < 0 {
// 			x = -x
// 		}
// 		if y < 0 {
// 			y = -y
// 		}
// 		if z < 0 {
// 			z = -z
// 		}
// 		x -= self.Size.X / 2;
// 		y -= self.Size.Y / 2;
// 		z -= self.Size.Z / 2;
// 		let a = x;
// 		if y > a {
// 			a = y;
// 		}
// 		if z > a {
// 			a = z;
// 		}
// 		if a > 0 {
// 			a = 0;
// 		}
// 		if x < 0 {
// 			x = 0;
// 		}
// 		if y < 0 {
// 			y = 0;
// 		}
// 		if z < 0 {
// 			z = 0;
// 		}
// 		let b = f64::sqrt(x*x + y*y + z*z);
// 		return a + b
// 	}
	
// 	pub fn BoundingBox(&self)-> BBox {
// 		let (x, y, z) = (self.Size.X/2, self.Size.Y/2, self.Size.Z/2);
// 		return BBox{Min:Vector{X:-x,Y: -y,Z: -z},Max: Vector{X:x, Y:y, Z:z}}
// 	}
// }


// // CylinderSDF

// pub struct CylinderSDF {
// 	pub Radius :f64,
// 	pub Height :f64,
// }

// pub fn NewCylinderSDF(radius:f64, height :f64) ->Box<dyn SDF> {
// 	return CylinderSDF{Radius:radius,Height: height}
// }
// impl CylinderSDF{

// 	pub fn Evaluate(&self, p:Vector) ->f64 {
// 		let mut x = f64::sqrt(p.X*p.X + p.Z*p.Z);
// 		let mut y = p.Y;
// 		if x < 0 {
// 			x = -x;
// 		}
// 		if y < 0 {
// 			y = -y;
// 		}
// 		x -= self.Radius;
// 		y -= self.Height / 2;
// 		let a = x;
// 		if y > a {
// 			a = y;
// 		}
// 		if a > 0 {
// 			a = 0;
// 		}
// 		if x < 0 {
// 			x = 0;
// 		}
// 		if y < 0 {
// 			y = 0;
// 		}
// 		let b = f64::sqrt(x*x + y*y);
// 		return a + b
// 	}
	
// 	pub fn BoundingBox(&self, )-> BBox {
// 		let r = self.Radius;
// 		let h = self.Height / 2;
// 		return BBox{Min:Vector{X:-r,Y: -h,Z: -r},Max: Vector{X:r, Y:h, Z:r}};
// 	}
// }


// // CapsuleSDF

// pub struct  CapsuleSDF{
// 	pub A:Vector, 
// 	pub B:Vector,
// 	pub Radius   :f64,
// 	pub Exponent :f64,
// }

// pub fn NewCapsuleSDF(a:Vector , b :Vector, radius: f64)-> Box<dyn SDF> {
// 	return CapsuleSDF{A:a,B: b,Radius: radius,Exponent: 2}
// }
// impl  CapsuleSDF{
// 	pub fn Evaluate(&self,p:Vector) ->f64 {
// 		let pa = p.Sub(self.A);
// 		let ba = self.B.Sub(self.A);
// 		let h  = f64::max(0, f64::min(1, pa.Dot(ba)/ba.Dot(ba)));
// 		return pa.Sub(ba.MulScalar(h)).LengthN(self.Exponent) - self.Radius;
// 	}
	
// 	pub fn BoundingBox(&self) ->BBox {
// 		let (a, b) = (self.A.Min(self.B), self.A.Max(self.B));
// 		return BBox{Min:a.SubScalar(self.Radius),Max: b.AddScalar(self.Radius)}
// 	}
// }


// // TorusSDF

// pub struct TorusSDF{
// 	pub MajorRadius   :f64,
// 	pub MinorRadius   :f64,
// 	pub MajorExponent :f64,
// 	pub MinorExponent :f64,
// }

// pub fn NewTorusSDF(major:f64, minor :f64)-> Box<dyn SDF> {
// 	return TorusSDF{MajorRadius:major,MinorRadius: minor,MajorExponent: 2,MinorExponent: 2}
// }
// impl TorusSDF{
// 	pub fn Evaluate(&self,p:Vector) ->f64 {
// 		let q = Vector{X:Vector{X:p.X,Y: p.Y,Z: 0}.LengthN(self.MajorExponent) - self.MajorRadius,Y: p.Z,Z: 0};
// 		return q.LengthN(self.MinorExponent) - self.MinorRadius
// 	}
	
// 	pub fn BoundingBox(&self,) ->BBox {
// 		let a = self.MinorRadius;
// 		let b = self.MinorRadius + self.MajorRadius;
// 		return BBox{Min:Vector{X:-b,Y: -b,Z: -a},Max: Vector{X:b,Y: b,Z: a}};
// 	}
// }


// // TransformSDF

// pub struct TransformSDF  {
// 	pub SDF:Box<dyn SDF>,
// 	pub Matrix  :Matrix,
// 	pub Inverse :Matrix,
// }

// pub fn NewTransformSDF(sdf: Box<dyn SDF>, matrix: Matrix) ->Box<dyn SDF> {
// 	return TransformSDF{SDF:sdf, Matrix:matrix, Inverse:matrix.Inverse()}
// }
// impl TransformSDF{

// 	pub fn Evaluate(&self,p: Vector)-> f64 {
// 		let q = self.Inverse.MulPosition(p);
// 		return self.SDF.Evaluate(q)
// 	}
	
// 	pub fn BoundingBox(&self) ->BBox {
// 		return self.Matrix.MulBox(self.SDF.BoundingBox())
// 	}
	
// }

// // ScaleSDF

// pub struct ScaleSDF{
// 	pub SDF:Box<dyn SDF>,
// 	pub Factor:f64,
// }

// pub fn NewScaleSDF(sdf: Box<dyn SDF>, factor :f64) ->Box<dyn SDF> {
// 	return ScaleSDF{SDF:sdf, Factor:factor}
// }
// impl ScaleSDF{
	
// 	pub fn Evaluate(&self,p:Vector) ->f64 {
// 		return self.SDF.Evaluate(p.DivScalar(self.Factor)) * self.Factor
// 	}

// 	pub fn BoundingBox(&self)-> BBox {
// 		let f = self.Factor;
// 		let m = Scale(Vector{X:f,Y: f,Z: f});
// 		return m.MulBox(self.SDF.BoundingBox())
// 	}
// }


// // UnionSDF

// pub struct UnionSDF{
// 	pub Items :Vec<Box<dyn SDF>>
// }

// pub fn NewUnionSDF(items :Vec<Box<dyn SDF>>)-> Box<dyn SDF> {
// 	return UnionSDF{Items:..items}
// }
// impl UnionSDF{
// 	pub fn Evaluate(&self,p :Vector)-> f64 {
// 		let result :f64;
// 		for (i, item) in self.Itemself.iter().enumerate() {
// 			let d = item.Evaluate(p);
// 			if i == 0 || d < result {
// 				result = d
// 			}
// 		}
// 		return result
// 	}
	
// 	pub fn BoundingBox(&self)-> BBox {
// 		let result: BBox;
// 		for (i, item) in self.Itemself.iter().enumerate() {
// 			let bx = item.BoundingBox();
// 			if i == 0 {
// 				result = bx;
// 			} else {
// 				result = result.Extend(bx);
// 			}
// 		}
// 		return result
// 	}
// }


// // DifferenceSDF

//  pub struct DifferenceSDF{
// 	pub Items: Vec<Box<dyn SDF>>
// }

// pub fn NewDifferenceSDF(items:Vec<Box<dyn SDF>>)-> Box<dyn SDF> {
// 	return DifferenceSDF{Items:..items}
// }

// impl DifferenceSDF{

// 	pub fn Evaluate(&self,p:Vector) ->f64 {
// 		let result :f64;
// 		for (i, item) in self.Itemself.iter().enumerate() {
// 			let d = item.Evaluate(p);
// 			if i == 0 {
// 				result = d;
// 			} else if -d > result {
// 				result = -d;
// 			}
// 		}
// 		return result
// 	}
	
// 	pub fn BoundingBox(&self) ->BBox {
// 		return self.Items[0].BoundingBox()
// 	}
	
// }

// // IntersectionSDF

// pub struct IntersectionSDF {
// 	pub Items:Vec<Box<dyn SDF>>
// }

// pub fn NewIntersectionSDF(items: Vec<Box<dyn SDF>>) ->Box<dyn SDF> {
// 	return IntersectionSDF{Items:..items}
// }
// impl IntersectionSDF{

// 	pub fn Evaluate(&self,p :Vector) ->f64 {
// 		let result :f64;
// 		for (i, item) in self.Items {
// 			let d = item.Evaluate(p);
// 			if i == 0 || d > result {
// 				result = d
// 			}
// 		}
// 		return result
// 	}
	
// 	pub fn BoundingBox(&self)-> BBox {
// 		// TODO: intersect boxes
// 		let mut result :BBox;
// 		for (i, item) in self.Itemself.iter().enumerate() {
// 			let bx = item.BoundingBox();
// 			if i == 0 {
// 				result = bx;
// 			} else {
// 				result = result.Extend(bx);
// 			}
// 		}
// 		return result
// 	}
	
// }

// // RepeatSDF

// pub struct RepeatSDF{
// 	pub SDF:Box<dyn SDF>,
// 	pub Step :Vector,
// }

// pub fn NewRepeatSDF(sdf:Box<dyn SDF>, step: Vector)-> Box<dyn SDF> {
// 	return RepeatSDF{SDF:sdf,Step:step}
// }

// impl RepeatSDF{
// 	pub fn Evaluate(&self,p :Vector) ->f64 {
// 		let q = p.Mod(self.Step).Sub(self.Step.DivScalar(2));
// 		return self.SDF.Evaluate(q)
// 	}
	
// 	pub fn BoundingBox(&self)-> BBox {
// 		// TODO: fix this
// 		return BBox::Default()
// 	}
	
// }
