use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ItemDetail {
    pub data_field: String,
    pub correct_field_name: String,
}

pub async fn get_item() {
    let res = reqwest::get("http://localhost:3000/items/1")
        .await
        .expect("Failed to send request")
        .json::<ItemDetail>()
        .await
        .expect("Fail to parse response");

    println!("{res:?}");
}