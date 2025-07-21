use anyhow::{Context as _, Result};
use futures::future::JoinAll;
use std::path::PathBuf;

pub(crate) struct Download {
    url: String,
    dest: PathBuf,
}

impl Download {
    pub(crate) fn new(url: String, dest: PathBuf) -> Self {
        Self { url, dest }
    }

    async fn try_perform(self) -> Result<()> {
        let image = reqwest::get(&self.url)
            .await
            .context("failed to request image")?
            .bytes()
            .await
            .context("failed to read image from response")?;

        tokio::fs::write(&self.dest, image)
            .await
            .with_context(|| format!("failed to write downloaded image to {:?}", self.dest))?;

        if let Some(s) = self.dest.to_str() {
            println!("{s}");
        }

        Ok(())
    }

    async fn perform(self) {
        if let Err(err) = self.try_perform().await {
            log::error!("{err:?}");
        }
    }

    pub(crate) async fn perform_many(downloads: impl Iterator<Item = Download>) {
        downloads.map(|d| d.perform()).collect::<JoinAll<_>>().await;
    }
}
