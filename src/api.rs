use reqwest::Error;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct SimilarWords {
    pub similar_words: Vec<String>,
}

pub async fn get_similar_words(api_similar_endpoint: String) -> Result<SimilarWords, Error> {
    let request_url = format!("{}", api_similar_endpoint);
    let response = reqwest::get(&request_url).await?.text().await?;

    let w: SimilarWords = serde_json::from_str(&response).unwrap();
    return Ok(w);
}
