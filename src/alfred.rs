use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Item {
    pub title: String,
    pub arg: String,
}

#[derive(Serialize, Deserialize)]
pub struct ListResult {
    pub items: Vec<Item>,
}
