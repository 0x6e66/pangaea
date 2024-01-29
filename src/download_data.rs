//! Access the data from a specific dataset

use std::{fs::File, io::Write};

use futures_util::StreamExt;
use reqwest::{Client, Response};

use crate::prelude::*;

/// Download the actual data that is accociated with the dataset with the id `dataset_id`.
/// The data will saved to `filename.zip` or `filename.txt`, depending on the filetype.
pub async fn download_data_by_id(dataset_id: u32, filename: impl Into<String>) -> Result<()> {
    let base_url = format!("https://doi.pangaea.de/10.1594/PANGAEA.{}", dataset_id);

    let mut filename: String = filename.into();

    let client = Client::new();
    let response = client
        .get(&base_url)
        .query(&[("format", "zip")])
        .send()
        .await?;

    let correct_response = match response.status().is_success() {
        true => {
            filename.push_str(".zip");
            response
        }
        false => {
            filename.push_str(".txt");
            client
                .get(base_url)
                .query(&[("format", "textfile")])
                .send()
                .await?
        }
    };

    if !correct_response.status().is_success() {
        return Err(Error::Generic(format!(
            "Unable to locate data for dataset with id {}",
            dataset_id
        )));
    }

    write_response_to_file(correct_response, filename).await
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
