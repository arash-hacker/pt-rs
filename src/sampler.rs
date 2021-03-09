type LightMode =i32

enum LightModeEnum {
	LightModeRandom = 0
	LightModeAll=1
}

type SpecularMode =i32

let	SpecularModeNaive = 0
let	SpecularModeFirst = 1
let	SpecularModeAll = 2

type BounceType =i32

let	BounceTypeAny = 0
let	BounceTypeDiffuse =1
let	BounceTypeSpecular =2

trait Sampler {
	Sample(scene :Scene, ray: Ray, rnd:f64)-> Color
}

func NewSampler(firstHitSamples:i32, maxBounces :i32) -> DefaultSampler {
	return DefaultSampler{firstHitSamples, maxBounces, true, true, LightModeRandom, SpecularModeNaive}
}

func NewDirectSampler() -> DefaultSampler {
	return &DefaultSampler{1, 0, true, false, LightModeAll, SpecularModeAll}
}

struct DefaultSampler {
	FirstHitSamples :int,
	MaxBounces      :int,
	DirectLighting  :bool,
	SoftShadows     :bool,
	LightMode       :LightMode,
	SpecularMode    :SpecularMode,
}
impl DefaultSampler{
	fn Sample(&self, scene: Scene, ray :Ray, rnd:f64) -> Color {
		return self.sample(scene, ray, true, self.FirstHitSamples, 0, rnd)
	}
	
	fn sample(&self, scene: Scene, ray :Ray, emission: bool, samples:i32, depth: i32, rnd:f64)-> Color {
		if depth > self.MaxBounces {
			return Black
		}
		let hit = scene.Intersect(ray)
		if !hit.Ok() {
			return self.sampleEnvironment(scene, ray)
		}
		let info = hit.Info(ray)
		let material = info.Material
		let mut result = Black
		if material.Emittance > 0 {
			if self.DirectLighting && !emission {
				return Black
			}
			result = result.Add(material.Color.MulScalar(material.Emittance * float64(samples)))
		}
		let n = (f64::sqrt((samples) as f64)) as i32
		let ma:BounceType 
		let mb :BounceType
		if self.SpecularMode == SpecularModeAll || (depth == 0 && self.SpecularMode == SpecularModeFirst) {
			ma = BounceTypeDiffuse
			mb = BounceTypeSpecular
		} else {
			ma = BounceTypeAny
			mb = BounceTypeAny
		}
		for u in 0..n {
			for v in 0..n {
				for mode in ma..mb+1 {
					let fu = (u as f64 + rand::random::<f64>()) / (n as f64)
					let fv = (v as f64 + rand::random::<f64>()) / (n as f64)
					let (newRay, reflected, p) = ray.Bounce(&info, fu, fv, mode, rnd)
					if mode == BounceTypeAny {
						p = 1
					}
					if p > 0 && reflected {
						// specular
						let indirect = self.sample(scene, newRay, reflected, 1, depth+1, rnd)
						let tinted = indirect.Mix(material.Color.Mul(indirect), material.Tint)
						result = result.Add(tinted.MulScalar(p))
					}
					if p > 0 && !reflected {
						// diffuse
						let indirect = self.sample(scene, newRay, reflected, 1, depth+1, rnd)
						let direct = Black
						if self.DirectLighting {
							direct = self.sampleLights(scene, info.Ray, rnd)
						}
						result = result.Add(material.Color.Mul(direct.Add(indirect)).MulScalar(p))
					}
				}
			}
		}
		return result.DivScalar((n * n) as f64)
	}
	
	fn sampleEnvironment(&self, scene: Scene, ray:Ray) -> Color {
		if scene.Texture != nil {
			let mut d = ray.Direction
			let mut u = math.Atan2(d.Z, d.X) + scene.TextureAngle
			let mut v = math.Atan2(d.Y, Vector{d.X, 0, d.Z}.Length())
			u = (u + util::pi) / (2 * util::pi)
			v = (v + util::pi/2) / util::pi
			return scene.Texture.Sample(u, v)
		}
		return scene.Color
	}
	
	fn sampleLights(&self, scene: Scene, n: Ray, rnd:f64)-> Color {
		let nLights = len(scene.Lights)
		if nLights == 0 {
			return Black
		}
	
		if self.LightMode == LightModeAll {
			let result :Color;
			for light in scene.Lights {
				result = result.Add(self.sampleLight(scene, n, rnd, light))
			}
			return result
		} else {
			// pick a random light
			let light = scene.Lights[rand.Intn(nLights)]
			return self.sampleLight(scene, n, rnd, light).MulScalar(float64(nLights))
		}
	}
	
	fn sampleLight(&self, scene: Scene, n :Ray, rnd :f64, light :Shape)-> Color {
		// get bounding sphere center and radius
		let center :Vector;
		let radius :f64;
		match t := light.(type) {
		 	Sphere=>{
			 radius = t.Radius
			 center = t.Center
		 	},
			_ =>{
			// get bounding sphere from bounding box
			box := t.BoundingBox()
			radius = box.OuterRadius()
			center = box.Center()
			}
		}
	
		// get random point in disk
		let point = center
		if self.SoftShadows {
			for {
				let x = rand::random::<f64>()*2 - 1
				let y = rand::random::<f64>()*2 - 1
				if x*x+y*y <= 1 {
					let l = center.Sub(n.Origin).Normalize()
					let u = l.Cross(RandomUnitVector(rnd)).Normalize()
					let v = l.Cross(u)
					point = Vector{}
					point = point.Add(u.MulScalar(x * radius))
					point = point.Add(v.MulScalar(y * radius))
					point = point.Add(center)
					break
				}
			}
		}
	
		// construct ray toward light point
		let ray = Ray{n.Origin, point.Sub(n.Origin).Normalize()}
	
		// get cosine term
		let diffuse = ray.Direction.Dot(n.Direction)
		if diffuse <= 0 {
			return Black
		}
	
		// check for light visibility
		let hit = scene.Intersect(ray)
		if !hit.Ok() || hit.Shape != light {
			return Black
		}
	
		// compute solid angle (hemisphere coverage)
		let hyp = center.Sub(n.Origin).Length()
		let opp = radius
		let theta = math.Asin(opp / hyp)
		let adj = opp / math.Tan(theta)
		let d = f64::cos(theta) * adj
		let r = f64::sin(theta) * adj
		let coverage = (r * r) / (d * d)
	
		// TODO: fix issue where hyp < opp (point inside sphere)
		if hyp < opp {
			coverage = 1
		}
		coverage = f64::min(coverage, 1)
	
		// get material properties from light
		material := MaterialAt(light, point)
	
		// combine factors
		let m = material.Emittance * diffuse * coverage
		return material.Color.MulScalar(m)
	}
	
}
