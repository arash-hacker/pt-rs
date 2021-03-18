use std::time::{self, SystemTime, Duration, Instant};
use rand::random;

use crate::{bbox::*, buffer::{Buffer, ChannelEnum}, camera::Camera, sampler, scene::Scene};
use crate::shape::*;
use crate::material::*;
use crate::sdf::*;
use crate::buffer::*;
use crate::hit::*;
use crate::vector::*;
use crate::sampler::*;
use crate::triangle::*;
use crate::tree::*;
use crate::axis::*;
use crate::util::{self, *};
pub struct Renderer{
	pub Scene              :Option<Scene>,
	pub Camera             :Option<Camera>,
	pub Sampler            :Option<Box<dyn Sampler>>,
	pub Buffer             :Option<Buffer>,
	pub SamplesPerPixel    :f64,
	pub StratifiedSampling :bool,
	pub AdaptiveSamples    :f64,
	pub AdaptiveThreshold  :f64,
	pub AdaptiveExponent   :f64,
	pub FireflySamples     :f64,
	pub FireflyThreshold   :f64,
	pub NumCPU             :i32,
	pub Verbose            :bool,
}

pub fn NewRenderer(scene :Scene, camera :Camera, sampler :Box<dyn Sampler>, w:i32, h:i32) -> Renderer {
	let mut r = Renderer::Default();
	r.Scene = Some(scene);
	r.Camera = Some(camera);
	r.Sampler = Some(sampler);
	r.Buffer = Some(NewBuffer(w, h));
	r.SamplesPerPixel = 1.0;
	r.StratifiedSampling = false;
	r.AdaptiveSamples = 0.0;
	r.AdaptiveThreshold = 1.0;
	r.AdaptiveExponent = 1.0;
	r.FireflySamples = 0.0;
	r.FireflyThreshold = 1.0;
	r.NumCPU = 2;
	r.Verbose = true;
	return r
}

impl Renderer {
	pub fn Default()->Renderer{
		return Renderer{
			Scene:None,
			Camera:None,
			Sampler:None,
			Buffer:None,
			SamplesPerPixel:0.0,
			StratifiedSampling:false,
			AdaptiveSamples:0.0,
			AdaptiveThreshold:0.0,
			AdaptiveExponent:0.0,
			FireflySamples:0.0,
			FireflyThreshold:0.0,
			NumCPU:0,
			Verbose:false,
		}
	}
	pub fn run(&self) {
		let mut scene = self.Scene.unwrap();
		let mut camera = self.Camera.unwrap();
		let mut sampler = self.Sampler.unwrap();
		let mut buf = self.Buffer.unwrap();
		let  (w, h) = (buf.W, buf.H);
		let mut spp = self.SamplesPerPixel;
		let mut sppRoot = f64::sqrt(self.SamplesPerPixel as f64) as i32;
		let mut ncpu = self.NumCPU;

		scene.Compile();
		println!("{} x {} pixels, {} spp, {} cores\n", w, h, spp, ncpu);
		let start = Instant::now();
		scene.rays = 0;
		for i in 0..ncpu {
			let rnd = random();
			for y in (i..h).step_by(ncpu as usize) {
				for x in 0..w {
					if self.StratifiedSampling {
						for u in 0..sppRoot {
							for v in 0..sppRoot {
								let fu = ((u )as f64 + 0.5) / (sppRoot) as f64;
								let fv = ((v )as f64 + 0.5) / (sppRoot) as f64;
								let ray = camera.CastRay(x, y, w, h, fu, fv, rand::random());
								let sample = sampler.Sample(scene, ray, rand::random());
								buf.AddSample(x, y, sample);
							}
						}
					} else {
						// random subsampling
						for i in 0..(spp as i32) {
							let fu = rand::random::<f64>();
							let fv = rand::random::<f64>();
							let ray = camera.CastRay(x, y, w, h, fu, fv, rnd);
							let sample = sampler.Sample(scene, ray, rnd);
							buf.AddSample(x, y, sample);
						}
					}
					// adaptive sampling
					if self.AdaptiveSamples > 0.0 {
						let mut v = buf.StandardDeviation(x, y).MaxComponent();
						v = util::Clamp(v/self.AdaptiveThreshold, 0.0, 1.0);
						v = f64::powf(v, self.AdaptiveExponent);
						let samples = (v * (self.AdaptiveSamples) as f64) as  i32;
						for i in 0..samples {
							let fu = rand::random::<f64>();
							let fv = rand::random::<f64>();
							let ray = camera.CastRay(x, y, w, h, fu, fv, rnd);
							let sample = sampler.Sample(scene, ray, rnd);
							buf.AddSample(x, y, sample)
						}
					}
					// firefly reduction
					if self.FireflySamples > 0.0 {
						if buf.StandardDeviation(x, y).MaxComponent() > self.FireflyThreshold {
							for i in 0..(self.FireflySamples as i32) {
								let fu = rand::random::<f64>();
								let fv = rand::random::<f64>();
								let ray = camera.CastRay(x, y, w, h, fu, fv, rnd);
								let sample = sampler.Sample(scene, ray, rnd);
								buf.AddSample(x, y, sample);
							}
						}
					}
				}
				
			}
		}
		self.showProgress(start, scene.RayCount(), 0, h);
		for i in 0..h {
			self.showProgress(start, scene.RayCount(), i+1, h);
		}
		println!("\n")
	}
	
