#![doc = include_str!("../README.md")]

mod dataset;
pub mod download_data;
mod error;
pub mod metadata;
mod prelude;
pub mod utils;

pub use dataset::datasettype::{
    Author, Dataset, Elevation, Extent, Geographic, Institution, Temporal,
};
