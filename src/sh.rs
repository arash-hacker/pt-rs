// use crate::material::*;
// use crate::vector::*;
// use crate::shape::*;
// use crate::mesh::*;
// use crate::mc::*;
// use crate::ray::*;
// use crate::hit::*;
// use crate::bbox::*;

// pub struct SphericalHarmonic {
// 	pub PositiveMaterial :Material,
// 	pub NegativeMaterial :Material,
// 	pub harmonicFunction :fn(Vector)-> f64,
// 	pub mesh             :Mesh,
// }

// pub fn NewSphericalHarmonic(l:i32, m: i32, pm:Material, nm: Material)-> Box<dyn Shape> {
// 	let mut sh = SphericalHarmonic::Default();
// 	sh.PositiveMaterial = pm;
// 	sh.NegativeMaterial = nm;
// 	sh.harmonicFunction = shFunc(l, m);
// 	sh.mesh = NewSDFMesh(sh, sh.BoundingBox(), 0.01);
// 	return sh
// }


// impl SphericalHarmonic{

// 	pub fn Default()->SphericalHarmonic{
// 		return SphericalHarmonic{
// 			PositiveMaterial:None,
// 			NegativeMaterial:None,
// 			harmonicFunction:None,
// 			mesh:None,
// 		}
// 	}
// 	pub fn Compile(&self ) {
// 		self.mesh.Compile()
// 	}
	
// 	pub fn BoundingBox(&self ) -> BBox {
// 		const R:f64 = 1.0;
// 		return BBox{Min:Vector{X:-R,Y: -R,Z: -R},Max: Vector{X:R,Y: R,Z: R}};
// 	}
	
// 	pub fn Intersect(&self, r :Ray)-> Hit {
// 		let hit = self.mesh.Intersect(r);
// 		if !hit.Ok() {
// 			return NoHit;
// 		}
// 		// TODO: refine T value
// 		return Hit{Shape:Some(*self), T:hit.T,HitInfo: None}
// 	}
	
// 	pub fn UV(&self, p: Vector) -> Vector {
// 		return Vector::Default();
// 	}
	
// 	pub fn MaterialAt(&self, p :Vector) -> Material {
// 		let h = self.EvaluateHarmonic(p);
// 		if h < 0 {
// 			return self.NegativeMaterial;
// 		} else {
// 			return self.PositiveMaterial;
// 		}
// 	}
	
// 	pub fn NormalAt(&self, p :Vector)-> Vector {
// 		const e:f64 = 0.0001;
// 		let (x, y, z) = (p.X, p.Y, p.Z);
// 		let n = Vector{
// 			X:self.Evaluate(Vector{X:x - e,Y: y,Z: z}) - self.Evaluate(Vector{X:x + e,Y: y,Z: z}),
// 			Y:self.Evaluate(Vector{X:x,Y: y - e,Z: z}) - self.Evaluate(Vector{X:x, Y:y + e, Z:z}),
// 			Z:self.Evaluate(Vector{X:x,Y: y, Z:z - e}) - self.Evaluate(Vector{X:x, Y:y, Z:z + e}),
// 		};
// 		return n.Normalize()
// 	}
	
// 	pub fn EvaluateHarmonic(&self, p :Vector)-> f64 {
// 		return self.harmonicFunction(p.Normalize())
// 	}
	
// 	pub fn Evaluate(&self,p :Vector) ->f64 {
// 		return p.Length() -f64::abs(self.harmonicFunction(p.Normalize()))
// 	}
	
// }



// pub fn sh00(d: Vector) ->f64 {
// 	// 0.5 * sqrt(1/pi)
// 	return 0.282095
// }

// pub fn sh1n1(d: Vector) ->f64 {
// 	// -sqrt(3/(4pi)) * y
// 	return -0.488603 * d.Y
// }

// pub fn sh10(d: Vector) ->f64 {
// 	// sqrt(3/(4pi)) * z
// 	return 0.488603 * d.Z
// }

