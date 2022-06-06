use super::SineBank;

impl SineBank {
	// This is Slow as Fuck! :D
	pub fn tick(&mut self) {
		for (i, p) in self.partials.iter().enumerate() {
			self.phases[i] += self.fundamental * p.ratio / self.sample_rate;
		}
	}

	pub fn retrigger(&mut self) {
		for (i, p) in self.partials.iter().enumerate() {
			self.phases[i] = p.phase;
		}
	}

	pub fn sample(&self) -> (f64, f64) {
		let (mut sum_l, mut sum_r): (f64, f64) = (0.0, 0.0);
		for (i, p) in self.partials.iter().enumerate() {
			// check if partial is valid
			// ratio must be > 0 and resulting frequency under Nyquist, amp must be > 0
			if p.ratio > 0.0 && p.ratio * self.fundamental < self.sample_rate / 2.0 && p.amp > 0.0 {
				// can't be arsed to work out a panning law or whatever for this,
				// so this is going to be way too fucking loud if pan is not 0
				// but ig just do mid/side and reduce the volume of the sides at the end to shut it up lmfao
				let smp = self.sine.sample(self.phases[i]);
				sum_l += smp * p.amp + p.pan;
				sum_r += smp * p.amp - p.pan;
			}
		}
		(sum_l, sum_r)
	}

	// this does not check any of the partials for validity, even for Nyquist, so sanitise the partials before calling this
	// however, this might be faster, as it skips all of the checks
	pub fn sample_unchecked(&self) -> (f64, f64) {
		let (mut sum_l, mut sum_r): (f64, f64) = (0.0, 0.0);
		for (i, p) in self.partials.iter().enumerate() {
			let smp = self.sine.sample(self.phases[i]);
				sum_l += smp * p.amp + p.pan;
				sum_r += smp * p.amp - p.pan;
		}
		(sum_l, sum_r)
	}
}
