pub fn mtof(midi: f64) -> f64 {
	440.0 * 2.0_f64.powf((midi - 69.0) / 12.0)
}

pub fn ftom(freq: f64) -> f64 {
	12.0 * (freq / 440.0).log2() + 69.0
}