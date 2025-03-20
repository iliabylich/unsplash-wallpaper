use anyhow::{Context as _, Result};

pub(crate) async fn download_one(url: String, dest_path: String) -> Result<()> {
    let image = reqwest::get(url)
        .await
        .context("failed to request image")?
        .bytes()
        .await
        .context("failed to read image from response")?;

    tokio::fs::write(&dest_path, image)
        .await
        .with_context(|| format!("failed to write downloaded image to {dest_path}"))?;

    println!("{dest_path}");

    Ok(())
}
