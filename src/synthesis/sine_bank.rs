use super::lut_sine::LUTSine;
use super::Partial;

mod output;

pub struct SineBank {
	partials: Vec<Partial>,
	num_partials: usize,
	phases: Vec<f64>,
	fundamental: f64,
	sample_rate: f64,
	sine: LUTSine,
}

impl SineBank {
	pub fn new(num_partials: usize, fundamental: f64, sample_rate: f64) -> SineBank {
		SineBank {
			partials: vec![Partial {ratio: 1.0, phase: 0.0, amp: 0.0, pan: 0.0,}; num_partials],
			num_partials,
			phases: vec![0.0; num_partials],
			fundamental,
			sample_rate,
			sine: LUTSine::new(512),
		}
	}

	pub fn get_fundamental(&self) -> f64 {
		self.fundamental
	}

	pub fn set_fundamental(&mut self, fundamental: f64) {
		self.fundamental = fundamental;
	}

	pub fn get_num_partials(&self) -> usize {
		self.num_partials
	}

	pub fn set_partials(&mut self, partials: Vec<Partial>) {
		self.partials = partials.clone();
		self.num_partials = partials.len();
	}

	pub fn set_partial_ratio(&mut self, partial: usize, ratio: f64) {
		self.partials[partial].ratio = ratio;
	}

	pub fn set_partial_phase(&mut self, partial: usize, phase: f64) {
		self.partials[partial].phase = phase;
	}

	pub fn set_partial_amp(&mut self, partial: usize, amp: f64) {
		self.partials[partial].amp = amp;
	}

	pub fn set_partial_pan(&mut self, partial: usize, pan: f64) {
		self.partials[partial].pan = pan;
	}

	pub fn get_partial(&self, partial: usize) -> &Partial {
		&self.partials[partial]
	}

	pub fn get_partials(&self) -> &Vec<Partial> {
		&self.partials
	}

	// these are mostly there to make processing only specific aspects of partials easier,
	// as a lot of filters may only care about the ratios or the amplitudes
	pub fn get_ratios(&self) -> Vec<f64> {
		self.partials.iter().map(|p| p.ratio).collect::<Vec<_>>()
	}

	pub fn set_ratios(&mut self, ratios: &[f64]) {
		for (partial, ratio) in self.partials.iter_mut().zip(ratios) {
			partial.ratio = *ratio;
		}
	}

	pub fn get_phases(&self) -> Vec<f64> {
		self.partials.iter().map(|p| p.phase).collect::<Vec<_>>()
	}

	pub fn set_phases(&mut self, phases: &[f64]) {
		for (partial, phase) in self.partials.iter_mut().zip(phases) {
			partial.phase = *phase;
		}
	}

	pub fn get_amps(&self) -> Vec<f64> {
		self.partials.iter().map(|p| p.amp).collect::<Vec<_>>()
	}

	pub fn set_amps(&mut self, amps: &[f64]) {
		for (partial, amp) in self.partials.iter_mut().zip(amps) {
			partial.amp = *amp;
		}
	}

	pub fn get_pans(&self) -> Vec<f64> {
		self.partials.iter().map(|p| p.pan).collect::<Vec<_>>()
	}

	pub fn set_pans(&mut self, pans: &[f64]) {
		for (partial, pan) in self.partials.iter_mut().zip(pans) {
			partial.pan = *pan;
		}
	}
}
