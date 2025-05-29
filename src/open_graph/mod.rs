use std::collections::HashMap;

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use reqwest::Url;
        use scraper::{Html, Selector};
    }
}

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use anyhow::Result;

#[derive(Debug, Deserialize, Serialize)]
pub struct OpenGraph {
    #[serde(rename = "og:title")]
    pub title: String,

    #[serde(rename = "og:type")]
    pub og_type: String,

    #[serde(rename = "og:image")]
    pub image: String,

    #[serde(rename = "og:url")]
    pub url: String,

    #[serde(rename = "og:description")]
    pub description: Option<String>,

    #[serde(rename = "og:site_name")]
    pub site_name: Option<String>,

    #[serde(rename = "og:locale")]
    pub locale: Option<String>,
}

#[cfg(feature = "ssr")]
pub async fn fetch_op_data(url: Url) -> Result<OpenGraph> {
    let html = reqwest::get(url).await?.text().await?;
    let document = Html::parse_document(&html);
    let selector = Selector::parse(r#"meta[property^="og:"]"#).unwrap();

    let mut metadata = HashMap::new();

    for tag in document.select(&selector) {
        let value = tag.value();
        if let (Some(prop), Some(content)) = (value.attr("property"), value.attr("content")) {
            metadata.insert(prop.to_string(), content.to_string());
        }
    }

    let json_value: Value = json!(metadata);

    Ok(serde_json::from_value(json_value)?)
}
