mod lut_sine;
pub mod sine_bank;
pub use sine_bank::SineBank; // avoid redundant qualifiers
pub mod partial;
pub use partial::Partial;

mod tests {
	// see if the fucking sine LUT even works
	#[test]
	fn run_sine() {
		let interval: usize = 22050;
		let spec = hound::WavSpec {
			channels: 1,
			sample_rate: 44100,
			bits_per_sample: 16,
			sample_format: hound::SampleFormat::Int,
		};

		let mut writer = hound::WavWriter::create("sine_no_bank.wav", spec).unwrap();

		let sine_lut = super::lut_sine::LUTSine::new(256);


		for t in 0..interval {
			writer.write_sample((sine_lut.sample((t as f64 * 40.0 / 44100.0) % 1.0) * i16::MAX as f64) as i16).unwrap();
		}


		for t in 0..interval {
			writer.write_sample((sine_lut.sample((t as f64 * 110.0 / 44100.0) % 1.0) * i16::MAX as f64) as i16).unwrap();
		}

		for t in 0..interval {
			writer.write_sample((sine_lut.sample((t as f64 * 330.0 / 44100.0) % 1.0) * i16::MAX as f64) as i16).unwrap();
		}
	}

	#[test]
	fn print_lut() {
		let size = 512;
		let test_iter = 40;
		let test_incr = 0.025;
		let sine_lut = super::lut_sine::LUTSine::new(size);

		for t in 0..test_iter {
			println!("{:?}", sine_lut.sample(t as f64 * test_incr));
		}
	}
}