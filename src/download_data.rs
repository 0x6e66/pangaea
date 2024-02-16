//! Access the data from a specific dataset

use std::{fs::File, io::Write};

use futures_util::StreamExt;
use reqwest::{
    header::{HeaderMap, HeaderValue, AUTHORIZATION},
    Client, Response,
};

use crate::prelude::*;

/// Download the actual data that is associated with the dataset with the id `dataset_id`.
/// The data will saved to `filename.zip` or `filename.txt`, depending on the filetype.
pub async fn download_data_by_id(
    dataset_id: u32,
    bearer_token: Option<String>,
    filename: impl Into<String>,
) -> Result<()> {
    let base_url = format!("https://doi.pangaea.de/10.1594/PANGAEA.{}", dataset_id);

    let mut filename: String = filename.into();

    let mut headers = HeaderMap::new();
    if let Some(bearer_token) = bearer_token {
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", bearer_token)).map_err(|inv_header_val| {
                Error::Generic(format!(
                    "Could not convert the provided bearer token '{}' to a valid HeaderValue. Message: '{}'",
                    bearer_token,
                    inv_header_val
                ))
            })?,
        );
    }

    let client = Client::new();

    let response = client
        .get(&base_url)
        .query(&[("format", "zip")])
        .headers(headers.clone())
        .send()
        .await?;
    if response.status().is_success() {
        filename.push_str(".zip");
        return write_response_to_file(response, filename).await;
    }

    let response = client
        .get(&base_url)
        .query(&[("format", "textfile")])
        .headers(headers)
        .send()
        .await?;
    if response.status().is_success() {
        filename.push_str(".txt");
        return write_response_to_file(response, filename).await;
    }

    return Err(Error::Generic(format!(
        "Unable to locate data for dataset with id '{}'",
        dataset_id
    )));
}

async fn write_response_to_file(response: Response, path: impl Into<String>) -> Result<()> {
    let mut file = File::create(path.into())?;
    let mut stream = response.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item?;
        file.write_all(&chunk)?;
    }
    file.flush()?;

    Ok(())
}
