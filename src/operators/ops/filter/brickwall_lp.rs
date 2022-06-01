use crate::synthesis::partial::Partial;
use crate::operators::filter_op::*;

// simple brickwall lowpass filter that just gets rid of partials past a certain frequency lmao
pub struct BrickwallLpOp {
	pub cutoff: f64,
}

impl FilterOp for BrickwallLpOp {
	fn transform(&self, partial: Partial, _idx: usize, fundamental: f64) -> Partial {
		match fundamental * partial.ratio {
			x if x < self.cutoff => partial,
			_ => {
				// is there any way to skip this clone and just return a modified struct as an expression directly?
				let result = partial.clone();
				result
			},
		}
	}
}