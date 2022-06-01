use crate::synthesis::partial::Partial;

// // base trait for all filter ops
// pub trait FilterOp {
// }

// // these are free to be implemented simultaneously on a single filter op if desired
// pub trait PhaseOp: FilterOp {
// 	fn process_phases(&self, phases: Vec<f64>, fundamental: f64) -> Vec<f64>;
// }

// pub trait RatioOp: FilterOp {
// 	fn process_ratios(&self, ratios: Vec<f64>, fundamental: f64) -> Vec<f64>;
// }

// pub trait AmpOp: FilterOp {
// 	fn process_amps(&self, amps: Vec<f64>, fundamental: f64) -> Vec<f64>;
// }

// pub trait StereoOp: FilterOp {
// 	fn process_pans(&self, pans: Vec<f64>, fundamental: f64) -> Vec<f64>;
// }

// // this one is special because it operates on full partial entries
// pub trait PartialOp: FilterOp {
// 	fn process_partials(&self, partials: Vec<Partial>) -> Vec<Partial>;
// }


// you know what, I don't think I need all of this shit - let's just make everything PartialOp
pub trait FilterOp {
	fn transform(&self, partial: Partial, idx: usize, fundamental: f64) -> Partial;
	fn process(&self, partials: Vec<Partial>, fundamental: f64) -> Vec<Partial> {
		let mut out = partials.clone();
		for (i, p) in partials.iter().enumerate() {
			out[i] = self.transform(p.clone(), i, fundamental);
		}
		out
	}
}