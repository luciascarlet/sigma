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
	extern crate hound;

	// see if the sine bank uses the sine LUT correctly
	#[test]
	fn run_sine_in_bank() {
		let mut sine_bank = synthesis::SineBank::new(1, 440.0, 44100.0);

		sine_bank.set_partial_amp(0, 0.75);
		
		let spec = hound::WavSpec {
			channels: 1,
			sample_rate: 44100,
			bits_per_sample: 16,
			sample_format: hound::SampleFormat::Int,
		};

		let mut writer = hound::WavWriter::create("sine_from_bank.wav", spec).unwrap();

		for _ in 0..44100 {
			sine_bank.tick()  ;
			writer.write_sample((sine_bank.sample().0 * i16::MAX as f64) as i16).unwrap();
		}
	}

	// im not even going to bother with an actual unit test i just want to see if the garbage my code generates even makes any sense
	#[test]
	fn run_graph() {
		// create sine bank to work with
		let mut sine_bank = synthesis::SineBank::new(2048, 40.0, 44100.0);

		// create graph and operators
		let mut graph =
			graph::OpGraph::new(sine_bank.get_num_partials(), sine_bank.get_fundamental());

		graph.add_source_op(Box::new(operators::source::SawOp::default()));
		graph.add_filter_op(Box::new(operators::filter::PhaseOverflowOp {
			pre_multiplier: 0.05,
			post_multiplier: 1.0,
			exponent: 2.0,
			offset: 0.0,
		}));

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
		let mut writer = hound::WavWriter::create("graph.wav", spec).unwrap();

		// run sine bank, write to wav
		sine_bank.set_partials(partials);
		sine_bank.retrigger();

		for _ in 0..44100 {
			sine_bank.tick();
			let out = sine_bank.sample();
			let ismp_l = (out.0 * i16::MAX as f64 * 0.25) as i16;
			let ismp_r = (out.1 * i16::MAX as f64 * 0.25) as i16;

			// WAV interleaves channels, so the destination channel is not explicitly specified
			writer.write_sample(ismp_l).unwrap();
			writer.write_sample(ismp_r).unwrap();
		}
	}

	// this doesn't bother with the sine bank or anything, just there to profile how long it takes to process a graph
	#[test]
	fn graph_process_timing() {
		// create graph and operators
		let mut graph =
			graph::OpGraph::new(2048, 120.0);

		graph.add_source_op(Box::new(operators::source::SawOp::default()));
		graph.add_filter_op(Box::new(operators::filter::PhaseOverflowOp::default()));
		graph.add_filter_op(Box::new(operators::filter::StiffStringOp::default()));

		// do this shit a bunch of times
		for _ in 0..2000 {
			graph.run();
		}
	}
}
