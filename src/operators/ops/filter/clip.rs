use crate::operators::FilterOp;
use crate::synthesis::Partial;

// simple spectral clipper, hard-limits partial amplitudes
pub struct ClipOp {
	pub ceiling: f64,
}

impl FilterOp for ClipOp {
	fn transform(&self, partial: Partial, _idx: usize, _fundamental: f64) -> Partial {
		let mut out = partial.clone();
		out.amp = out.amp.min(self.ceiling);
		out
	}
}

impl Default for ClipOp {
	fn default() -> Self {
		ClipOp {
			ceiling: 1.0,
		}
	}
}