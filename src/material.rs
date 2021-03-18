use crate::color::{*, self};
use crate::texture::*;
use crate::shape::*;
use crate::sdf::*;
use crate::hit::*;
use crate::vector::*;
use crate::bbox::*;
use crate::triangle::*;
use crate::tree::*;

#[derive(Debug,Clone)]
pub struct Material{
	pub Color          :Option<Color>,
	pub Texture        :Option<Texture>,
	pub NormalTexture  :Option<Texture>,
	pub BumpTexture    :Option<Texture>,
	pub GlossTexture   :Option<Texture>,
	pub BumpMultiplier :Option<f64>,
	pub Emittance      :Option<f64>,
	pub Index          :Option<f64>, // refractive index
	pub Gloss          :Option<f64>, // reflection cone angle in radians
	pub Tint           :Option<f64>, // specular and refractive tinting
	pub Reflectivity   :Option<f64>, // metallic reflection
	pub Transparent    :Option<bool>,
}
impl Material{
	pub fn Default()->Material{
		return Material{
			Color          :None,
			Texture        :None,
			NormalTexture  :None,
			BumpTexture    :None,
			GlossTexture   :None,
			BumpMultiplier :None,
			Emittance      :None,
			Index          :None,
			Gloss          :None,
			Tint           :None,
			Reflectivity   :None,
			Transparent    :None,
		}
		
	}
}

pub fn DiffuseMaterial(color:Color)-> Material {
	return Material{
		Color:Some(color),
		Texture: None,
		NormalTexture:None,
		BumpTexture: None,
		GlossTexture:None,
		BumpMultiplier:Some(1.0),
		Emittance:Some(0.0),
		Index:Some(1.0),
		Gloss:Some(0.0),
		Tint:Some(0.0),
		Reflectivity:Some(-1.0),
		Transparent:Some(false),
	}
}

pub fn SpecularMaterial(color: Color, index: f64)-> Material {
	return Material{
		Color:Some(color),
		Texture: None,
		NormalTexture:  None,
		BumpTexture:  None,
		GlossTexture:  None,
		BumpMultiplier:  Some(1.0),
		Emittance:  Some(0.0),
		Index:  Some(index),
		Gloss:  Some(0.0),
		Tint:  Some(0.0),
		Reflectivity:  Some(-1.0),
		Transparent:  Some(false)}
}

pub fn GlossyMaterial(color: Color, index:f64, gloss: f64)-> Material {
	return Material{
		Color:Some(color),
		Texture:None,
		NormalTexture:None,
		BumpTexture:None,
		GlossTexture:None,
		BumpMultiplier:Some(1.0),
		Emittance:Some(0.0),
		Index:Some(index),
		Gloss:Some(gloss),
		Tint:Some(0.0),
		Reflectivity:Some(-1.0),
		Transparent:Some(false)}
}

pub fn ClearMaterial(index:f64, gloss:f64)-> Material {
	return Material{
		Color:Some(Black),
		Texture:None,
		NormalTexture:None,
		BumpTexture:None,
		GlossTexture:None,
		BumpMultiplier:Some(1.0),
		Emittance:Some(0.0),
		Index:Some(index),
		Gloss:Some(gloss),
		Tint:Some(0.0),
		Reflectivity:Some(-1.0),
		Transparent:Some(true),
	}
}

pub fn TransparentMaterial(color:Color,index:f64, gloss:f64, tint: f64)-> Material {
	return Material{
		Color:Some(color),
		Texture: None,
		NormalTexture: None,
		BumpTexture: None,
		GlossTexture: None,
		BumpMultiplier: Some(1.0),
		Emittance: Some(0.0),
		Index: Some(index),
		Gloss: Some(gloss),
		Tint: Some(tint),
		Reflectivity: Some(-1.0),
		Transparent: Some(true)
	}
}

pub fn MetallicMaterial(color:Color, gloss:f64, tint: f64)-> Material {
	return Material{
		Color:Some(color),
		Texture:None,
		NormalTexture:None,
		BumpTexture:None,
		GlossTexture:None,
		BumpMultiplier:Some(1.0),
		Emittance:Some(0.0),
		Index:Some(1.0),
		Gloss:Some(gloss),
		Tint:Some(tint),
		Reflectivity:Some(1.0),
		Transparent:Some(false),
	}
}

pub fn LightMaterial(color :Color, emittance: f64)-> Material {
	return Material{
		Color:Some(color),
		Texture:None,
		NormalTexture:None,
		BumpTexture:None,
		GlossTexture:None,
		BumpMultiplier:Some(1.0),
		Emittance:Some(emittance),
		Index:Some(1.0),
		Gloss:Some(0.0),
		Tint:Some(0.0),
		Reflectivity:Some(-1.0),
		Transparent:Some(false)}
}

pub fn MaterialAt(shape:Box<dyn Shape>, point:Vector)-> Material {
	let mut material = shape.MaterialAt(point);
	let uv = shape.UV(point);
	if material.Texture.is_none() {
		material.Color = Some(material.Texture.unwrap().Sample(uv.X, uv.Y));
	}
	if material.GlossTexture.is_none() {
		let c = material.GlossTexture.unwrap().Sample(uv.X, uv.Y);
		material.Gloss = Some((c.R + c.G + c.B) / 3.0);
	}
	return material
}
