pub mod data;
pub mod domain;
pub mod service;
pub mod web;

pub use domain::clip::field::ShortCode;
pub use domain::clip::ClipError;
pub use domain::time::Time;

//model::clip for database module bacuase it potentially has errors
//domain::clip

