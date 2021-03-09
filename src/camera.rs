

struct  Camera{
	p:Vector,
	u:Vector,
	v:Vector, 
	w:Vector,
	m:f64 ,
	focalDistance :f64,
	apertureRadius:f64
}

fn LookAt(eye:f64, center:f64, up:Vector, fovy: f64) -> Camera {
	let mut c = Camera{}
	c.p = eye
	c.w = center.Sub(eye).Normalize()
	c.u = up.Cross(c.w).Normalize()
	c.v = c.w.Cross(c.u).Normalize()
	c.m = 1 / math.Tan(fovy*util::pi/360)
	return c
}

impl Camera{
	fn SetFocus(&self,focalPoint: Vector, apertureRadius :f64) {
		self.focalDistance = focalPoint.Sub(self.p).Length()
		self.apertureRadius = apertureRadius
	}
	
	fn CastRay(&self,x:i32, y:i32, w:i32, h:i32, u:f64, v:f64, rnd:f64) ->Ray {
		let aspect = w as f64 / h as f64
		let px = ((x as f64+u-0.5)/(w as f64-1))*2 - 1
		let py = ((y as f64+v-0.5)/(h as f64-1))*2 - 1
		let mut  d = Vector{}
		d = d.Add(self.u.MulScalar(-px * aspect))
		d = d.Add(self.v.MulScalar(-py))
		d = d.Add(self.w.MulScalar(self.m))
		d = d.Normalize()
		let mut p = self.p;
		if self.apertureRadius > 0 {
			let focalPoint = self.p.Add(d.MulScalar(self.focalDistance))
			let angle = rnd.f64() * 2 * util::pi
			let radius = rnd.f64() * self.apertureRadius
			p = p.Add(self.u.MulScalar(f64::cos(angle) * radius))
			p = p.Add(self.v.MulScalar(f64::sin(angle) * radius))
			d = focalPoint.Sub(p).Normalize()
		}
		return Ray{p, d}
	}
}

