use crate::unsplash::Params;
use anyhow::{Context as _, Result};

pub(crate) struct Client;

impl Client {
    const URL: &str = "https://api.unsplash.com/photos/random";

    pub(crate) async fn call(token: &str, params: &Params) -> Result<Vec<String>> {
        let client = reqwest::Client::new();

        let images = client
            .get(Self::URL)
            .header("Authorization", format!("Client-ID {token}"))
            .query(&[
                ("collections", params.collections.clone()),
                ("count", format!("{}", params.count)),
            ])
            .send()
            .await
            .context("failed to send API request")?
            .json::<response::Response>()
            .await
            .context("unexpected response from Unsplash API")?;

        let urls = images
            .into_iter()
            .map(|image| image.urls.raw)
            .collect::<Vec<_>>();

        Ok(urls)
    }
}

mod response {
    use serde::Deserialize;

    pub(crate) type Response = Vec<Image>;

    #[derive(Deserialize)]
    pub(crate) struct Image {
        pub(crate) urls: Urls,
    }

    #[derive(Deserialize)]
    pub(crate) struct Urls {
        pub(crate) raw: String,
    }
}
