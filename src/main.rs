#![doc = include_str!("../README.md")]

mod dirs;
mod download;
mod token;
mod unsplash;

use anyhow::Result;
use dirs::Dirs;
use download::Download;
use token::Token;
use unsplash::{Client, Params};

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let token = Token::read().await?;
    let params = Params::read().await?;
    let urls = Client::call(&token, &params).await?;
    let dest_dir = params.full_dest_dir();

    let filenames = (1..).map(|idx| {
        let filename = format!("unsplash-{}.jpeg", idx + 1);
        dest_dir.join(filename)
    });

    Download::perform_many(
        urls.into_iter()
            .zip(filenames)
            .map(|(url, dest)| Download::new(url, dest)),
    )
    .await;

    Ok(())
}
