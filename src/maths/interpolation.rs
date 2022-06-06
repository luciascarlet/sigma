use crate::synthesis::Partial;

// simple 3rd-order smoothstep function
// TODO: unfuck this because CLEARLY IT DOES NOT FUCKING WORK WHAT THE FUCK I JUST FUCKING BLEW MY EARS OUT TO THIS FUCKING ABORTIOn
pub fn smooth_step(min: f64, max: f64, pos: f64) -> f64 {
	let mut x = pos.clone();
	match x {
		x if x < min => 0.0,
		x if x > max => 1.0,
		_ => {
			x = (x - min) / (max - min);
			x * x * (3.0 - 2.0 * x)
		}
	}
}

// literally just a linear interpolation lol
pub fn lerp(a: f64, b: f64, pos: f64) -> f64 {
	a * (1.0 - pos) + b * pos
}

// same shit but for partials
pub fn lerp_partials(a: Partial, b: Partial, pos: f64) -> Partial {
	let mut out = a.clone();
	out.amp = a.amp * (1.0 - pos) + b.amp * pos;
	out.ratio = a.ratio * (1.0 - pos) + b.ratio * pos;
	out.phase = a.phase * (1.0 - pos) + b.phase * pos;
	out.pan = a.pan * (1.0 - pos) + b.pan * pos;
	out
}
