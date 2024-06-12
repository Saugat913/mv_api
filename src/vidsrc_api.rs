use reqwest;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct APIItem {
    #[serde(rename = "type")]
    item_type: String,
    title: String,
    pub imdb_id: String,
    embed_url_imdb: String,
    pub tmdb_id: Option<String>,
    embed_url_tmdb: Option<String>,

    number: Option<i32>,
    season: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
struct APIResult {
    page: i32,
    items: Vec<APIItem>,
}

#[derive(Serialize, Deserialize, Debug)]
struct APIResponse {
    status: i32,
    result: APIResult,
}

pub async fn get_new_movies() -> Vec<APIItem> {
    let body: APIResponse = reqwest::get("https://vidsrc.to/vapi/tv/new")
        .await
        .unwrap()
        .json::<APIResponse>()
        .await
        .unwrap();

    if body.status == 200 {
        let items = body.result.items;

        return items;
    }
    return vec![];
}
