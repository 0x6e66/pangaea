use pangaea::{metadata::metadatatype::MetaDataType, utils::get_elastic_client};

#[tokio::test]
async fn metadata_new() {
    let dataset_id = 82037;

    let _ = MetaDataType::new(dataset_id, &get_elastic_client().unwrap()).await.unwrap();
}

#[tokio::test]
async fn metadata_search() {
    let _ = MetaDataType::search(0, 10, None, &["sp-lastModified:desc"], &get_elastic_client().unwrap())
        .await
        .unwrap();
}