// pub fn sh1p1(d: Vector) ->f64 {
// 	// -sqrt(3/(4pi)) * x
// 	return -0.488603 * d.X
// }

// pub fn sh2n2(d: Vector) ->f64 {
// 	// 0.5 * sqrt(15/pi) * x * y
// 	return 1.092548 * d.X * d.Y
// }

// pub fn sh2n1(d: Vector) ->f64 {
// 	// -0.5 * sqrt(15/pi) * y * z
// 	return -1.092548 * d.Y * d.Z
// }

// pub fn sh20(d: Vector) ->f64 {
// 	// 0.25 * sqrt(5/pi) * (-x^2-y^2+2z^2)
// 	return 0.315392 * (-d.X*d.X - d.Y*d.Y + 2.0*d.Z*d.Z)
// }

// pub fn sh2p1(d: Vector) ->f64 {
// 	// -0.5 * sqrt(15/pi) * x * z
// 	return -1.092548 * d.X * d.Z
// }

// pub fn sh2p2(d: Vector) ->f64 {
// 	// 0.25 * sqrt(15/pi) * (x^2 - y^2)
// 	return 0.546274 * (d.X*d.X - d.Y*d.Y)
// }

// pub fn sh3n3(d: Vector) ->f64 {
// 	// -0.25 * sqrt(35/(2pi)) * y * (3x^2 - y^2)
// 	return -0.590044 * d.Y * (3.0*d.X*d.X - d.Y*d.Y)
// }

// pub fn sh3n2(d: Vector) ->f64 {
// 	// 0.5 * sqrt(105/pi) * x * y * z
// 	return 2.890611 * d.X * d.Y * d.Z
// }

// pub fn sh3n1(d: Vector) ->f64 {
// 	// -0.25 * sqrt(21/(2pi)) * y * (4z^2-x^2-y^2)
// 	return -0.457046 * d.Y * (4.0*d.Z*d.Z - d.X*d.X - d.Y*d.Y)
// }

// pub fn sh30(d: Vector) ->f64 {
// 	// 0.25 * sqrt(7/pi) * z * (2z^2 - 3x^2 - 3y^2)
// 	return 0.373176 * d.Z * (2.0*d.Z*d.Z - 3.0*d.X*d.X - 3.0*d.Y*d.Y)
// }

// pub fn sh3p1(d: Vector) ->f64 {
// 	// -0.25 * sqrt(21/(2pi)) * x * (4z^2-x^2-y^2)
// 	return -0.457046 * d.X * (4.0*d.Z*d.Z - d.X*d.X - d.Y*d.Y)
// }

// pub fn sh3p2(d: Vector) ->f64 {
// 	// 0.25 * sqrt(105/pi) * z * (x^2 - y^2)
// 	return 1.445306 * d.Z * (d.X*d.X - d.Y*d.Y)
// }

// pub fn sh3p3(d: Vector) ->f64 {
// 	// -0.25 * sqrt(35/(2pi)) * x * (x^2-3y^2)
// 	return -0.590044 * d.X * (d.X*d.X - 3.0*d.Y*d.Y)
// }

// pub fn sh4n4(d: Vector) ->f64 {
// 	// 0.75 * sqrt(35/pi) * x * y * (x^2-y^2)
// 	return 2.503343 * d.X * d.Y * (d.X*d.X - d.Y*d.Y)
// }

// pub fn sh4n3(d: Vector) ->f64 {
// 	// -0.75 * sqrt(35/(2pi)) * y * z * (3x^2-y^2)
// 	return -1.770131 * d.Y * d.Z * (3.0*d.X*d.X - d.Y*d.Y)
// }

// pub fn sh4n2(d: Vector) ->f64 {
// 	// 0.75 * sqrt(5/pi) * x * y * (7z^2-1)
// 	return 0.946175 * d.X * d.Y * (7.0*d.Z*d.Z - 1.0)
// }

