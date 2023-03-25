use core::time;

use reqwest::{
    header::{HeaderMap, HeaderValue, ACCEPT, USER_AGENT},
    Client,
};
use serde_json::Value;
use std::fs;
use std::{io, thread};
use tokio::{self};

// async fn get_data(username: &str, ) -> Result<String, Error> {}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let token = fs::read_to_string("./token")
        .expect("Failed to read file \"./token\". Are you sure it's there?");
    println!("Initializing - Github Analyzer...");
    let sleep_time = time::Duration::from_millis(1000);
    let mut username = String::new();
    thread::sleep(sleep_time);
    println!("Enter a Github Username:\n");
    io::stdin()
        .read_line(&mut username)
        .expect("Failed to read line from file: ./token");

    // clear terminal
    println!("{}[2J", 27 as char); // Figure out how this works fool
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // Not sure how this works look into me
                                                    // end clear terminal

    let api_url = format!("https://api.github.com/users/{username}/repos",);
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

    let req = Client::new()
        .get(&api_url)
        .headers(headers)
        .bearer_auth(token.trim_end())
        .send()
        .await?
        .text()
        .await?;
    let js: Value = serde_json::from_str(&req).expect("This should just work");
    let pretty = serde_json::to_string_pretty(&js).expect("This should just work");
    // write to file for a lil test
    // println!("{:#?}", req);
    // let mut file = File::create("out.txt").await.expect("Please work lol");
    // file.write_all(pretty.as_bytes())
    //     .await
    //     .expect("Failed to write to file");
    println!("{}", &pretty);

    Ok(())
}
