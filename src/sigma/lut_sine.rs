pub struct LUTSine {
	pub lut: [f64; 512],
	size: usize,
}

impl LUTSine {
	pub fn new(size: usize) -> LUTSine {
		let mut lut = [0.0; size + 1];
		for i in 0..size {
			lut[i] = (i as f64 / size as f64 * PI / 2.0).sin();
		}
		lut[size + 1] = 1.0; // for interpolation
		LUTSine { lut }
	}

	// return sine wave sample at a given phase between 0.0 and 1.0
	pub fn sample(&self, phase: f64) -> f64 {
		let phase = phase % 1.0;
		let lerp = (phase * self.size as f64).fract();
		match phase {
			0.0..0.25 => {
				let peek = phase * self.size as f64;
				self.lut[peek as usize] * (1.0 - lerp) + self.lut[peek as usize + 1] * lerp
			},
			0.25..0.5 => {
				let peek = (-phase + 0.5) * self.size as f64;
				self.lut[peek as usize] * (1.0 - lerp) + self.lut[peek as usize + 1] * lerp
			},
			0.5..0.75 => {
				let peek = (phase - 0.5) * self.size as f64;
				-self.lut[peek as usize] * (1.0 - lerp) + self.lut[peek as usize + 1] * lerp
			},
			0.75..1.0 => {
				let peek = (-phase + 1) * self.size as f64;
				-self.lut[peek as usize] * (1.0 - lerp) + self.lut[peek as usize + 1] * lerp
			},
		}
	}
}