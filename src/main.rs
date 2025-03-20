#![doc = include_str!("../README.md")]

mod dirs;
pub(crate) use dirs::Dirs;

mod token;
use futures::future::join_all;
pub(crate) use token::Token;

mod unsplash;
pub(crate) use unsplash::{Client, Params};

mod download;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let token = Token::read().await?;
    let params = Params::read().await?;
    let urls = Client::call(&token, &params).await?;
    let dest_dir = params.dest_dir()?;

    let mut requests = vec![];
    for (idx, url) in urls.into_iter().enumerate() {
        let dest_path = format!("{}/{}.jpeg", dest_dir, idx + 1);
        let request = download::download_one(url, dest_path);
        requests.push(request);
    }

    join_all(requests).await;

    Ok(())
}
