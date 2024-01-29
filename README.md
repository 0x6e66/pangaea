# A rust module to to access [PANGAEA](https://www.pangaea.de/) (meta)data

## Getting started
Run `cargo add pangaea` or add `pangaea = "0.1.0"` to your dependencies

## Get metadata for a specific PANGAEA dataset
```rust
use std::{fs::File, io::Write};
use pangaea::elastic::elastic_doc;

#[tokio::main]
pub async fn main() {
    let dataset_id = 820322;
    let metadata = elastic_doc(dataset_id).await.unwrap();

    let mut file = File::create(format!("pangaea-dataset-{}.json", dataset_id)).unwrap();
    let json = serde_json::to_string(&metadata).unwrap();
    write!(file, "{}", json).unwrap();
}
```

## Search for multiple datasets
```rust
use std::{fs::File, io::Write};
use pangaea::elastic::elastic_search;

#[tokio::main]
pub async fn main() {
    let res = elastic_search(0, 10, None, &["sp-lastModified:desc"])
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
