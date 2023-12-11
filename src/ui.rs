#![allow(unused_attributes)]

use rocket::{get, response::content::RawHtml};
use std::io::Error;

#[get("/")]
pub async fn index() -> Result<RawHtml<String>, Error> {
    Ok(RawHtml(
        r#"
        <!DOCTYPE html>
        <html>
        
        <head>
          <title>MacRemote UI</title>
          <meta name="viewport" content="width=device-width, initial-scale=1" />
        </head>
        <style>
          * {
            font-family: 'DejaVu Sans', Arial;
          }
        
          body {
            margin: 0;
            background-color: black;
          }
        
          .vertical-text {
            writing-mode: vertical-rl;
          }
        
          .banner {
            background-color: black;
            color: white;
            padding: 10px;
            text-align: center;
            font-size: 2.5em;
          }
        
          .small-button {
            font-size: 1.0em;
            color: #ffd700;
            text-decoration: none;
          }
        
          .divider {
            background-color: #ffd700;
            height: 3px;
            margin-bottom: 10px;
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
            <a href="/media/lefttab" onclick="sendRequest(event)" class="small-button">&#5130;</a>
            <a href="/media/righttab" onclick="sendRequest(event)" class="small-button">&#5125;</a>
            <a href="/media/closetab" onclick="sendRequest(event)" class="small-button">&#10006;</a>
            <a href="/media/changewindow" onclick="sendRequest(event)" class="small-button">&#128260;</a>
            <a href="/media/openfile" onclick="sendRequest(event)" class="small-button">&#x1F50D;</a>
            <a href="/media/closewindow" onclick="sendRequest(event)" class="small-button">&#x274C;</a>
          </div>
          <div class="divider"></div>
          <div class="buttons">
            <a href="/media/maximize" onclick="sendRequest(event)" class="button">max</a>
            <a href="/media/volumeup" onclick="sendRequest(event)" class="button">
              <span class="vertical-text">&#x276E;</span>
            </a>
            <a href="/media/pause" onclick="sendRequest(event)" class="button">II</a>
            <a href="/media/volumedown" onclick="sendRequest(event)" class="button">
              <span class="vertical-text">&#x276F;</span>
            </a>
            <a href="/media/backward" onclick="sendRequest(event)" class="button">&#x276E;</a>
            <a href="/media/forward" onclick="sendRequest(event)" class="button">&#x276F;</a>
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
