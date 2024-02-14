# A rust module to to access [PANGAEA](https://www.pangaea.de/) (meta)data

## Get metadata for a specific PANGAEA dataset
```rust
use std::{fs::File, io::Write};

use pangaea::dataset::datasettype::Dataset;
use elasticsearch::{Elasticsearch, http::transport::Transport};

#[tokio::main]
pub async fn main() {
    let dataset_id = 820322;
    let transport = Transport::single_node("https://ws.pangaea.de/es/pangaea").unwrap();
    let client = Elasticsearch::new(transport);

    let dateset = Dataset::new(dataset_id, &client).await.unwrap();

    let mut file = File::create(format!("pangaea-dataset-{}.json", dataset_id)).unwrap();
    let json = serde_json::to_string(&dateset).unwrap();
    write!(file, "{}", json).unwrap();
}
```

## Search for multiple datasets
```rust
use std::{fs::File, io::Write};

use pangaea::dataset::datasettype::Dataset;
use elasticsearch::{Elasticsearch, http::transport::Transport};

#[tokio::main]
pub async fn main() {
    let transport = Transport::single_node("https://ws.pangaea.de/es/pangaea").unwrap();
    let client = Elasticsearch::new(transport);

    let res = Dataset::search(0, 10, None, &["sp-lastModified:desc"], &client)
        .await
        .unwrap();

    let mut file = File::create(format!("pangaea-datasets.jsonl")).unwrap();

    res.into_iter()
        .filter_map(|md_res| md_res.ok())
        .for_each(|md| {
            let json = serde_json::to_string(&md).unwrap();
            writeln!(file, "{}", json).unwrap();
        });
}
```

## Download the actual data associated with the dataset
```rust
use pangaea::download_data::download_data_by_id;

#[tokio::main]
async fn main() {
    let id = 921673;
    download_data_by_id(id, "downloaded_file").await.unwrap();
}

```
The data will saved to `filename.zip` or `filename.txt`, depending on the datatype that is returned
