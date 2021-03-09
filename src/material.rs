use crate::color::{*, self};
use crate::texture::*;
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

fn DiffuseMaterial(color:Color)-> Material {
	return Material{
		Color:color,
		Texture: f64::INFINITY,
		NormalTexture:  f64::INFINITY,
		BumpTexture:   f64::INFINITY,
		GlossTexture:    f64::INFINITY,
		BumpMultiplier:	 1,
		Emittance:	  0,
		Index:	   1,
		Gloss:	    0,
		Tint:		 0,
		Reflectivity:		  -1,
		Transparent:		   false,
	}
}

fn SpecularMaterial(color: Color, index: f64)-> Material {
	return Material{
		Color:color,
		Texture: f64::INFINITY,
		NormalTexture:  f64::INFINITY,
		BumpTexture:  f64::INFINITY,
		GlossTexture:  f64::INFINITY,
		BumpMultiplier:  1,
		Emittance:  0,
		Index:  index,
		Gloss:  0,
		Tint:  0,
		Reflectivity:  -1,
		Transparent:  false}
}

fn GlossyMaterial(color: Color, index:f64, gloss: f64)-> Material {
	return Material{
		Color:color,
		Texture:f64::INFINITY,
		NormalTexture:f64::INFINITY,
		BumpTexture:f64::INFINITY,
		GlossTexture:f64::INFINITY,
		BumpMultiplier:1,
		Emittance:0,
		Index:index,
		Gloss:gloss,
		Tint:0,
		Reflectivity:-1,
		Transparent:false}
}

fn ClearMaterial(index:f64, gloss:f64)-> Material {
	return Material{
		Color:Black,
		Texture:f64::INFINITY,
		NormalTexture:f64::INFINITY,
		BumpTexture:f64::INFINITY,
		GlossTexture:f64::INFINITY,
		BumpMultiplier:1,
		Emittance:0,
		Index:index,
		Gloss:gloss,
		Tint:0,
		Reflectivity:-1,
		Transparent:true}
}

fn
TransparentMaterial(color:Color,index:f64, gloss:f64, tint: f64)-> Material {
	return Material{
		Color:color,
		Texture: f64::INFINITY,
		NormalTexture: f64::INFINITY,
		BumpTexture: f64::INFINITY,
		GlossTexture: f64::INFINITY,
		BumpMultiplier: 1,
		Emittance: 0,
		Index: index,
		Gloss: gloss,
		Tint: tint,
		Reflectivity: -1,
		Transparent: true}
}

fn MetallicMaterial(color:Color, gloss:f64, tint: f64)-> Material {
	return Material{
		Color:color,
		Texture:f64::INFINITY,
		NormalTexture:f64::INFINITY,
		BumpTexture:f64::INFINITY,
		GlossTexture:f64::INFINITY,
		BumpMultiplier:1,
		Emittance:0,
		Index:1,
		Gloss:gloss,
		Tint:tint,
		Reflectivity:1,
		Transparent:false
	}
}

fn LightMaterial(color :Color, emittance: f64)-> Material {
	return Material{
		Color:color,
		Texture:f64::INFINITY,
		NormalTexture:f64::INFINITY,
		BumpTexture:f64::INFINITY,
		GlossTexture:f64::INFINITY,
		BumpMultiplier:1,
		Emittance:emittance,
		Index:1,
		Gloss:0,
		Tint:0,
		Reflectivity:-1,
		Transparent:false}
}

fn MaterialAt(shape:Shape, point:Vector)-> Material {
	let mut material = shape.MaterialAt(point);
	let uv = shape.UV(point);
	if material.Texture != f64::INFINITY {
		material.Color = material.Texture.Sample(uv.X, uv.Y);
	}
	if material.GlossTexture != f64::INFINITY {
		let c = material.GlossTexture.Sample(uv.X, uv.Y);
		material.Gloss = (c.R + c.G + c.B) / 3;
	}
	return material
}
