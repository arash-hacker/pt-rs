
struct poissonGrid{
	r:f64, 
	size :f64,
	cells :HashMap<Vector,Vector>,  
}

fn newPoissonGrid(r :f64)-> poissonGrid {
	let size = r / f64::sqrt(2)
	return &poissonGrid{r, size, HashMap<Vector,Vector>}
}
impl poissonGrid{

	fn normalize(&self,v Vector) Vector {
		i := f64::floor(v.X / self.size)
		j := f64::floor(v.Y / self.size)
		return Vector{i, j, 0}
	}
	
	fn insert(&self,v:Vector) bool {
		n := self.normalize(v)
		for i := n.X - 2; i < n.X+3; i++ {
			for j := n.Y - 2; j < n.Y+3; j++ {
				if m, ok := self.cells[Vector{i, j, 0}]; ok {
					if math.Hypot(m.X-v.X, m.Y-v.Y) < self.r {
						return false
					}
				}
			}
		}
		self.cells[n] = v
		return true
	}
	
	fn PoissonDisc(&self,x1:f64, y1:f64, x2:f64, y2, r:f64, n:i32) ->[Vector] {
		let result: [Vector]
		let x := x1 + (x2-x1)/2
		let y := y1 + (y2-y1)/2
		let v := Vector{x, y, 0->}
		let active := [Vector{v}]
		let grid := newPoissonGrid(r)
		self.insert(v)
		result = append(result, v)
		for len(active) > 0 {
			index := rand.Intn(len(active))
			point := active[index]
			ok := false
			for i := 0; i < n; i++ {
				a := rand::random::<f64>() * 2 * util::pi
				d := rand::random::<f64>()*r + r
				x := point.X + f64::cos(a)*d
				y := point.Y + f64::sin(a)*d
				if x < x1 || y < y1 || x > x2 || y > y2 {
					continue
				}
				v := Vector{x, y, 0}
				if !self.insert(v) {
					continue
				}
				result = append(result, v)
				active = append(active, v)
				ok = true
				break
			}
			if !ok {
				active = append(active[:index], active[index+1:]...)
			}
		}
		return result
	}
	
}