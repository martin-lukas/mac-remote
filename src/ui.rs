#![allow(unused_attributes)]

use rocket::{get, response::content::RawHtml};
use std::io::Error;

#[get("/ui")]
pub async fn index() -> Result<RawHtml<String>, Error> {
    Ok(RawHtml(
        r#"<!DOCTYPE html>
        <html>
          <head>
            <meta name="viewport" content="width=device-width, initial-scale=1" />
          </head>
          <style>
            body {
              margin: 0;
              font-family: Arial, sans-serif;
              background-color: black;
            }
            .banner {
              background-color: black;
              color: white;
              padding: 30px;
              text-align: center;
              font-size: 2.5em;
            }
            .divider {
              background-color: #ffd700;
              height: 3px;
              margin-bottom: 30px;
            }
            .buttons {
              display: grid;
              grid-template-columns: repeat(2, 1fr);
              gap: 30px;
              padding: 20px;
            }
            .button {
              background-color: #ffd700;
              color: black;
              display: flex;
              align-items: center;
              justify-content: center;
              text-decoration: none;
              font-size: 4em;
              font-weight: bold;
              height: calc((100vw - 80px) / 2);
              width: calc((100vw - 80px) / 2);
              border-radius: 50%;
            }
          </style>
          <body>
            <div class="banner">
              VLC - Remote
            </div>
            <div class="divider"></div>
            <div class="buttons">
              <a href="/media/maximize" onclick="sendRequest(event)" class="button">MX</a>
              <a href="/media/volumeup" onclick="sendRequest(event)" class="button">/\</a>
              <a href="/media/pause" onclick="sendRequest(event)" class="button">II</a>
              <a href="/media/volumedown" onclick="sendRequest(event)" class="button">\/</a>
              <a href="/media/backward" onclick="sendRequest(event)" class="button">⫷</a>
              <a href="/media/forward" onclick="sendRequest(event)" class="button">⫸</a>
            </div>
            <script>
              function sendRequest(event) {
                event.preventDefault(); // prevent navigating to the link
                const url = event.currentTarget.href; // get the URL from the clicked link
                fetch(url) // send an HTTP GET request to the URL
                  .then((response) => response.text()) // parse the response as text
                  .then((text) => console.log(text)) // log the response text to the console
                  .catch((error) => console.error(error)); // log any errors to the console
              }
            </script>
          </body>
        </html>
        "#
        .to_string(),
    ))
}
