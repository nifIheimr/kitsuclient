
use reqwest::Error;
use serde::{Deserialize, Serialize};

use crate::model::anime::Anime; 

#[derive(Serialize, Deserialize)]
 pub struct KitsuResponse {
    pub data: Vec<Anime>
}

pub async fn get_animes() -> Result<KitsuResponse, Error> {
    let response = reqwest::get("https://kitsu.io/api/edge/trending/anime")
    .await?
    .json::<KitsuResponse>()
    .await?;

    Ok(response)
}

