#![allow(unused_attributes)]

use rocket::{get, response::content::RawHtml};
use std::{io::Error, path::Path};
use tokio::fs::read_to_string;

#[get("/ui")]
pub async fn index() -> Result<RawHtml<String>, Error> {
    let path = Path::new("web/index.html");
    let contents = read_to_string(path).await?;
    Ok(RawHtml(contents))
}
