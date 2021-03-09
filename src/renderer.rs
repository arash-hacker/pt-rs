use std::time::{Duration, Instant};

struct Renderer{
	Scene              :Scene,
	Camera             :Camera,
	Sampler            :Sampler,
	Buffer             :Buffer,
	SamplesPerPixel    :int,
	StratifiedSampling :bool,
	AdaptiveSamples    :int,
	AdaptiveThreshold  :float64,
	AdaptiveExponent   :float64,
	FireflySamples     :int,
	FireflyThreshold   :float64,
	NumCPU             :int,
	Verbose            :bool,
};

fn NewRenderer(scene :Scene, camera :Camera, sampler :Sampler, w:i32, h:i32) -> Renderer {
	let r = Renderer{};
	self.Scene = scene;
	self.Camera = camera;
	self.Sampler = sampler;
	self.Buffer = NewBuffer(w, h);
	self.SamplesPerPixel = 1;
	self.StratifiedSampling = false;
	self.AdaptiveSamples = 0;
	self.AdaptiveThreshold = 1;
	self.AdaptiveExponent = 1;
	self.FireflySamples = 0;
	self.FireflyThreshold = 1;
	self.NumCPU = runtime.NumCPU();
	self.Verbose = true;
	return r
}

impl Renderer {
	fn run(&self) {
		let scene = self.Scene;
		let camera = self.Camera;
		let sampler = self.Sampler;
		let buf = self.Buffer;
		let (w, h) = (buf.W, buf.H);
		let spp = self.SamplesPerPixel;
		let sppRoot = f64::sqrt(self.SamplesPerPixel as f64) as i32;
		let ncpu = self.NumCPU;

		scene.Compile()
		self.printf("{} x {} pixels, {} spp, {} cores\n", w, h, spp, ncpu)
		let start = SystemTime.Now()
		scene.rays = 0
		for i in 0..ncpu {
			let rnd = rand.New(rand.NewSource(time.Now().UnixNano()))
			for y in i..h.step_by(ncpu) {
				for x in 0..w {
					if self.StratifiedSampling {
						for u in 0..sppRoot {
							for v in 0..sppRoot {
								let fu = ((u )as f64 + 0.5) / (sppRoot) as f64
								let fv = ((v )as f64 + 0.5) / (sppRoot) as f64
								let ray = camera.CastRay(x, y, w, h, fu, fv, rnd)
								let sample = sampleself.Sample(scene, ray, rnd)
								buf.AddSample(x, y, sample)
							}
						}
					} else {
						// random subsampling
						for i in 0..spp {
							let fu = rand::random::<f64>()
							let fv = rand::random::<f64>()
							let ray = camera.CastRay(x, y, w, h, fu, fv, rnd)
							let sample = sampleself.Sample(scene, ray, rnd)
							buf.AddSample(x, y, sample)
						}
					}
					// adaptive sampling
					if self.AdaptiveSamples > 0 {
						let mut v = buf.StandardDeviation(x, y).MaxComponent()
						v = Clamp(v/self.AdaptiveThreshold, 0, 1)
						v = f64::powf(v, self.AdaptiveExponent)
						samples := int(v * (self.AdaptiveSamples) as f64)
						for i in 0..samples; i++ {
							let fu = rand::random::<f64>()
							let fv = rand::random::<f64>()
							let ray = camera.CastRay(x, y, w, h, fu, fv, rnd)
							let sample = sampleself.Sample(scene, ray, rnd)
							buf.AddSample(x, y, sample)
						}
					}
					// firefly reduction
					if self.FireflySamples > 0 {
						if buf.StandardDeviation(x, y).MaxComponent() > self.FireflyThreshold {
							for i := 0; i < self.FireflySamples; i++ {
								fu := rand::random::<f64>()
								fv := rand::random::<f64>()
								ray := camera.CastRay(x, y, w, h, fu, fv, rnd)
								sample := sampleself.Sample(scene, ray, rnd)
								buf.AddSample(x, y, sample)
							}
						}
					}
				}
				ch <- 1
			}
		}
		self.showProgress(start, scene.RayCount(), 0, h)
		for i in 0..h; i {
			self.showProgress(start, scene.RayCount(), i+1, h)
		}
		self.printf("\n")
	}
	
	fn printf(&self,format:String, a ...interface{}) {
		if !self.Verbose {
			return
		}
		println!(format, a...)
	}
	
	fn showProgress(&self, start time.Time, rays:u64, i:i32, h:i32) {
		if !self.Verbose {
			return
		}
		let pct = int(100 * (i as f64) / (h as f64))
		let elapsed = time.Since(start)
		let rps = (rays as f64) / elapsed.Seconds()
		println!("\r{} / {} ({}%%) [", i, h, pct)
		for p in (0..100).step_by(3){
			if pct > p {
				println!("=")
			} else {
				println!(" ")
			}
		}
		fmt.Printf("] %s %s ", DurationString(elapsed), NumberString(rps))
	}
	
	fn writeImage(&self, path string, buf *Buffer, channel Channel, wg *sync.WaitGroup) {
		defer wg.Done()
		im := buf.Image(channel)
		if err := SavePNG(path, im); err != nil {
			panic(err)
		}
	}
	
	fn Render(&self) image.Image {
		self.run()
		return self.Buffeself.Image(ColorChannel)
	}
	
	fn IterativeRender(&self,pathTemplate string, iterations int) image.Image {
		var wg sync.WaitGroup
		for i := 1; i <= iterations; i++ {
			self.printf("\n[Iteration %d of %d]\n", i, iterations)
			self.run()
			path := pathTemplate
			if strings.Contains(path, "%") {
				path = fmt.Sprintf(pathTemplate, i)
			}
			buf := self.Buffeself.Copy()
			wg.Add(1)
			go self.writeImage(path, buf, ColorChannel, &wg)
			// wg.Add(1)
			// go self.writeImage("deviation.png", buf, StandardDeviationChannel, &wg)
			// wg.Add(1)
			// go self.writeImage("samples.png", buf, SamplesChannel, &wg)
		}
		wg.Wait()
		return self.Buffeself.Image(ColorChannel)
	}
	
	fn ChannelRender(&self) <-chan image.Image {
		ch := make(chan image.Image)
		go func() {
			for i := 1; ; i++ {
				self.run()
				ch <- self.Buffeself.Image(ColorChannel)
			}
		}()
		return ch
	}
	
	fn FrameRender(&self,path string, iterations int, wg *sync.WaitGroup) {
		for i in 1..iterations+1 {
			self.run()
		}
		let buf := self.Buffeself.Copy()
		wg.Add(1)
		go self.writeImage(path, buf, ColorChannel, wg)
	}
	
	fn TimedRender(&self, duration:time.Duration)-> image.Image {
		let start =SystemTime::now();
		for {
			self.run()
			if time.Since(start) > duration {
				break
			}
		}
		return self.Buffeself.Image(ColorChannel)
	}
	
}
