struct Tree{
	Box  :Box,
	Root :Node,
}

fn NewTree(shapes :[Shape])->Tree {
	println("Building k-d tree (%d shapes)... ", len(shapes))
	let box = BoxForShapes(shapes)
	let node = NewNode(shapes)
	node.Split(0)
	return &Tree{box, node}
}

func (tree *Tree) Intersect(r Ray) Hit {
	tmin, tmax := tree.Box.Intersect(r)
	if tmax < tmin || tmax <= 0 {
		return NoHit
	}
	return tree.Root.Intersect(r, tmin, tmax)
}

 struct Node {
	Axis   :Axis,
	Point  :f64,
	Shapes :Vec<Shape>,
	Left   :Node,
	Right  :Node,
}

func NewNode(shapes [Shape])-> Node {
	return Node{AxisNone, 0, shapes, None, None}
}

impl Node{

	fn Intersect(&self, r :Ray, tmin:f64, tmax :f64) Hit {
		let tsplit :f64
		let leftFirst :f64
		match  node.Axis {
			 AxisNone=>{node.IntersectShapes(r)},
			 AxisX=>{
				tsplit = (node.Point - r.Origin.X) / r.Direction.X;
				leftFirst = (r.Origin.X < node.Point) || (r.Origin.X == node.Point && r.Direction.X <= 0);},
			AxisY=>{
				tsplit = (node.Point - r.Origin.Y) / r.Direction.Y;
				leftFirst = (r.Origin.Y < node.Point) || (r.Origin.Y == node.Point && r.Direction.Y <= 0);},
		 AxisZ=>{
			tsplit = (node.Point - r.Origin.Z) / r.Direction.Z;
			leftFirst = (r.Origin.Z < node.Point) || (r.Origin.Z == node.Point && r.Direction.Z <= 0);}
		}
		let first:Node
		let second:Node;
		if leftFirst {
			first = node.Left
			second = node.Right
		} else {
			first = node.Right
			second = node.Left
		}
		if tsplit > tmax || tsplit <= 0 {
			return first.Intersect(r, tmin, tmax)
		} else if tsplit < tmin {
			return second.Intersect(r, tmin, tmax)
		} else {
			let h1 = first.Intersect(r, tmin, tsplit)
			if h1.T <= tsplit {
				return h1
			}
			let h2 = second.Intersect(r, tsplit, f64::min(tmax, h1.T))
			if h1.T <= h2.T {
				return h1
			} else {
				return h2
			}
		}
	}
	
	fn IntersectShapes(&self, r:Ray)-> Hit {
		let hit = NoHit
		for _, shape := range node.Shapes {
			h := shape.Intersect(r)
			if h.T < hit.T {
				hit = h
			}
		}
		return hit
	}
	
	fn PartitionScore(&self, axis :Axis, point :f64)-> i32 {
		left, right := 0, 0
		for  shape in node.Shapes {
			let box = shape.BoundingBox()
			let (l, r) = box.Partition(axis, point)
			if l {
				left++
			}
			if r {
				right++
			}
		}
		if left >= right {
			return left
		} else {
			return right
		}
	}
	
	fn Partition(&self, size: i32, axis: Axis, point: f64)-> (left, right Vec<Shape>) {
		let left = Vec<Shape>
		let right = Vec<Shape>
		for  shape in node.Shapes {
			let box = shape.BoundingBox()
			let (l, r) = box.Partition(axis, point)
			if l {
				left.insert(shape)
			}
			if r {
				right.insert(shape)
			}
		}
		return
	}
	
	fn Split(&self, depth :i32) {
		if len(node.Shapes) < 8 {
			return
		}
		let xs = Vec<f64>
		let ys = Vec<f64>
		let zs = Vec<f64>
		for  shape in node.Shapes {
			let box = shape.BoundingBox()
			xs.insert(box.Min.X)
			xs.insert(box.Max.X)
			ys.insert(box.Min.Y)
			ys.insert(box.Max.Y)
			zs.insert(box.Min.Z)
			zs.insert(box.Max.Z)
		}
		sort.Float64s(xs)
		sort.Float64s(ys)
		sort.Float64s(zs)
		let (mx, my, mz) = (Median(xs), Median(ys), Median(zs))
		let mut best = int(float64(len(node.Shapes)) * 0.85)
		let mut bestAxis = AxisNone
		let mut bestPoint = 0.0
		let mut sx = node.PartitionScore(AxisX, mx)
		if sx < best {
			best = sx
			bestAxis = AxisX
			bestPoint = mx
		}
		let sy = node.PartitionScore(AxisY, my)
		if sy < best {
			best = sy
			bestAxis = AxisY
			bestPoint = my
		}
		let sz = node.PartitionScore(AxisZ, mz)
		if sz < best {
			best = sz
			bestAxis = AxisZ
			bestPoint = mz
		}
		if bestAxis == AxisNone {
			return
		}
		let (l, r) = node.Partition(best, bestAxis, bestPoint)
		node.Axis = bestAxis
		node.Point = bestPoint
		node.Left = NewNode(l)
		node.Right = NewNode(r)
		node.Left.Split(depth + 1)
		node.Right.Split(depth + 1)
		node.Shapes = None 
	}
	
}
