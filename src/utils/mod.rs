//! utils module

use crate::prelude::*;
use chrono::{DateTime, Utc};
use elasticsearch::{http::transport::Transport, Elasticsearch};

/// crate a client to the pangaea elasticsearch instance
pub fn get_elastic_client() -> Result<Elasticsearch> {
    let transport = Transport::single_node("https://ws.pangaea.de/es/pangaea")?;
    Ok(Elasticsearch::new(transport))
}

pub fn string_to_datetime(dt: String) -> Option<DateTime<Utc>> {
    DateTime::parse_from_rfc3339(format!("{}+01:00", dt).as_str())
            .ok()
            .map(|dt| dt.to_utc())
}