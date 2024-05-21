use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Anime {
    pub id: String,
    pub attributes: Attributes,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attributes {
    pub slug: String,
    pub titles: Titles,
    pub canonical_title: String,
    pub average_rating: String,
    #[serde(rename = "nextRelease")]
    pub next_release: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Titles {
    pub en: Option<String>,
    #[serde(rename = "ja_jp")]
    pub ja_jp: String,
}