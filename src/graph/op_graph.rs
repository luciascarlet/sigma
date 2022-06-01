use crate::operators::{filter_op::FilterOp, source_op::SourceOp};
use crate::synthesis::partial::Partial;

// i have no fucking idea how lifetimes work i just accepted the quick fixes
// wait i don't actually think i need to give the graph a sine bank
pub struct OpGraph {
	num_partials: usize,
	fundamental: f64,
	source_ops: Vec<Box<dyn SourceOp>>,
	filter_ops: Vec<Box<dyn FilterOp>>,
}

impl OpGraph {
	fn new(num_partials: usize, fundamental: f64) -> OpGraph {
		OpGraph {
			num_partials,
			fundamental,
			source_ops: Vec::new(),
			filter_ops: Vec::new(),
		}
	}

	fn get_fundamental(&self) -> f64 {
		self.fundamental
	}

	fn set_fundamental(&mut self, fundamental: f64) {
		self.fundamental = fundamental;
	}

	fn add_source_op(&mut self, source_op: Box<dyn SourceOp>) {
		self.source_ops.push(source_op);
	}

	fn add_filter_op(&mut self, filter_op: Box<dyn FilterOp>) {
		self.filter_ops.push(filter_op);
	}

	fn get_source_ops(&self) -> &Vec<Box<dyn SourceOp>> {
		&self.source_ops
	}

	fn get_filter_ops(&self) -> &Vec<Box<dyn FilterOp>> {
		&self.filter_ops
	}

	fn reorder_source_ops(&mut self, from: usize, to: usize) {
		let source_op = self.source_ops.remove(from);
		self.source_ops.insert(to, source_op);
	}

	fn reorder_filter_ops(&mut self, from: usize, to: usize) {
		let filter_op = self.filter_ops.remove(from);
		self.filter_ops.insert(to, filter_op);
	}

	fn remove_source_op(&mut self, idx: usize) {
		self.source_ops.remove(idx);
	}

	fn remove_filter_op(&mut self, idx: usize) {
		self.filter_ops.remove(idx);
	}

	fn run(&self) -> Vec<Partial> {
		// create blank set of partials
		let mut partials = vec![
			Partial {
				ratio: 1.0,
				phase: 0.0,
				amp: 0.0,
				pan: 0.0,
			};
			self.num_partials
		];

		// add partials from source ops
		// i should change iter to iter_mut, looks better lmao, but i'm too lazy
		for (i, op) in self.source_ops.iter().enumerate() {
			let partial = op.process_partial(i, self.fundamental);
			partials[i].amp += partial.amp;
			partials[i].ratio += partial.ratio;
			// oh god this one is hard
			partials[i].phase = partials[i].phase + partial.phase % 1.0; // should i maybe average them or something? i cba counting the amount of ops though so fuck this, don't care for now anyway
															 // who the fuck is even going to combine source ops anyway? why the fuck am i implementing this shit
			partials[i].pan += partial.pan.clamp(-1.0, 1.0);
		}

		// sequentially process filter ops
		for op in &self.filter_ops {
			for (p_idx, p) in partials.iter_mut().enumerate() {
				*p = op.transform(*p, p_idx, self.fundamental);
			}
		}

		partials
	}
}