	// pub fn printf(&self,format:String, a ...interface{}) {
	// 	if !self.Verbose {
	// 		return
	// 	}
	// 	println!(format, a...)
	// }
	
	pub fn showProgress(&self, start:Instant, rays:u64, i:i32, h:i32) {
		if !self.Verbose {
			return
		}
		let pct = (100.0 * (i as f64) / (h as f64)) as i32;
		
		let elapsed = time::Instant::now().duration_since(start);
		let rps = (rays as f64) / elapsed.as_secs() as f64;
		println!("\r{} / {} ({}%%) [", i, h, pct);
		for p in (0..100).step_by(3){
			if pct > p {
				println!("=");
			} else {
				println!(" ");
			}
		}
		println!("] {} {} ", DurationString(elapsed), NumberString(rps))
	}
	
	pub fn writeImage(&self, path :String, buf :Buffer,channel:ChannelEnum) {
		let im = buf.Image(channel);
		SavePNG(path, im);
	}
	
	pub fn Render(&self) ->image::ImageBuffer<image::Rgb<u8>, Vec<u8>> {
		self.run();
		return self.Buffer.unwrap().Image(ChannelEnum::ColorChannel);
	}
	
	pub fn IterativeRender(&self,pathTemplate:String, iterations:i32) ->image::ImageBuffer<image::Rgb<u8>, Vec<u8>> {
		for i in 1..iterations+1 {
			println!("\n[Iteration {} of {}]\n", i, iterations);
			self.run();
			let path = pathTemplate;
			if path.contains("%") {
				//path = fmt.Sprintf(pathTemplate, i)
			}
			let buf = self.Buffer.unwrap().Copy();
			self.writeImage(path, buf, ChannelEnum::ColorChannel)
			// wg.Add(1)
			// go self.writeImage("deviation.png", buf, StandardDeviationChannel, &wg)
			// wg.Add(1)
			// go self.writeImage("samples.png", buf, SamplesChannel, &wg)
		}
		return self.Buffer.unwrap().Image(ChannelEnum::ColorChannel)
	}
	
	// pub fn ChannelRender(&self) <-chan image.Image {
	// 	ch := make(chan image.Image)
	// 	go func() {
	// 		for i := 1; ; i++ {
	// 			self.run()
	// 			ch <- self.Buffeself.Image(ChannelEnum::ColorChannel)
	// 		}
	// 	}()
	// 	return ch
	// }
	
	// pub fn FrameRender(&self,path string, iterations int, wg *sync.WaitGroup) {
	// 	for i in 1..iterations+1 {
	// 		self.run()
	// 	}
	// 	let buf := self.Buffeself.Copy()
	// 	wg.Add(1)
	// 	go self.writeImage(path, buf, ChannelEnum::ColorChannel, wg)
	// }
	
	pub fn TimedRender(&self, duration:time::Duration)-> image::ImageBuffer<image::Rgb<u8>, Vec<u8>> {
		let start =Instant::now();
		loop {
			self.run();
			if start.elapsed() > duration {
				break
			}
		}
		return self.Buffer.unwrap().Image(ChannelEnum::ColorChannel)
	}
	
}
