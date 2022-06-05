// shut the fuck up
#![allow(dead_code)]
// shut the fuck up 2: electric boogaloo
#![allow(unused_imports)]

pub mod graph;
pub mod maths;
pub mod operators;
pub mod synthesis;

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_doesnt_work() {
		// im not even going to bother with an actual unit test i just want to see if the garbage my code generates even makes any sense
		extern crate hound;

		// create sine bank to work with
		let mut sine_bank = synthesis::SineBank::new(2048, 80.0, 44100.0);

		// create graph and operators
		let mut graph = graph::OpGraph::new(sine_bank.get_num_partials(), sine_bank.get_fundamental());

		graph.add_source_op(Box::new(operators::source::SawOp::default()));
		graph.add_filter_op(Box::new(operators::filter::PhaseOverflowOp::default()));
		graph.add_filter_op(Box::new(operators::filter::StiffStringOp::default()));

		// create partials
		let partials: Vec<synthesis::Partial> = graph.run();
		print!("{:?}", partials);

		// prepare wav writer
		let spec = hound::WavSpec {
			channels: 2,
			sample_rate: 44100,
			bits_per_sample: 16,
			sample_format: hound::SampleFormat::Int,
		};
		let mut writer = hound::WavWriter::create("test.wav", spec).unwrap();

		// run sine bank, write to wav
		sine_bank.set_partials(partials);

		for _ in 0..44100 {
			sine_bank.tick();
			let out = sine_bank.sample();
			let ismp_l = (out.0 * i16::MAX as f64 * 0.5) as i16;
			let ismp_r = (out.1 * i16::MAX as f64 * 0.5) as i16;
			
			// WAV interleaves channels, so the destination channel is not explicitly specified
			writer.write_sample(ismp_l).unwrap();
			writer.write_sample(ismp_r).unwrap();
		}

	}
}