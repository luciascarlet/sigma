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
		let start_ratio: usize = (self.min / fundamental).floor() as usize;
		let end_ratio = max((self.max / fundamental).ceil() as usize, pre.len());

		// filter required partials
		for i in start_ratio..end_ratio {
			filtered[i] = filter.transform(pre[i].clone(), i, fundamental);
		}

		// determine blend ranges
		let min_slope_end = ((mtof(ftom(self.min) + self.width)) / fundamental);
		let max_slope_start = ((mtof(ftom(self.max) - self.width)) / fundamental);

		// blend partials
		

		out
	}

}
