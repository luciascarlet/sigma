impl SineBank {
	pub fn tick(&self) {
		&self.partials.iter().map(|p| {
			p.phase = (p.phase + (p.ratio * &self.fundamental) / &self.sample_rate) % 1.0;
		});
	}
	
	pub fn sample(&self) {
		let (mut sum_l, mut sum_r): f64;
		&self.partials.iter().map(|p| {
			
		});
	}
}