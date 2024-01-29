use std::{fs::File, io::Write};

use pangaea_rs::elastic::elastic_doc;

#[tokio::main]
pub async fn main() {
    let dataset_id = 820322;
    let metadata = elastic_doc(dataset_id).await.unwrap();

    let mut file = File::create(format!("pangaea-dataset-{}.json", dataset_id)).unwrap();
    let json = serde_json::to_string(&metadata).unwrap();
    write!(file, "{}", json).unwrap();
}
