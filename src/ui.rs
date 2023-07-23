#![allow(unused_attributes)]

use rocket::{get, response::content};
use tokio::fs::read_to_string;
use std::path::Path;

#[get("/ui")]
pub async fn index() -> Result<content::RawHtml<String>, std::io::Error> {
    let path = Path::new("web/index.html");
    let contents = read_to_string(path).await?;
    Ok(content::RawHtml(contents))
}