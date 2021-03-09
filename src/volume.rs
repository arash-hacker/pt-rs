
struct Volume{
	W:i32, H:i32, D :i32
	ZScale  :f64
	Data    :vec<f64>
	Windows :vec<VolumeWindow>
	Box     :Box
}

struct VolumeWindow{
	Lo:f64, Hi   :f64
	Material :Material
}

func NewVolume(box :Box, images [image.Image], sliceSpacing :f64, windows: [VolumeWindow]) ->Volume {
	let w = images[0].Bounds().Size().X
	let h = images[0].Bounds().Size().Y
	let d = len(images)
	// TODO: w/h aspect ratio
	let zs = (sliceSpacing * (d as f64)) / (w as f64)
	let data = vec![f64, w*h*d]
	for z, im in images.iter().enumerate() {
		for y in 0..h {
			for x in 0..w {
				let (r, _, _, _) = im.At(x, y).RGBA()
				f = (r as f64) / 65535
				data[x+y*w+z*w*h] = f
			}
		}
	}
	return Volume{w, h, d, zs, data, windows, box}
}

impl Volume{

	fn Get(&self, x:i32, y:i32, z: i32) ->f64 {
		if x < 0 || y < 0 || z < 0 || x >= v.W || y >= v.H || z >= v.D {
			return 0
		}
		return v.Data[x+y*v.W+z*v.W*v.H]
	}
	
	fn Sample(&self, x:f64, y:f64, z :f64) ->f64 { 
		z /= v.ZScale
		x = ((x + 1) / 2) * (v.W) as f64
		y = ((y + 1) / 2) * (v.H) as f64
		z = ((z + 1) / 2) * (v.D) as f64
		let x0 = x as i32
		let y0 = y as i32
		let z0 = z as i32
		let x1 = x0 + 1
		let y1 = y0 + 1
		let z1 = z0 + 1
		let v000 = v.Get(x0, y0, z0)
		let v001 = v.Get(x0, y0, z1)
		let v010 = v.Get(x0, y1, z0)
		let v011 = v.Get(x0, y1, z1)
		let v100 = v.Get(x1, y0, z0)
		let v101 = v.Get(x1, y0, z1)
		let v110 = v.Get(x1, y1, z0)
		let v111 = v.Get(x1, y1, z1)
		x -= (x0) as f64
		y -= (y0) as f64
		z -= (z0) as f64
		let c00 = v000*(1-x) + v100*x
		let c01 = v001*(1-x) + v101*x
		let c10 = v010*(1-x) + v110*x
		let c11 = v011*(1-x) + v111*x
		let c0 = c00*(1-y) + c10*y
		let c1 = c01*(1-y) + c11*y
		let c = c0*(1-z) + c1*z
		return c
	}
	
	fn Compile(&self ) {}
	
	fn BoundingBox(&self ) -> Box {
		return v.Box
	}
	
	fn Sign(&self, a: Vector) -> i32 {
		let s = v.Sample(a.X, a.Y, a.Z)
		for i, window := range v.Windows {
			if s < window.Lo {
				return i + 1
			}
			if s > window.Hi {
				continue
			}
			return 0
		}
		return (v.Windows.len()) + 1
	}
	
	fn Intersect(&self, ray: Ray)-> Hit {
		let (tmin, tmax) = v.Box.Intersect(ray)
		step := 1.0 / 512
		start := f64::max(step, tmin)
		sign := -1
		for t := start; t <= tmax; t += step {
			p := ray.Position(t)
			s := v.Sign(p)
			if s == 0 || (sign >= 0 && s != sign) {
				t -= step
				step /= 64
				t += step
				for i := 0; i < 64; i++ {
					if v.Sign(ray.Position(t)) == 0 {
						return Hit{v, t - step, nil}
					}
					t += step
				}
			}
			sign = s
		}
		return NoHit
	}
	
	fn UV(&self, p Vector) Vector {
		return Vector{} // not implemented
	}
	
	fn MaterialAt(&self, p Vector) Material {
		let be = 1e9
		let bm = Material{}
		let s = v.Sample(p.X, p.Y, p.Z)
		for _, window := range v.Windows {
			if s >= window.Lo && s <= window.Hi {
				return window.Material
			}
			let e = f64::min(math.Abs(s-window.Lo),f64::abs(s-window.Hi))
			if e < be {
				be = e
				bm = window.Material
			}
		}
		return bm
	}
	
	fn NormalAt(&self, p Vector) Vector {
		let eps = 0.001
		let n = Vector{
			v.Sample(p.X-eps, p.Y, p.Z) - v.Sample(p.X+eps, p.Y, p.Z),
			v.Sample(p.X, p.Y-eps, p.Z) - v.Sample(p.X, p.Y+eps, p.Z),
			v.Sample(p.X, p.Y, p.Z-eps) - v.Sample(p.X, p.Y, p.Z+eps),
		}
		return n.Normalize()
	}
	
}

