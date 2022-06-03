use crate::synthesis::Partial;

pub trait SourceOp {
	fn process_partial(&self, idx: usize, fundamental: f64) -> Partial;
	fn process_all(&self, num_partials: usize, fundamental: f64) -> Vec<Partial> {
		// is this slower or faster than filling it with a bunch of blank partials first?
		// whatever, copilot gave me this and i can't be arsed to change it
		let mut partials = Vec::with_capacity(num_partials);
		for idx in 0..num_partials {
			partials.push(self.process_partial(idx, fundamental));
		}
		partials
	}
}
