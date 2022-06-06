use crate::maths::lerp;
use core::f64::consts::PI;

pub struct LUTSine {
	pub lut: Vec<f64>,
	size: usize,
}

impl LUTSine {
	pub fn new(size: usize) -> LUTSine {
		let mut lut = vec![0.0; size + 2];
		for i in 0..size {
			lut[i] = (i as f64 / size as f64 * PI * 0.5).sin();
		}
		lut[size] = 1.0; // for interpolation
		lut[size + 1] = 1.0; // dumbass fucking hack that i can't be arsed to actually fix
		LUTSine {
			lut: lut,
			size: size,
		}
	}

	// return sine wave sample at a given phase between 0.0 and 1.0
	pub fn sample(&self, phase: f64) -> f64 {
		let phase = phase % 1.0;
		match phase {
			x if x >= 0.0 && x <= 0.25 => {
				let peek = phase * self.size as f64 * 4.0;
				lerp(
					self.lut[peek as usize],
					self.lut[(peek + 1.0) as usize],
					peek.fract(),
				)
			}
			x if x > 0.25 && x <= 0.5 => {
				let peek = (-phase + 0.5) * self.size as f64 * 4.0;
				lerp(
					self.lut[peek as usize],
					self.lut[(peek + 1.0) as usize],
					peek.fract(),
				)
			}
			x if x > 0.5 && x <= 0.75 => {
				let peek = (phase - 0.5) * self.size as f64 * 4.0;
				-lerp(
					self.lut[peek as usize],
					self.lut[(peek + 1.0) as usize],
					peek.fract(),
				)
			}
			x if x > 0.75 && x <= 1.0 => {
				let peek = (-phase + 1.0) * self.size as f64 * 4.0;
				-lerp(
					self.lut[peek as usize],
					self.lut[(peek + 1.0) as usize],
					peek.fract(),
				)
			}
			_ => panic!("phase outside of 0-1; what the fuck?"),
		}
	}
}
