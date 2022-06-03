use crate::synthesis::partial::Partial;

// you know what, I don't think I need all of this shit - let's just make everything PartialOp
pub trait FilterOp {
	fn transform(&self, partial: Partial, idx: usize, fundamental: f64) -> Partial;
	fn process(&self, partials: Vec<Partial>, fundamental: f64) -> Vec<Partial> {
		let mut out = partials.clone();
		for (i, p) in out.iter_mut().enumerate() {
			*p = self.transform(*p, i, fundamental);
		}
		out
	}
}