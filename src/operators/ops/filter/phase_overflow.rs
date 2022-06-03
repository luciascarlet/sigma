use crate::operators::FilterOp;
use crate::synthesis::Partial;

// applies an overflowing function to partial phases for more interesting textures
pub struct PhaseOverflowOp {
	pub pre_multiplier: f64,
	pub post_multiplier: f64,
	pub exponent: f64,
	pub offset: f64,
}

impl FilterOp for PhaseOverflowOp {
	fn transform(&self, partial: Partial, idx: usize, _fundamental: f64) -> Partial {
		let mut out = partial.clone();
		out.phase = (out.phase + idx as f64 * self.pre_multiplier).powf(self.exponent) * self.post_multiplier + self.offset;
		out
	}
}

impl Default for PhaseOverflowOp {
	fn default() -> PhaseOverflowOp {
		PhaseOverflowOp {
			multiplier: 0.05,
			exponent: 2.0,
			offset: 0.0,
		}
	}
}
