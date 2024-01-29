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
