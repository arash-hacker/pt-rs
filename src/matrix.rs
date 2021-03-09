use crate::bbox::*;
use crate::vector::*;
pub struct Matrix{
	x00:f64, x01:f64, x02:f64, x03:f64,
	x10:f64, x11:f64, x12:f64, x13:f64,
	x20:f64, x21:f64, x22:f64, x23:f64,
	x30:f64, x31:f64, x32:f64, x33:f64,
}

fn Identity()-> Matrix {
	return Matrix{
		x00:1,x01: 0,x02: 0,x03: 0,
		x10:0,x11: 1,x12: 0,x13: 0,
		x20:0,x21: 0,x22: 1,x23: 0,
		x30:0,x31: 0,x32: 0,x33: 1
	}
}

fn Translate(v:Vector) ->Matrix {
	return Matrix{
		x00:1,x01: 0,x02: 0,x03: v.X,
		x10:0,x11: 1,x12: 0,x13: v.Y,
		x20:0,x21: 0,x22: 1,x23: v.Z,
		x30:0,x31: 0,x32: 0,x33: 1}
}

fn Scale(v :Vector)-> Matrix {
	return Matrix{
		x00:v.X,x01:0,  x02:  0,x03: 0,
		x10:0,  x11:v.Y,x12:  0,x13: 0,
		x20:0,  x21:0,  x22:v.Z,x23: 0,
		x30:0,  x31:0,  x32:  0,x33: 1
	}
}

fn Rotate(&mut v :Vector, a :f64)-> Matrix {
	v = v.Normalize();
	let s = f64::sin(a);
	let c = f64::cos(a);
	let m = 1 - c;
	return Matrix{
		x00: m*v.X*v.X + c,	    x01: m*v.X*v.Y + v.Z*s, x02: m*v.Z*v.X - v.Y*s,   	x03: 0,
		x10: m*v.X*v.Y - v.Z*s, x11: m*v.Y*v.Y + c,     x12: m*v.Y*v.Z + v.X*s,   	x13:0,
		x20: m*v.Z*v.X + v.Y*s, x21:m*v.Y*v.Z - v.X*s,  x22:m*v.Z*v.Z + c,			x23: 0,
		x30: 0,					x31: 0,					x32: 0,						x33: 1,
	}
}

fn Frustum(l:f64, r:f64, b:f64, t:f64, n:f64, f:f64)-> Matrix {
	let t1 = 2 * n;
	let t2 = r - l;
	let t3 = t - b;
	let t4 = f - n;
	return Matrix{
		x00:t1 / t2,		x01:0,			x02:(r + l) / t2,			x03:0,
		x10:0,				x11:t1 / t3,	x12:(t + b) / t3,			x13:0,
		x20:0,				x21:0,			x22:(-f - n) / t4,			x23:(-t1 * f) / t4,
		x30:0,				x31:0,			x32:-1,						x33:0}
}

fn Orthographic(l:f64, r:f64, b:f64, t:f64, n:f64, f:f64)-> Matrix {
	return Matrix{
		x00:2 / (r - l), x01:0, 		   x02:0, 			x03:-(r + l) / (r - l),
		x10:0,			 x11:2 / (t - b),  x12:0, 			x13:-(t + b) / (t - b),
		x20:0,			 x21:0, 		   x22:-2 / (f - n), x23:-(f + n) / (f - n),
		x30:0,			 x31:0,			   x32:0,				x33: 1,
	}
}

fn Perspective(fovy:f64, aspect:f64, near:f64, far: f64)-> Matrix {
	let ymax = near * math.Tan(fovy*util::pi/360);
	let xmax = ymax * aspect;
	return Frustum(-xmax, xmax, -ymax, ymax, near, far)
}

