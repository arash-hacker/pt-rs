use crate::{axis, bbox::*};
use crate::shape::*;
use crate::material::*;
use crate::sdf::*;
use crate::hit::*;
use crate::vector::*;
use crate::triangle::*;
use crate::ray::*;
use crate::axis::*;
use crate::util::{self, *};

pub struct Tree<T:Shape>{
	pub Bx  :BBox,
	pub Root :Node<T>,
}

pub fn NewTree<T:Shape>(shapes :Vec<Box<T>>)->Tree<T> {
	println!("Building k-d tree ({} shapes)... ", shapes.len());
	let bx = BoxForShapes(shapes);
	let node = NewNode(shapes);
	node.Split(0);
	return Tree{Bx:bx,Root: node};
}



 pub struct Node<T:Shape> {
	pub Axis   :Axis,
	pub Point  :f64,
	pub Shapes :Vec<Box<T>>,
	pub Left   :Option<Box<Node<T>>>,
	pub Right  :Option<Box<Node<T>>>,
}

pub fn NewNode<T:Shape>(shapes :Vec<Box<T>>)-> Node<T> {
	return Node{Axis:Axis::AxisNone,Point: 0.0,Shapes:shapes,Left: None,Right: None};
}


impl Node<T>{

	pub fn Intersect3<T:Shape>(&self, r :Ray, tmin:f64, tmax :f64) ->Hit {
		let tsplit :f64;
		let leftFirst :bool;
		match  self.Axis {
			 AxisNone=>{return self.IntersectShapes(r)},
			 AxisX=>{
				tsplit = (self.Point - r.Origin.X) / r.Direction.X;
				leftFirst = (r.Origin.X < self.Point) || (r.Origin.X == self.Point && r.Direction.X <= 0.0);},
			AxisY=>{
				tsplit = (self.Point - r.Origin.Y) / r.Direction.Y;
				leftFirst = (r.Origin.Y < self.Point) || (r.Origin.Y == self.Point && r.Direction.Y <= 0.0);},
			AxisZ=>{
				tsplit = (self.Point - r.Origin.Z) / r.Direction.Z;
				leftFirst = (r.Origin.Z < self.Point) || (r.Origin.Z == self.Point && r.Direction.Z <= 0.0);}
		}
		let first:Node<T>;
		let second:Node<T>;
		if leftFirst {
			first = *self.Left.unwrap();
			second = *self.Right.unwrap();
		} else {
			first = *self.Right.unwrap();
			second = *self.Left.unwrap();
		}
		if tsplit > tmax || tsplit <= 0.0 {
			return first.Intersect3(r, tmin, tmax)
		} else if tsplit < tmin {
			return second.Intersect3(r, tmin, tmax)
		} else {
			let h1 = first.Intersect3(r, tmin, tsplit);
			if h1.T <= tsplit {
				return h1
			}
			let h2 = second.Intersect3(r, tsplit, f64::min(tmax, h1.T));
			if h1.T <= h2.T {
				return h1
			} else {
				return h2
			}
		}
	}
	
	pub fn IntersectShapes(&self, r:Ray)-> Hit {
		let hit = NoHit;
		for shape in self.Shapes {
			let h = shape.Intersect(r);
			if h.T < hit.T {
				hit = h
			}
		}
		return hit
	}
	
	pub fn PartitionScore(&self, axis :Axis, point :f64)-> i32 {
		let (left, right) = (0, 0);
		for  shape in self.Shapes {
			let bx = shape.BoundingBox();
			let (l, r) = bx.Partition(axis, point);
			if l {
				left+=1;
			}
			if r {
				right+=1;
			}
		}

		if left >= right {
			return left
		} else {
			return right
		}
	}
	
	pub fn Partition<T>(&self, size: i32, axis: Axis, point: f64)-> (Vec<Box<T>>,Vec<Box<T>>) {
		let left :Vec<Box<T>>;
		let right : Vec<Box<T>>;
		for  shape in self.Shapes {
			let bx = shape.BoundingBox();
			let (l, r) = bx.Partition(axis, point);
			if l {
				left.push(Box::new(*shape));
			}
			if r {
				right.push(Box::new(*shape));
			}
		}
		return (left,right)
	}
	
	pub fn Split(&self, depth :i32) {
		if self.Shapes.len() < 8 {
			return
		}
		let xs : Vec<i32>;
		let ys : Vec<i32>;
		let zs : Vec<i32>;
		for  shape in self.Shapes {
			let bx = shape.BoundingBox();
			xs.push(bx.Min.X as i32);
			xs.push(bx.Max.X as i32);
			ys.push(bx.Min.Y as i32);
			ys.push(bx.Max.Y as i32);
			zs.push(bx.Min.Z as i32);
			zs.push(bx.Max.Z as i32);
		}
		xs.sort();
		ys.sort();
		zs.sort();
		let (mx, my, mz) = (util::Median(xs), util::Median(ys), util::Median(zs));
		let mut best = (self.Shapes.len() as f64 * 0.85) as i32;
		let mut bestAxis = Axis::AxisNone;
		let mut bestPoint = 0.0;
		let mut sx = self.PartitionScore(Axis::AxisX, mx);
		if sx < best {
			best = sx;
			bestAxis = Axis::AxisX;
			bestPoint = mx;
		}
		let sy = self.PartitionScore(Axis::AxisY, my);
		if sy < best {
			best = sy;
			bestAxis = Axis::AxisY;
			bestPoint = my;
		}
		let sz = self.PartitionScore(Axis::AxisZ, mz);
		if sz < best {
			best = sz;
			bestAxis = Axis::AxisZ;
			bestPoint = mz;
		}
		if bestAxis == Axis::AxisNone {
			return
		}
		let (l, r) = self.Partition(best, bestAxis, bestPoint);
		self.Axis = bestAxis;
		self.Point = bestPoint;
		self.Left = Some(Box::new(NewNode(l)));
		self.Right = Some(Box::new(NewNode(r)));
		self.Left.unwrap().Split(depth + 1);
		self.Right.unwrap().Split(depth + 1);
		self.Shapes = vec![] ;
	}
	
}
impl<T:Shape> Node<T>{


	fn Compile(&self)->(){()}
	fn UV(&self,v:Vector) ->Vector{Vector::Default()}
	fn NormalAt(&self,v:Vector) ->Vector{Vector::Default()}
	fn MaterialAt(&self,v:Vector) ->Material{Material::Default()}
	fn GetType(&self)-> &str {"Tree"}
	fn BoundingBox(&self)-> BBox {BBox::Default()}
	fn Intersect(&self, r:Ray)->Hit {Hit::Default()}

}

impl<T> Tree<T> {
	pub fn Intersect(&self,r:Ray)->Hit {
		let (tmin, tmax) = self.Bx.Intersect(r);
		if tmax < tmin || tmax <= 0.0 {
			return NoHit;
		}
		return self.Root.Intersect3(r, tmin, tmax);
	}
}