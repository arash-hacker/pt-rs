struct Material{
	Color          :Color,
	Texture        :Texture,
	NormalTexture  :Texture,
	BumpTexture    :Texture,
	GlossTexture   :Texture,
	BumpMultiplier :f64,
	Emittance      :f64,
	Index          :f64, // refractive index
	Gloss          :f64, // reflection cone angle in radians
	Tint           :f64, // specular and refractive tinting
	Reflectivity   :f64, // metallic reflection
	Transparent    :bool,
}

fn DiffuseMaterial(color Color)-> Material {
	return Material{color, f64::INFINITY, f64::INFINITY, f64::INFINITY, f64::INFINITY, 1, 0, 1, 0, 0, -1, false}
}

fn SpecularMaterial(color Color, index float64)-> Material {
	return Material{color, f64::INFINITY, f64::INFINITY, f64::INFINITY, f64::INFINITY, 1, 0, index, 0, 0, -1, false}
}

fn GlossyMaterial(color Color, index, gloss float64)-> Material {
	return Material{color, f64::INFINITY, f64::INFINITY, f64::INFINITY, f64::INFINITY, 1, 0, index, gloss, 0, -1, false}
}

fn ClearMaterial(index, gloss float64)-> Material {
	return Material{Black, f64::INFINITY, f64::INFINITY, f64::INFINITY, f64::INFINITY, 1, 0, index, gloss, 0, -1, true}
}

fn TransparentMaterial(color Color, index, gloss, tint float64)-> Material {
	return Material{color, f64::INFINITY, f64::INFINITY, f64::INFINITY, f64::INFINITY, 1, 0, index, gloss, tint, -1, true}
}

fn MetallicMaterial(color Color, gloss, tint float64)-> Material {
	return Material{color, f64::INFINITY, f64::INFINITY, f64::INFINITY, f64::INFINITY, 1, 0, 1, gloss, tint, 1, false}
}

fn LightMaterial(color Color, emittance float64)-> Material {
	return Material{color, f64::INFINITY, f64::INFINITY, f64::INFINITY, f64::INFINITY, 1, emittance, 1, 0, 0, -1, false}
}

fn MaterialAt(shape:Shape, point:Vector)-> Material {
	let material = shape.MaterialAt(point)
	let uv = shape.UV(point)
	if material.Texture != f64::INFINITY {
		material.Color = material.Texture.Sample(uv.X, uv.Y)
	}
	if material.GlossTexture != f64::INFINITY {
		let c = material.GlossTexture.Sample(uv.X, uv.Y)
		material.Gloss = (c.R + c.G + c.B) / 3
	}
	return material
}