fn LookAtMatrix(eye: Vector, center: Vector, up: Vector) -> Matrix {
	let up = up.Normalize();
	let f = center.Sub(eye).Normalize();
	let s = f.Cross(up).Normalize();
	let u = s.Cross(f);
	let m = Matrix{
		x00:s.X,x01: u.X,x02: f.X,x03: 0,
		x10:s.Y,x11: u.Y,x12: f.Y,x13: 0,
		x20:s.Z,x21: u.Z,x22: f.Z,x23: 0,
		x30:0, 	x31: 0, x32:  0, x33:  1,
	};
	return m.Transpose().Inverse().Translate(eye)
}
impl Matrix {
	fn Translate(v :Vector)-> Matrix {
		return Translate(v).Mul(self)
	}
	
	fn Scale(&self, v :Vector)-> Matrix {
		return Scale(v).Mul(self)
	}
	
	fn Rotate(&self, v :Vector, a: f64) ->Matrix {
		return Rotate(v, a).Mul(self)
	}
	
	fn Frustum(&self, l:f64, r:f64, b:f64, t:f64, n:f64, f :f64) ->Matrix {
		return Frustum(l, r, b, t, n, f).Mul(self)
	}
	
	fn Orthographic(&self, l:f64, r:f64, b:f64, t:f64, n:f64, f:f64)-> Matrix {
		return Orthographic(l, r, b, t, n, f).Mul(self)
	}
	
	fn Perspective(&self, fovy:f64, aspect:f64, near:f64, far:f64)-> Matrix {
		return Perspective(fovy, aspect, near, far).Mul(self)
	}
	
	fn Mul(b :Matrix) ->Matrix {
		let mut m = Matrix::Default();
		m.x00 = self.x00*b.x00 + self.x01*b.x10 + self.x02*b.x20 + self.x03*b.x30;
		m.x10 = self.x10*b.x00 + self.x11*b.x10 + self.x12*b.x20 + self.x13*b.x30;
		m.x20 = self.x20*b.x00 + self.x21*b.x10 + self.x22*b.x20 + self.x23*b.x30;
		m.x30 = self.x30*b.x00 + self.x31*b.x10 + self.x32*b.x20 + self.x33*b.x30;
		m.x01 = self.x00*b.x01 + self.x01*b.x11 + self.x02*b.x21 + self.x03*b.x31;
		m.x11 = self.x10*b.x01 + self.x11*b.x11 + self.x12*b.x21 + self.x13*b.x31;
		m.x21 = self.x20*b.x01 + self.x21*b.x11 + self.x22*b.x21 + self.x23*b.x31;
		m.x31 = self.x30*b.x01 + self.x31*b.x11 + self.x32*b.x21 + self.x33*b.x31;
		m.x02 = self.x00*b.x02 + self.x01*b.x12 + self.x02*b.x22 + self.x03*b.x32;
		m.x12 = self.x10*b.x02 + self.x11*b.x12 + self.x12*b.x22 + self.x13*b.x32;
		m.x22 = self.x20*b.x02 + self.x21*b.x12 + self.x22*b.x22 + self.x23*b.x32;
		m.x32 = self.x30*b.x02 + self.x31*b.x12 + self.x32*b.x22 + self.x33*b.x32;
		m.x03 = self.x00*b.x03 + self.x01*b.x13 + self.x02*b.x23 + self.x03*b.x33;
		m.x13 = self.x10*b.x03 + self.x11*b.x13 + self.x12*b.x23 + self.x13*b.x33;
		m.x23 = self.x20*b.x03 + self.x21*b.x13 + self.x22*b.x23 + self.x23*b.x33;
		m.x33 = self.x30*b.x03 + self.x31*b.x13 + self.x32*b.x23 + self.x33*b.x33;
		return m
	}
	
	fn MulPosition(b :Vector) ->Vector {
		let x = self.x00*b.X + self.x01*b.Y + self.x02*b.Z + self.x03;
		let y = self.x10*b.X + self.x11*b.Y + self.x12*b.Z + self.x13;
		let z = self.x20*b.X + self.x21*b.Y + self.x22*b.Z + self.x23;
		return Vector{X:x, Y:y, Z:z}
	}
	
