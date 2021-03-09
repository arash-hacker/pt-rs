use image::*;
use std::time;
use crate::common;
pub const PI:f64=3.14159265359;
use std::path::Path;
use crate::vector::{self,Vector};

pub fn Radians(degrees :f64)-> f64 {
	return degrees * PI / 180.0
}

pub fn Degrees(radians :f64) ->f64 {
	return radians * 180.0 / PI
}

pub fn Cone(direction :Vector, theta:f64, u:f64, v:f64) -> Vector {
	if theta < common::EPS as f64 {
		return direction
	}
	let theta = theta * (1.0 - (2.0 * f64::acos(u) / PI));
	let m1 = f64::sin(theta);
	let m2 = f64::cos(theta);
	let a = v * 2.0 * PI;
	let q = vector::RandomUnitVector();
	let s = direction.Cross(q);
	let t = direction.Cross(s);
	let mut  d = Vector::Default();
	d = d.Add(s.MulScalar(m1 * f64::cos(a)));
	d = d.Add(t.MulScalar(m1 * f64::sin(a)));
	d = d.Add(direction.MulScalar(m2));
	d = d.Normalize();
	return d
}
//TODO error handling inside result 
pub fn LoadImage(path:String) ->ImageResult<DynamicImage> {
	return image::open(path);
}

pub fn SavePNG(path:String, im:image::RgbaImage) {
	im.save(path).unwrap()
}

pub fn Median(items :Vec<f64>)-> f64 {
	let n = items.len() ;
		if n== 0 {0.0}
		else if n%2 == 1 {items[n/2]}
		else {	
			let a = items[n/2-1];
			let b = items[n/2];
			return (a + b) / 2.0;
		}
	
}

pub fn DurationString(d: time::Duration) ->String {
	
	let h = d.as_secs() /60;
	let m = (d.as_secs() / 60) % 60;
	let s = d.as_secs() / 3600;
	return format!("{}:{}:{}", h, m, s)
}

pub fn NumberString(x: f64) ->String {
	let suffixes = &["", "k", "M", "G"];
	let mut x=x;
	for  suffix in suffixes.iter() {
		if x < 1000.0 {
			return format!("{}{}", x, suffix)
		}
		x /= 1000.0;
	}
	return format!("{}{}", x, "T")
}

pub fn ParseFloats(items:Vec<String>)-> Vec<f64> {
	let mut result=vec![0.0;items.len()];
	for (i, item) in items.iter().enumerate() {
		let f:f64 = item.parse::<f64>().unwrap();
		result[i] = f;
	}
	return result
}

pub fn ParseInts(items: Vec<String>)-> Vec<i32> {
	let mut result=vec![0; items.len()];
	for (i, item) in items.iter().enumerate() {
		result[i] = item.parse::<i32>().unwrap();
	}
	return result
}

pub fn RelativePath(path1:String, path2:String) ->String {
	let dir  = Path::new(&path1).parent().unwrap();
	return String::from(Path::new(dir).join(path2).to_str().unwrap())
}
pub fn Modf(x:f64)->(f64,f64){
	return (f64::floor(x),f64::ceil(x)-x)
}
pub fn Fract(x :f64) -> f64 {
	let (_, x) = Modf(x);
	return x
}

pub fn Clamp(x:f64, lo:f64, hi:f64)-> f64 {
	if x < lo {
		return lo
	}
	if x > hi {
		return hi
	}
	return x
}

pub fn ClampInt(x:i32, lo:i32, hi :i32)->i32 {
	if x < lo {
		return lo
	}
	if x > hi {
		return hi
	}
	return x
}
