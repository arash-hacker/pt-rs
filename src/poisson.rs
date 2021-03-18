use std::collections::HashMap;
use crate::bbox::*;
use crate::shape::*;
use crate::vector::*;
use crate::material::*;
use crate::sdf::*;
use crate::hit::*;
use crate::vector::*;
use crate::triangle::*;
use crate::tree::*;
use crate::axis::*;
use crate::util::{self, *};
extern crate rand;
pub struct poissonGrid{
	pub r:f64, 
	pub size :f64,
	pub cells :HashMap<Vector,Vector>,  
}

pub fn newPoissonGrid(r :f64)-> poissonGrid {
	let size = r / f64::sqrt(2.0);
	let hs=HashMap::new();
	return poissonGrid{r:r, size:size, cells: hs}
}
impl poissonGrid{

	pub fn normalize(&self,v: Vector) ->Vector {
		let i = f64::floor(v.X / self.size);
		let j = f64::floor(v.Y / self.size);
		return Vector{X:i, Y:j, Z:0.0}
	}
	
	pub fn insert(&self,v:Vector) ->bool {
		let n = self.normalize(v);
		for i in ((n.X - 2.0) as i32)..(n.X+3.0) as i32 {
			for j in ((n.Y - 2.0) as i32)..(n.Y+3.0) as i32 {
				if let (m) = self.cells.get_mut(&Vector{X:i as f64, Y:j as f64, Z:0.0}) {
					
					if m.is_some() && (m.unwrap().X - v.X).hypot(m.unwrap().Y-v.Y) < self.r {
						return false
					}
				}
			}
		}
		let get_mut = self.cells.get_mut(&n).unwrap();
		*get_mut = v;
		return true
	}
	
	pub fn PoissonDisc(&self,x1:f64, y1:f64, x2:f64, y2:f64, r:f64, n:i32) ->Vec<Vector> {
		let result: Vec<Vector>;
		let x = x1 + (x2-x1)/2.0;
		let y = y1 + (y2-y1)/2.0;
		let v = Vector{X:x,Y: y, Z:0.0};
		let active = vec![Vector{X:v.X,Y:v.Y,Z:v.Z}];
		let grid = newPoissonGrid(r);
		self.insert(v);
		result.push(v);
		loop  {
			if active.len() > 0 {
				break;
			}

			let index:i32 = (rand::random::<f64>()*((active).len()) as f64 )as i32;
			let point:Vector = active[index as usize];
			let ok = false;
			for i in 0..n {
				let a = rand::random::<f64>() * 2.0 * util::PI;
				let d = rand::random::<f64>()*r + r;
				let x = point.X + f64::cos(a)*d;
				let y = point.Y + f64::sin(a)*d;
				if x < x1 || y < y1 || x > x2 || y > y2 {
					continue
				}
				let v = Vector{X:x, Y:y, Z:0.0};
				if !self.insert(v) {
					continue
				}
				result.push(v);
				active.push(v);
				ok = true;
				break
			}
			if !ok {
				//TODO :???
				// active = append(active[:index], active[index+1:]...)
			}
		}
		return result
	}
	
}