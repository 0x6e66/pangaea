#![doc = include_str!("../README.md")]

pub mod dataset;
pub mod download_data;
mod error;
pub mod metadata;
mod prelude;
pub mod utils;

pub use dataset::Dataset;
pub use metadata::MetaDataType;