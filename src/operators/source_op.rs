use crate::synthesis::partial::Partial;

pub trait SourceOp {
	fn process(&self, num_partials: usize, fundamental: f64) -> Vec<Partial>;
	
}
