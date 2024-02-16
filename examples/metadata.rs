use pangaea::{metadata::MetaDataType, utils::get_elastic_client};
use std::{fs::File, io::Write};

#[tokio::main]
async fn main() {
    metadata_new().await;
    metadata_search().await;
}

async fn metadata_new() {
    let dataset_id = 82037;

    let metadata = MetaDataType::new(dataset_id, &get_elastic_client().unwrap())
        .await
        .unwrap();

    let mut file = File::create(format!("pangaea-metadata-{}.json", dataset_id)).unwrap();
    let json = serde_json::to_string(&metadata).unwrap();
    write!(file, "{}", json).unwrap();
}

async fn metadata_search() {
    let res = MetaDataType::search(
        0,
        10,
        None,
        &["sp-lastModified:desc"],
        &get_elastic_client().unwrap(),
    )
    .await
    .unwrap();

    let mut file = File::create(format!("pangaea-metadata.jsonl")).unwrap();

    res.into_iter()
        .filter_map(|md_res| md_res.ok())
        .for_each(|md| {
            let json = serde_json::to_string(&md).unwrap();
            writeln!(file, "{}", json).unwrap();
        });
}
