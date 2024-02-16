//! utils module

use crate::prelude::*;
use elasticsearch::{http::transport::Transport, Elasticsearch};

/// crate a client to the pangaea elasticsearch instance
pub fn get_elastic_client() -> Result<Elasticsearch> {
    let transport = Transport::single_node("https://ws.pangaea.de/es/pangaea")?;
    Ok(Elasticsearch::new(transport))
}
