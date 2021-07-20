pub mod error;
pub use error::{GSError, GSResult};

pub mod load;
pub use load::load_from_file;
