pub mod decide;
pub mod quarantine;

pub use decide::{accept, reject};
pub use quarantine::open_quarantine;
