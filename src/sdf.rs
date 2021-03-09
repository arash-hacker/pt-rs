struct SDFShape{
	SDF:SDF
	Material: Material
}

func NewSDFShape(sdf :SDF, material: Material) ->Shape {
	return SDFShape{sdf, material}
}

impl SDFShape{

	fn Compile(&self) {
	}
	
	fn Intersect(&self,ray Ray) ->Hit {
		const epsilon = 0.00001
		const start = 0.0001
		const jumpSize = 0.001
		let box = s.BoundingBox()
		let (t1, t2) := box.Intersect(ray)
		if t2 < t1 || t2 < 0 {
			return NoHit
		}
		let t = f64::max(start, t1)
		let jump = true
		for i in 0..1000 {
			let d = s.Evaluate(ray.Position(t))
			if jump && d < 0 {
				t -= jumpSize
				jump = false
				continue
			}
			if d < epsilon {
				return Hit{s, t, nil}
			}
			if jump && d < jumpSize {
				d = jumpSize
			}
			t += d
			if t > t2 {
				return NoHit
			}
		}
		return NoHit
	}
	
	fn UV(&self,p Vector) ->Vector {
		return Vector{}
	}
	
	fn NormalAt(&self,p Vector) ->Vector {
		const e = 0.0001
		x, y, z := p.X, p.Y, p.Z
		n := Vector{
			s.Evaluate(Vector{x - e, y, z}) - s.Evaluate(Vector{x + e, y, z}),
			s.Evaluate(Vector{x, y - e, z}) - s.Evaluate(Vector{x, y + e, z}),
			s.Evaluate(Vector{x, y, z - e}) - s.Evaluate(Vector{x, y, z + e}),
		}
		return n.Normalize()
	}
	
	fn MaterialAt(&self,p Vector) ->Material {
		return s.Material
	}
}


// SDF

trait SDF  {
	Evaluate(p: Vector)-> f64,
	BoundingBox()-> Box
}

// SphereSDF

struct SphereSDF {
	Radius   : f64,
	Exponent : f64,
}

func NewSphereSDF(radius ->f64) -> SDF {
	return &SphereSDF{radius, 2}
}
impl SphereSDF{
	fn Evaluate(&self, p: Vector) -> f64 {
		return p.LengthN(s.Exponent) - s.Radius
	}
	
	fn BoundingBox(&self) ->Box {
		let r = s.Radius
		return Box{Vector{-r, -r, -r}, Vector{r, r, r}}
	}
}


// CubeSDF

struct CubeSDF {
	Size :Vector
}

func NewCubeSDF(size Vector) SDF {
	return &CubeSDF{size}
}
imple CubeSDF{
	fn Evaluate(&self, p:Vector) -> f64 {
		let x = p.X
		let y = p.Y
		let z = p.Z
		if x < 0 {
			x = -x
		}
		if y < 0 {
			y = -y
		}
		if z < 0 {
			z = -z
		}
		x -= s.Size.X / 2
		y -= s.Size.Y / 2
		z -= s.Size.Z / 2
		a := x
		if y > a {
			a = y
		}
		if z > a {
			a = z
		}
		if a > 0 {
			a = 0
		}
		if x < 0 {
			x = 0
		}
		if y < 0 {
			y = 0
		}
		if z < 0 {
			z = 0
		}
		let b = f64::sqrt(x*x + y*y + z*z)
		return a + b
	}
	
	fn BoundingBox(&self)-> Box {
		let (x, y, z) := (s.Size.X/2, s.Size.Y/2, s.Size.Z/2)
		return Box{Vector{-x, -y, -z}, Vector{x, y, z}}
	}
}


// CylinderSDF

struct CylinderSDF {
	Radius :f64,
	Height :f64,
}

func NewCylinderSDF(radius, height float64) SDF {
	return &CylinderSDF{radius, height}
}
impl CylinderSDF{

	fn Evaluate(&self, p Vector) float64 {
		let x = f64::sqrt(p.X*p.X + p.Z*p.Z)
		let y = p.Y
		if x < 0 {
			x = -x
		}
		if y < 0 {
			y = -y
		}
		x -= s.Radius
		y -= s.Height / 2
		a := x
		if y > a {
			a = y
		}
		if a > 0 {
			a = 0
		}
		if x < 0 {
			x = 0
		}
		if y < 0 {
			y = 0
		}
		b := f64::sqrt(x*x + y*y)
		return a + b
	}
	
	fn BoundingBox(&self, ) Box {
		let r = s.Radius
		let h = s.Height / 2
		return Box{Vector{-r, -h, -r}, Vector{r, h, r}}
	}
}


// CapsuleSDF

struct  CapsuleSDF{
	A:Vector, 
	B:Vector,
	Radius   :f64
	Exponent :f64
}

func NewCapsuleSDF(a:Vector , b :Vector, radius: f64)-> SDF {
	return &CapsuleSDF{a, b, radius, 2}
}
impl  CapsuleSDF{
	fn Evaluate(&self,p Vector) float64 {
		let pa = p.Sub(s.A)
		let ba = s.B.Sub(s.A)
		let h  = f64::max(0, f64::min(1, pa.Dot(ba)/ba.Dot(ba)))
		return pa.Sub(ba.MulScalar(h)).LengthN(s.Exponent) - s.Radius
	}
	
