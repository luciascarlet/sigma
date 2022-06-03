use crate::synthesis::Partial;
use crate::operators::DualOp;

pub struct DiffOp {
	pub mix: f64,
	pub diff_phase: bool,
	pub diff_ratio: bool,
}

// returns difference between two partials
impl DualOp for DiffOp {
	fn transform(&self, p1: Partial, p2: Partial, _idx: usize, _fundamental: f64) -> Partial {
		let mut out = p1.clone();
		out.amp = (p1.amp - p2.amp) * self.mix + p1.amp * (1.0 - self.mix);
		if self.diff_phase {
			out.phase = (p1.phase - p2.phase) * self.mix + p1.phase * (1.0 - self.mix);
		}
		if self.diff_ratio {
			out.ratio = (p1.ratio - p2.ratio) * self.mix + p1.ratio * (1.0 - self.mix);
		}
		out
	}
}