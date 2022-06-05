// pub mod adjustable;
// ^lol. lmao, even

pub mod dual_op;
pub mod filter_op;
pub mod source_op;
pub mod meta_op;

pub mod ops;


pub use dual_op::DualOp;
pub use filter_op::FilterOp;
pub use source_op::SourceOp;
pub use meta_op::MetaOp;
pub use ops::*;