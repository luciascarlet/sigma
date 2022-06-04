use crate::synthesis::Partial;
use crate::operators::FilterOp;

pub trait MetaOp {
	fn process(&self, pre: Vec<Partial>, filter: &dyn FilterOp, aux: Option<Vec<Partial>>, fundamental: f64) -> Vec<Partial>;
}