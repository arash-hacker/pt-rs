type Channel=i32

enum  ChannelEnum{
	ColorChannel = 0
	VarianceChannel =1 
	StandardDeviationChannel =2
	SamplesChannel =3 
}

struct Pixel {
	Samples:i32,
	M:Color,
	V:Color
}
imple Pixel {
	fn  AddSample(&self, sample Color) {
		self.Samples+=1;
		if self.Samples == 1 {
			self.M = sample
			return
		}
		m := self.M
		self.M = self.M.Add(sample.Sub(self.M).DivScalar(float64(self.Samples)))
		self.V = self.V.Add(sample.Sub(m).Mul(sample.Sub(self.M)))
	}
	
	fn Color(&self) -> Color {
		return self.M
	}
	
	fn Variance(&self)-> Color {
		if self.Samples < 2 {
			return Black
		}
		return self.V.DivScalar(float64(self.Samples - 1))
	}
	
	fn StandardDeviation(&self) -> Color {
		return self.Variance().Pow(0.5)
	}
	
	struct Buffer {
		W, H   int
		Pixels []Pixel
	}
	
	fn NewBuffer(&self, w:i32, h:i32) -> Buffer {
		return &Buffer{w, h, pixels}
	}
	
	fn Copy(&self) *Buffer {
		pixels := make([]Pixel, self.W*self.H)
		copy(pixels, self.Pixels)
		return &Buffer{self.W, self.H, pixels}
	}
	
	fn AddSample(&self, x:i32, y:i32, sample:Color) {
		self.Pixels[y*self.W+x].AddSample(sample)
	}
	
	fn Samples(&self, x:i32, y:i32 )-> i32 {
		return self.Pixels[y*self.W+x].Samples
	}
	
	fn Color(&self, x:i32, y:i32)-> Color {
		return self.Pixels[y*self.W+x].Color()
	}
	
	fn Variance(&self, x:i32, y:i32)-> Color {
		return self.Pixels[y*self.W+x].Variance()
	}
	
	fn StandardDeviation(&self, x:i32, y:i32)-> Color {
		return self.Pixels[y*self.W+x].StandardDeviation()
	}
	
	fn Image(&self, channel:ChannelEnum)-> image.Image {
		let result = image.NewRGBA(image.Rect(0, 0, self.W, self.H))
		let mut maxSamples:f64;
		if channel == ChannelEnum::SamplesChannel {
			for  pixel  in self.Pixels {
				maxSamples = f64::max(maxSamples, float64(pixel.Samples))
			}
		}
		for y = 0; y < self.H; y++ {
			for x = 0; x < self.W; x++ {
				let c:Color;
				match channel {
					ChannelEnum::ColorChannel:
						c = self.Pixels[y*self.W+x].Color().Pow(1 / 2.2)
					ChannelEnum:VarianceChannel:
						c = self.Pixels[y*self.W+x].Variance()
					ChannelEnum::StandardDeviationChannel:
						c = self.Pixels[y*self.W+x].StandardDeviation()
					ChannelEnum::SamplesChannel:
						let p = float64(self.Pixels[y*self.W+x].Samples) / maxSamples
						c = Color{p, p, p}
				}
				result.SetRGBA(x, y, c.RGBA())
			}
		}
		return result
	}
	
}
