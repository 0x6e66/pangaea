use pangaea::download_data::download_data_by_id;

#[tokio::main]
async fn main() {
    let id = 921673;
    // let id = 839111;
    download_data_by_id(id, "downloaded_file").await.unwrap();
}