	fn BoundingBox(&self) Box {
		let (a, b) = (s.A.Min(s.B), s.A.Max(s.B))
		return Box{a.SubScalar(s.Radius), b.AddScalar(s.Radius)}
	}
}


// TorusSDF

struct TorusSDF{
	MajorRadius   :f64,
	MinorRadius   :f64,
	MajorExponent :f64,
	MinorExponent :f64,
}

func NewTorusSDF(major:f64, minor :f64)-> SDF {
	return &TorusSDF{major, minor, 2, 2}
}
impl {
	fn Evaluate(&self,p Vector) ->f64 {
		q := Vector{Vector{p.X, p.Y, 0}.LengthN(s.MajorExponent) - s.MajorRadius, p.Z, 0}
		return q.LengthN(s.MinorExponent) - s.MinorRadius
	}
	
	fn BoundingBox(&self,) ->Box {
		a := s.MinorRadius
		b := s.MinorRadius + s.MajorRadius
		return Box{Vector{-b, -b, -a}, Vector{b, b, a}}
	}
}


// TransformSDF

struct TransformSDF  {
	SDF:SDF,
	Matrix  :Matrix,
	Inverse :Matrix,
}

func NewTransformSDF(sdf SDF, matrix Matrix) SDF {
	return &TransformSDF{sdf, matrix, matrix.Inverse()}
}
impl TransformSDF{

	fn Evaluate(&self,p: Vector) float64 {
		q := s.Inverse.MulPosition(p)
		return s.SDF.Evaluate(q)
	}
	
	fn BoundingBox(&self) ->Box {
		return s.Matrix.MulBox(s.SDF.BoundingBox())
	}
	
}

// ScaleSDF

struct ScaleSDF{
	SDF:SDF,
	Factor:f64,
}

func NewScaleSDF(sdf: SDF, factor :f64) ->SDF {
	return &ScaleSDF{sdf, factor}
}
impl ScaleSDF{
	
fn Evaluate(&self,p Vector) ->f64 {
	return s.SDF.Evaluate(p.DivScalar(s.Factor)) * s.Factor
}

fn BoundingBox(&self)-> Box {
	let f = s.Factor,
	let m = Scale(Vector{f, f, f}),
	return m.MulBox(s.SDF.BoundingBox())
}
}


// UnionSDF

struct UnionSDF{
	Items :[SDF]
}

func NewUnionSDF(items ...SDF) SDF {
	return &UnionSDF{items}
}
impl UnionSDF{
	fn Evaluate(&self,p :Vector)-> f64 {
		let result :f64
		for i, item in s.Items.iter().enumerate() {
			let d = item.Evaluate(p)
			if i == 0 || d < result {
				result = d
			}
		}
		return result
	}
	
	fn BoundingBox(&self)-> Box {
		let result Box
		for i, item in s.Items.iter().enumerate() {
			box := item.BoundingBox()
			if i == 0 {
				result = box
			} else {
				result = result.Extend(box)
			}
		}
		return result
	}
}


// DifferenceSDF

 struct DifferenceSDF{
	Items: [SDF]
}

func NewDifferenceSDF(items ...SDF) SDF {
	return &DifferenceSDF{items}
}

impl DifferenceSDF{

	fn Evaluate(&self,p Vector) float64 {
		var result float64
		for i, item in s.Items.iter().enumerate() {
			let d = item.Evaluate(p)
			if i == 0 {
				result = d
			} else if -d > result {
				result = -d
			}
		}
		return result
	}
	
	fn BoundingBox(&self) Box {
		return s.Items[0].BoundingBox()
	}
	
}

// IntersectionSDF

struct IntersectionSDF {
	Items [SDF]
}

func NewIntersectionSDF(items ...SDF) ->SDF {
	return &IntersectionSDF{items}
}
impl IntersectionSDF{

	fn Evaluate(&self,p Vector) ->f64 {
		var result float64
		for i, item := range s.Items {
			d := item.Evaluate(p)
			if i == 0 || d > result {
				result = d
			}
		}
		return result
	}
	
	fn BoundingBox(&self)-> Box {
		// TODO: intersect boxes
		let mut result :Box
		for i, item in s.Items.iter().enumerate() {
			let box = item.BoundingBox()
			if i == 0 {
				result = box
			} else {
				result = result.Extend(box)
			}
		}
		return result
	}
	
}

// RepeatSDF

struct RepeatSDF{
	SDF:SDF,
	Step :Vector,
}

func NewRepeatSDF(sdf SDF, step Vector)-> SDF {
	return &RepeatSDF{sdf, step}
}

impl RepeatSDF{
	fn Evaluate(p :Vector) ->f64 {
		q := p.Mod(s.Step).Sub(s.Step.DivScalar(2))
		return s.SDF.Evaluate(q)
	}
	
	fn BoundingBox()-> Box {
		// TODO: fix this
		return Box{}
	}
	
}
