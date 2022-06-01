use crate::operators::source_op::SourceOp;
use crate::synthesis::partial::Partial;

/* i should probably come up with a proc macro solution for parameters in ops or something, so i don't have to fucking expose the fields directly - something that allows me to actually control what happens when they are get and set without requiring getter/setter methods, as that is nasty and unpredictable */

pub struct SawOp {
	pub phase: f64, // i am going to kill myself
	pub falloff: f64,
}

impl Default for SawOp {
	fn default() -> SawOp {
		SawOp {
			phase: 0.0,
			falloff: 1.0,
		}
	}
}

impl SourceOp for SawOp {
	// note: we won't need the fundamental here at all, but it's required by the op trait
	fn process(&self, num_partials: usize, _fundamental: f64) -> Vec<Partial> {
		let mut partials = vec![
			Partial {
				ratio: 1.0,
				phase: 1.0,
				amp: 0.0,
				pan: 0.0,
			};
			num_partials
		];
		for i in 0..num_partials {
			partials[i].ratio = (i + 1) as f64;
			partials[i].phase = self.phase % 1.0; // you never know...
			partials[i].amp = 1.0 / ((i + 1) as f64 * self.falloff);
		}
		partials
	}
}