	fn MulDirection(b :Vector) ->Vector {
		let x = self.x00*b.X + self.x01*b.Y + self.x02*b.Z;
		let y = self.x10*b.X + self.x11*b.Y + self.x12*b.Z;
		let z = self.x20*b.X + self.x21*b.Y + self.x22*b.Z;
		return Vector{X:x, Y:y, Z:z}.Normalize()
	}
	
	fn MulRay(&self, b :Ray)-> Ray {
		return Ray{Origin:self.MulPosition(b.Origin),Direction: self.MulDirection(b.Direction)}
	}
	
	fn  MulBox(&self, bx:BBox)-> BBox {
		// http://dev.theomader.com/transform-bounding-boxes/
		let mut  r = Vector{X:self.x00, Y:self.x10,Z: self.x20};
		let mut  u = Vector{X:self.x01, Y:self.x11,Z: self.x21};
		let mut  b = Vector{X:self.x02, Y:self.x12,Z: self.x22};
		let mut  t = Vector{X:self.x03, Y:self.x13,Z: self.x23};
		let  mut xa = r.MulScalar(bx.Min.X);
		let  mut xb = r.MulScalar(bx.Max.X);
		let  mut ya = u.MulScalar(bx.Min.Y);
		let  mut yb = u.MulScalar(bx.Max.Y);
		let  mut za = b.MulScalar(bx.Min.Z);
		let  mut zb = b.MulScalar(bx.Max.Z);
		let (xa, xb) = (x.Min(xb), xa.Max(xb));
		let (ya, yb) = (ya.Min(yb), ya.Max(yb));
		let (za, zb) = (za.Min(zb), za.Max(zb));
		let min = xa.Add(ya).Add(za).Add(t);
		let max = xb.Add(yb).Add(zb).Add(t);
		return BBox{Min:min,Max: max};
	}
	
	fn Transpose(&self)-> Matrix {
		return Matrix{
			x00:self.x00,x01: self.x10,x02: self.x20,x03: self.x30,
			x10:self.x01,x11: self.x11,x12: self.x21,x13: self.x31,
			x20:self.x02,x21: self.x12,x22: self.x22,x23: self.x32,
			x30:self.x03,x31: self.x13,x32: self.x23,x33: self.x33}
	}
	
	fn Determinant(&self) ->f64 {
		return (
			self.x00*self.x11*self.x22*self.x33 - self.x00*self.x11*self.x23*self.x32 +
			self.x00*self.x12*self.x23*self.x31 - self.x00*self.x12*self.x21*self.x33 +
			self.x00*self.x13*self.x21*self.x32 - self.x00*self.x13*self.x22*self.x31 -
			self.x01*self.x12*self.x23*self.x30 + self.x01*self.x12*self.x20*self.x33 -
			self.x01*self.x13*self.x20*self.x32 + self.x01*self.x13*self.x22*self.x30 -
			self.x01*self.x10*self.x22*self.x33 + self.x01*self.x10*self.x23*self.x32 +
			self.x02*self.x13*self.x20*self.x31 - self.x02*self.x13*self.x21*self.x30 +
			self.x02*self.x10*self.x21*self.x33 - self.x02*self.x10*self.x23*self.x31 +
			self.x02*self.x11*self.x23*self.x30 - self.x02*self.x11*self.x20*self.x33 -
			self.x03*self.x10*self.x21*self.x32 + self.x03*self.x10*self.x22*self.x31 -
			self.x03*self.x11*self.x22*self.x30 + self.x03*self.x11*self.x20*self.x32 -
			self.x03*self.x12*self.x20*self.x31 + self.x03*self.x12*self.x21*self.x30)
	}
	
