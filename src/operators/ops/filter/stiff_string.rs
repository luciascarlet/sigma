use crate::synthesis::partial::Partial;
use crate::operators::filter_op::*;

// simple exponential ratio stretcher, similar to NI Razor's "stiff string" mode
pub struct StiffStringOp {
	pub intensity: f64,
}

impl FilterOp for StiffStringOp {
	fn transform(&self, partial: Partial, idx: usize, _fundamental: f64) -> Partial {
		let mut out = partial.clone();
		out.ratio = out.ratio.powf(idx as f64 * self.intensity + 1.0);
		out
	}
}

impl Default for StiffStringOp {
	fn default() -> StiffStringOp {
		StiffStringOp {
			intensity: 0.025,
		}
	}
}