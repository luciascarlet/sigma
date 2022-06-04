pub mod note_values;
pub use note_values::ftom;
pub use note_values::mtof;

pub mod interpolation;
pub use interpolation::smooth_step;
pub use interpolation::lerp;
pub use interpolation::lerp_partials;