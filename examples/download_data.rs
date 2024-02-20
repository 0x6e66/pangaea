use pangaea::download_data::download_data_by_id;

#[tokio::main]
async fn main() {
    download_zip().await;
    download_txt().await;
}

async fn download_zip() {
    let id = 921673;
    download_data_by_id(id, None, "downloaded_file")
        .await
        .unwrap();
}

async fn download_txt() {
    let id = 839111;
    download_data_by_id(id, None, "downloaded_file")
        .await
        .unwrap();
}
