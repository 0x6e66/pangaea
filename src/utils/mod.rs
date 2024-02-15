use elasticsearch::{http::transport::Transport, Elasticsearch};
use crate::prelude::*;

pub fn get_elastic_client() -> Result<Elasticsearch> {
    let transport = Transport::single_node("https://ws.pangaea.de/es/pangaea")?;
    Ok(Elasticsearch::new(transport))
}