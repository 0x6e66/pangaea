use pangaea::{dataset::datasettype::Dataset, utils::get_elastic_client};

#[tokio::test]
async fn dataset_new() {
    let dataset_id = 82037;

    let _ = Dataset::new(dataset_id, &get_elastic_client().unwrap()).await.unwrap();
}

#[tokio::test]
async fn dataset_search() {
    let _ = Dataset::search(0, 10, None, &["sp-lastModified:desc"], &get_elastic_client().unwrap())
        .await
        .unwrap();
}
