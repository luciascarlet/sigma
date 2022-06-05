use std::cmp::max;

use crate::operators::FilterOp;
use crate::operators::MetaOp;
use crate::synthesis::Partial;
use crate::maths::{
	note_values::*,
	interpolation::{
		smooth_step,
		lerp,
		lerp_partials,
	},
};

pub struct FreqRangeOp {
	pub min: f64,
	pub max: f64,
	pub width: f64,
}

// this shit is probably less efficient than fucking bogo sort
impl MetaOp for FreqRangeOp {

	fn process(
		&self,
		pre: Vec<Partial>,
		filter: &dyn FilterOp,
		_aux: Option<Vec<Partial>>,
		fundamental: f64,
	) -> Vec<Partial> {

		let mut out = pre.clone();		// filtered + blended partials
		let mut filtered = pre.clone();		// filtered partials (in order to compute out)

		// determine total output range so only the required partials are processed
		let start_idx: usize = (self.min / fundamental).floor() as usize;
		let end_idx = max((self.max / fundamental).ceil() as usize, pre.len());

		// filter required partials
		for i in start_idx..end_idx {
			filtered[i] = filter.transform(pre[i].clone(), i, fundamental);
		}

		// determine blend ranges
		let min_slope_end = (mtof(ftom(self.min) + self.width)) / fundamental;
		let max_slope_start = (mtof(ftom(self.max) - self.width)) / fundamental;

		// calculate a vector of blend factors
		let mut blend_factors = vec![0.0_f64; filtered.len()];
		for i in start_idx..end_idx {
			match i {
				i if i < min_slope_end.ceil() as usize => {
					blend_factors[i] = smooth_step(0.0, 1.0, ((i - start_idx) as f64) / min_slope_end);
				},
				i if i > min_slope_end.ceil() as usize && i < max_slope_start.floor() as usize => {
					blend_factors[i] = 1.0;
				},
				i if i > max_slope_start.floor() as usize => {
					blend_factors[i] = smooth_step(1.0, 0.0, (i - start_idx) as f64 - max_slope_start / (filtered.len() as f64 - max_slope_start));
				},
				_ => {}
			}

		}

		// I literally do not even fucking know if ANY of this shit is going to WORK I'm just completely going in blind
		for (i, p) in out.iter_mut().enumerate() {
			*p = lerp_partials(*p, filtered[i].clone(), blend_factors[i]);
		}

		out
	}

}
