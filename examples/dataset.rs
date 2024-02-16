use pangaea::{dataset::Dataset, utils::get_elastic_client};
use std::{fs::File, io::Write};

#[tokio::main]
async fn main() {
    dataset_new().await;
    dataset_search().await;
}

async fn dataset_new() {
    let dataset_id = 82037;

    let dataset = Dataset::new(dataset_id, &get_elastic_client().unwrap())
        .await
        .unwrap();

    let mut file = File::create(format!("pangaea-dataset-{}.json", dataset_id)).unwrap();
    let json = serde_json::to_string(&dataset).unwrap();
    write!(file, "{}", json).unwrap();
}

async fn dataset_search() {
    let res = Dataset::search(
        0,
        10,
        None,
        &["sp-lastModified:desc"],
        &get_elastic_client().unwrap(),
    )
    .await
    .unwrap();

    let mut file = File::create(format!("pangaea-datasets.jsonl")).unwrap();

    res.into_iter()
        .filter_map(|ds_res| ds_res.ok())
        .for_each(|ds| {
            let json = serde_json::to_string(&ds).unwrap();
            writeln!(file, "{}", json).unwrap();
        });
}
