use std::time::{self,Duration};
use crate::{renderer::*, sampler::NewSampler, util::SavePNG};
use crate::camera::*;
use crate::sampler::{self, LightModeEnum};
use crate::scene::*;
use crate::util;




pub fn ParameterTest(scene:Scene, camera:Camera, w:i32, h :i32, duration: time::Duration) {
	let mut sampler :sampler::DefaultSampler;
	let mut renderer :Renderer;

	sampler = NewSampler(1, 4);
	renderer = NewRenderer(scene, camera, Box::from(sampler), w, h);
	util::SavePNG(String::from("1.Default.png"), renderer.TimedRender(duration));

	sampler = NewSampler(4, 4);
	renderer = NewRenderer(scene, camera, Box::from(sampler), w, h);
	util::SavePNG(String::from("2.StratifiedFirstHit.png"), renderer.TimedRender(duration));

	sampler = NewSampler(4, 4);
	sampler.LightMode = LightModeEnum::LightModeAll as i32;
	renderer = NewRenderer(scene, camera, Box::from(sampler), w, h);
	util::SavePNG(String::from("3.LightModeAll.png"), renderer.TimedRender(duration));

	sampler = NewSampler(4, 4);
	sampler.SpecularMode = sampler::SpecularModeFirst;
	renderer = NewRenderer(scene, camera, Box::from(sampler), w, h);
	util::SavePNG(String::from("4.SpecularModeFirst.png"), renderer.TimedRender(duration));

	sampler = NewSampler(4, 4);
	sampler.SpecularMode = sampler::SpecularModeAll;
	renderer = NewRenderer(scene, camera, Box::from(sampler), w, h);
	util::SavePNG(String::from("5.SpecularModeAll.png"), renderer.TimedRender(duration));

	sampler = NewSampler(4, 4);
	renderer = NewRenderer(scene, camera, Box::from(sampler), w, h);
	renderer.AdaptiveSamples = 16.0;
	SavePNG(String::from("6.AdaptiveSamples.png"), renderer.TimedRender(duration));

	sampler = NewSampler(4, 4);
	renderer = NewRenderer(scene, camera, Box::from(sampler), w, h);
	renderer.FireflySamples = 256.0;
	SavePNG(String::from("7.FireflySamples.png"), renderer.TimedRender(duration));

	sampler = NewSampler(4, 4);
	sampler.LightMode = LightModeEnum::LightModeAll as i32;
	sampler.SpecularMode = sampler::SpecularModeFirst;
	renderer = NewRenderer(scene, camera, Box::from(sampler), w, h);
	renderer.AdaptiveSamples = 16.0;
	renderer.FireflySamples = 256.0;
	SavePNG(String::from("8.Everything.png"), renderer.TimedRender(duration));
}
