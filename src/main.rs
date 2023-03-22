use core::time;

use std::{io, thread};

use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue, ACCEPT, USER_AGENT},
    Client,
};
use std::fs;
use tokio;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let token = fs::read_to_string("./token").unwrap();
    println!("Github Analyzer - Initializing");
    let sleep_time = time::Duration::from_millis(2000);
    let mut buffer = String::new();

    io::stdin().read_line(&mut buffer).unwrap();
    // clear terminal
    println!("{}[2J", 27 as char); // Figure out how this works fool
    thread::sleep(sleep_time);
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // Not sure how this works look into me
                                                    // end clear terminal

    let api_url = format!(
        "https://api.github.com/users/{username}/repos",
        username = &buffer.trim_end()
    );
    println!("{}", api_url);

    // Now lets jump into some API hijinx

    // Lets set the headers appropriately
    let mut headers = HeaderMap::new();
    headers.insert(
        ACCEPT,
        HeaderValue::from_static("application/vnd.github+json"),
    );
    headers.insert(
        "X-GitHub-Api-Version",
        HeaderValue::from_static("2022-11-28"),
    );
    headers.insert(USER_AGENT, HeaderValue::from_static("KauMah"));
    // the headers have been created, so now lets goo!
    // Aaaaaaaand it breaks

    let req = Client::new()
        .get(&api_url)
        .headers(headers)
        .bearer_auth(token.trim_end())
        .send()
        .await?
        .text()
        .await?;
    println!("{:#?}", req);
    Ok(())
}
