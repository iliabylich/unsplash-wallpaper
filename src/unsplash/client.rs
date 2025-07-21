use crate::unsplash::Params;
use anyhow::{Context as _, Result};
use serde::Deserialize;

pub(crate) struct Client;

impl Client {
    const URL: &str = "https://api.unsplash.com/photos/random";

    pub(crate) async fn call(token: &str, params: &Params) -> Result<Vec<String>> {
        let client = reqwest::Client::new();

        let images = client
            .get(Self::URL)
            .header("Authorization", format!("Client-ID {token}"))
            .query(&[
                ("collections", &params.collections),
                ("count", &params.count.to_string()),
            ])
            .send()
            .await
            .context("failed to send API request")?
            .json::<Vec<Image>>()
            .await
            .context("unexpected response from Unsplash API")?;

        let urls = images
            .into_iter()
            .map(|image| image.urls.raw)
            .collect::<Vec<_>>();

        Ok(urls)
    }
}

#[derive(Deserialize)]
struct Image {
    urls: Urls,
}

#[derive(Deserialize)]
struct Urls {
    raw: String,
}