// pub fn sh4n1(d: Vector) ->f64 {
// 	// -0.75 * sqrt(5/(2pi)) * y * z * (7z^2-3)
// 	return -0.669047 * d.Y * d.Z * (7.0*d.Z*d.Z - 3.0)
// }

// pub fn sh40(d: Vector) ->f64 {
// 	// 3/16 * sqrt(1/pi) * (35z^4-30z^2+3)
// 	let z2 = d.Z * d.Z;
// 	return 0.105786 * (35.0*z2*z2 - 30.0*z2 + 3.0)
// }

// pub fn sh4p1(d: Vector) ->f64 {
// 	// -0.75 * sqrt(5/(2pi)) * x * z * (7z^2-3)
// 	return -0.669047 * d.X * d.Z * (7.0*d.Z*d.Z - 3.0)
// }

// pub fn sh4p2(d: Vector) ->f64 {
// 	// 3/8 * sqrt(5/pi) * (x^2 - y^2) * (7z^2 - 1)
// 	return 0.473087 * (d.X*d.X - d.Y*d.Y) * (7.0*d.Z*d.Z - 1.0)
// }

// pub fn sh4p3(d: Vector) ->f64 {
// 	// -0.75 * sqrt(35/(2pi)) * x * z * (x^2 - 3y^2)
// 	return -1.770131 * d.X * d.Z * (d.X*d.X - 3.0*d.Y*d.Y)
// }

// pub fn sh4p4(d: Vector) ->f64 {
// 	// 3/16*sqrt(35/pi) * (x^2 * (x^2 - 3y^2) - y^2 * (3x^2 - y^2))
// 	let x2 = d.X * d.X;
// 	let y2 = d.Y * d.Y;
// 	return 0.625836 * (x2*(x2-3.0*y2) - y2*(3.0*x2-y2))
// }

// pub fn shFunc(l:i32, m:i32) -> Fn(Vector)-> f64 {
// 	let f : Fn(Vector) ->f64;
// 	if l == 0 && m == 0 {
// 		f = sh00
// 	} else if l == 1 && m == -1 {
// 		f = sh1n1
// 	} else if l == 1 && m == 0 {
// 		f = sh10
// 	} else if l == 1 && m == 1 {
// 		f = sh1p1
// 	} else if l == 2 && m == -2 {
// 		f = sh2n2
// 	} else if l == 2 && m == -1 {
// 		f = sh2n1
// 	} else if l == 2 && m == 0 {
// 		f = sh20
// 	} else if l == 2 && m == 1 {
// 		f = sh2p1
// 	} else if l == 2 && m == 2 {
// 		f = sh2p2
// 	} else if l == 3 && m == -3 {
// 		f = sh3n3
// 	} else if l == 3 && m == -2 {
// 		f = sh3n2
// 	} else if l == 3 && m == -1 {
// 		f = sh3n1
// 	} else if l == 3 && m == 0 {
// 		f = sh30
// 	} else if l == 3 && m == 1 {
// 		f = sh3p1
// 	} else if l == 3 && m == 2 {
// 		f = sh3p2
// 	} else if l == 3 && m == 3 {
// 		f = sh3p3
// 	} else if l == 4 && m == -4 {
// 		f = sh4n4
// 	} else if l == 4 && m == -3 {
// 		f = sh4n3
// 	} else if l == 4 && m == -2 {
// 		f = sh4n2
// 	} else if l == 4 && m == -1 {
// 		f = sh4n1
// 	} else if l == 4 && m == 0 {
// 		f = sh40
// 	} else if l == 4 && m == 1 {
// 		f = sh4p1
// 	} else if l == 4 && m == 2 {
// 		f = sh4p2
// 	} else if l == 4 && m == 3 {
// 		f = sh4p3
// 	} else if l == 4 && m == 4 {
// 		f = sh4p4
// 	} else {
// 		panic!("unsupported spherical harmonic")
// 	}
// 	return f
// }
