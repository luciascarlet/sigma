use std::cmp::max;

use crate::synthesis::Partial;

// operator for running processing on two sets of partials
pub trait DualOp {
	fn transform(&self, p1: Partial, p2: Partial, idx: usize, fundamental: f64) -> Partial;
	fn process(&self, p1: Vec<Partial>, p2: Vec<Partial>, fundamental: f64) -> Vec<Partial> {
		let mut out = Vec::with_capacity(max(p1.len(), p2.len()));
		for (i, p) in out.iter_mut().enumerate() {
			*p = self.transform(p1[i], p2[i], i, fundamental);
		}
		out
	}
}
