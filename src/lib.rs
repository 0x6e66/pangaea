//! A rust module to to access [PANGAEA](https://www.pangaea.de/) (meta)data
//!
//! ## Getting started
//! Run `cargo add pangaea` or add `pangaea = "0.1.0"` to your dependencies
//!
//!
//! ## Get metadata for a specific PANGAEA dataset
//! ```
//! use std::{fs::File, io::Write};
//! use pangaea::elastic::elastic_doc;
//!
//! #[tokio::main]
//! pub async fn main() {
//!     let dataset_id = 820322;
//!     let metadata = elastic_doc(dataset_id).await.unwrap();
//!     let json = serde_json::to_string(&metadata).unwrap();
//! }
//! ```
//!
//! ## Search for multiple datasets
//! ```
//! use pangaea::elastic::elastic_search;
//!
//! #[tokio::main]
//! pub async fn main() {
//!     let res = elastic_search(0, 10, None, &["sp-lastModified:desc"])
//!         .await
//!         .unwrap();
//!
//!     res.into_iter()
//!         .filter_map(|md_res| md_res.ok())
//!         .for_each(|md| {
//!             let json = serde_json::to_string(&md).unwrap();
//!         });
//! }
//! ```

pub mod download_data;
pub mod elastic;
mod error;
mod prelude;
pub mod types;
