use image::{ImageBuffer, Rgb, Rgba};

use crate::{bbox::*, color::{Black, Color}};
use crate::shape::*;
use crate::material::*;
use crate::sdf::*;
use crate::hit::*;
use crate::vector::*;
use crate::triangle::*;
use crate::tree::*;
use crate::axis::*;
extern crate image;

pub type Channel=i32;

#[derive(PartialEq)]
pub enum  ChannelEnum{
	ColorChannel = 0,
	VarianceChannel =1, 
	StandardDeviationChannel =2,
	SamplesChannel =3,
}

pub struct Pixel {
	pub Samples:i32,
	pub M:Color,
	pub V:Color
}
impl Pixel {
	pub fn  AddSample(&self, sample :Color) {
		self.Samples+=1;
		if self.Samples == 1 {
			self.M = sample;
			return
		}
		let m = self.M;
		self.M = self.M.Add(sample.Sub(self.M).DivScalar((self.Samples)as f64));
		self.V = self.V.Add(sample.Sub(m).Mul(sample.Sub(self.M)));
	}
	
	pub fn Color(&self) -> Color {
		return self.M
	}
	
	pub fn Variance(&self)-> Color {
		if self.Samples < 2 {
			return Black
		}
		return self.V.DivScalar((self.Samples - 1) as f64)
	}
	
	pub fn StandardDeviation(&self) -> Color {
		return self.Variance().Pow(0.5)
	}
}
pub struct Buffer {
	pub W:i32,
	pub H:i32,
	pub Pixels: Vec<Pixel>
}
pub fn NewBuffer( w:i32, h:i32) -> Buffer {
	let vv=vec![];
	return Buffer{W:w, H:h,Pixels: vv}
}
impl Buffer{	
	
	pub fn Copy(&self) ->Buffer {
		return Buffer{W:self.W,H:self.H,Pixels: self.Pixels}
	}
	
	pub fn AddSample(&self, x:i32, y:i32, sample:Color) {
		self.Pixels[(y*self.W+x) as usize].AddSample(sample)
	}
	
	pub fn Samples(&self, x:i32, y:i32 )-> i32 {
		return self.Pixels[(y*self.W+x) as usize].Samples
	}
	
	pub fn Color(&self, x:i32, y:i32)-> Color {
		return self.Pixels[(y*self.W+x) as usize].Color()
	}
	
	pub fn Variance(&self, x:i32, y:i32)-> Color {
		return self.Pixels[(y*self.W+x) as usize].Variance()
	}
	
	pub fn StandardDeviation(&self, x:i32, y:i32)-> Color {
		return self.Pixels[(y*self.W+x) as usize].StandardDeviation()
	}
	
	pub fn Image(&self, channel:ChannelEnum)-> image::ImageBuffer<image::Rgb<u8>, Vec<u8>>
	{
		let result =image::ImageBuffer::new(self.W as u32, self.H as u32);
		let mut maxSamples:f64;
		if channel == ChannelEnum::SamplesChannel {
			for  pixel  in self.Pixels {
				maxSamples = f64::max(maxSamples, (pixel.Samples) as f64)
			}
		}
		for y in 0..self.H {
			for x in 0..self.W {
				let mut c:Color;
				match channel {
					ChannelEnum::ColorChannel=>{
						c = self.Pixels[(y*self.W+x) as usize].Color().Pow(1.0 / 2.2);},
					ChannelEnum::VarianceChannel=>{	
						c = self.Pixels[(y*self.W+x) as usize].Variance();}
					ChannelEnum::StandardDeviationChannel=>{
						c = self.Pixels[(y*self.W+x) as usize].StandardDeviation();}
					ChannelEnum::SamplesChannel=>{
						let p = (self.Pixels[(y*self.W+x) as usize].Samples) as f64 / maxSamples;
						c = Color{R:p, G:p, B:p};
					},
					_=>{},
				}

				let mut pixel = result.get_pixel_mut(x as u32, y as u32);
				*pixel=image::Rgb([c.RGBA().R as u8, c.RGBA().G as u8, c.RGBA().B as u8]);

			}
		}
		return result
	}
	
}