	fn Inverse(&self) ->Matrix {
		let mut m = Matrix::Default();
		let d = self.Determinant();
		m.x00 = (self.x12*self.x23*self.x31 - self.x13*self.x22*self.x31 + self.x13*self.x21*self.x32 - self.x11*self.x23*self.x32 - self.x12*self.x21*self.x33 + self.x11*self.x22*self.x33) / d;
		m.x01 = (self.x03*self.x22*self.x31 - self.x02*self.x23*self.x31 - self.x03*self.x21*self.x32 + self.x01*self.x23*self.x32 + self.x02*self.x21*self.x33 - self.x01*self.x22*self.x33) / d;
		m.x02 = (self.x02*self.x13*self.x31 - self.x03*self.x12*self.x31 + self.x03*self.x11*self.x32 - self.x01*self.x13*self.x32 - self.x02*self.x11*self.x33 + self.x01*self.x12*self.x33) / d;
		m.x03 = (self.x03*self.x12*self.x21 - self.x02*self.x13*self.x21 - self.x03*self.x11*self.x22 + self.x01*self.x13*self.x22 + self.x02*self.x11*self.x23 - self.x01*self.x12*self.x23) / d;
		m.x10 = (self.x13*self.x22*self.x30 - self.x12*self.x23*self.x30 - self.x13*self.x20*self.x32 + self.x10*self.x23*self.x32 + self.x12*self.x20*self.x33 - self.x10*self.x22*self.x33) / d;
		m.x11 = (self.x02*self.x23*self.x30 - self.x03*self.x22*self.x30 + self.x03*self.x20*self.x32 - self.x00*self.x23*self.x32 - self.x02*self.x20*self.x33 + self.x00*self.x22*self.x33) / d;
		m.x12 = (self.x03*self.x12*self.x30 - self.x02*self.x13*self.x30 - self.x03*self.x10*self.x32 + self.x00*self.x13*self.x32 + self.x02*self.x10*self.x33 - self.x00*self.x12*self.x33) / d;
		m.x13 = (self.x02*self.x13*self.x20 - self.x03*self.x12*self.x20 + self.x03*self.x10*self.x22 - self.x00*self.x13*self.x22 - self.x02*self.x10*self.x23 + self.x00*self.x12*self.x23) / d;
		m.x20 = (self.x11*self.x23*self.x30 - self.x13*self.x21*self.x30 + self.x13*self.x20*self.x31 - self.x10*self.x23*self.x31 - self.x11*self.x20*self.x33 + self.x10*self.x21*self.x33) / d;
		m.x21 = (self.x03*self.x21*self.x30 - self.x01*self.x23*self.x30 - self.x03*self.x20*self.x31 + self.x00*self.x23*self.x31 + self.x01*self.x20*self.x33 - self.x00*self.x21*self.x33) / d;
		m.x22 = (self.x01*self.x13*self.x30 - self.x03*self.x11*self.x30 + self.x03*self.x10*self.x31 - self.x00*self.x13*self.x31 - self.x01*self.x10*self.x33 + self.x00*self.x11*self.x33) / d;
		m.x23 = (self.x03*self.x11*self.x20 - self.x01*self.x13*self.x20 - self.x03*self.x10*self.x21 + self.x00*self.x13*self.x21 + self.x01*self.x10*self.x23 - self.x00*self.x11*self.x23) / d;
		m.x30 = (self.x12*self.x21*self.x30 - self.x11*self.x22*self.x30 - self.x12*self.x20*self.x31 + self.x10*self.x22*self.x31 + self.x11*self.x20*self.x32 - self.x10*self.x21*self.x32) / d;
		m.x31 = (self.x01*self.x22*self.x30 - self.x02*self.x21*self.x30 + self.x02*self.x20*self.x31 - self.x00*self.x22*self.x31 - self.x01*self.x20*self.x32 + self.x00*self.x21*self.x32) / d;
		m.x32 = (self.x02*self.x11*self.x30 - self.x01*self.x12*self.x30 - self.x02*self.x10*self.x31 + self.x00*self.x12*self.x31 + self.x01*self.x10*self.x32 - self.x00*self.x11*self.x32) / d;
		m.x33 = (self.x01*self.x12*self.x20 - self.x02*self.x11*self.x20 + self.x02*self.x10*self.x21 - self.x00*self.x12*self.x21 - self.x01*self.x10*self.x22 + self.x00*self.x11*self.x22) / d;
		return m
	}
}

