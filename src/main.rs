use reqwest::{
    header::{HeaderMap, HeaderValue, ACCEPT, USER_AGENT},
    Client, RequestBuilder,
};
use serde_json::Value;
use std::error::Error;
use std::fs;
use std::io;
use tokio::{self};

fn make_request(token: &str, username: &str, page: i16) -> Result<RequestBuilder, Box<dyn Error>> {
    let api_url = format!(
        "https://api.github.com/users/{username}/repos?per_page=5&page={}",
        page
    );
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
    let req = Client::new()
        .get(&api_url)
        .headers(headers)
        .bearer_auth(token.trim_end());
    return Ok(req);
}

async fn get_git_urls(rb: RequestBuilder) -> Result<Vec<String>, Box<dyn Error>> {
    let mut urls = Vec::new();
    let res = rb.send().await?.text().await?;
    let json: Vec<Value> = serde_json::from_str(&res).expect("Failed to convert response to JSON");
    for repo in json.iter() {
        let url = repo
            .get("git_url")
            .expect("Failed to get 'git_url' from JSON object")
            .to_string();
        // println!("{}", &url[7..url.len() - 1]);
        urls.push(url[7..url.len() - 1].to_string());
    }

    Ok(urls)
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    println!("Initializing - Github Analyzer...");
    let token = fs::read_to_string("./token").expect("Could not read token form ./token");

    let mut username = String::new();
    println!("Enter a Github Username:\n");
    io::stdin()
        .read_line(&mut username)
        .expect("Something went wrong reading username from stdin");

    // clear terminal
    println!("{}[2J", 27 as char);
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    // end clear terminal

    let mut git_urls: Vec<String> = Vec::new();
    let mut page: i16 = 1;
    loop {
        let req =
            make_request(&token, &username, page).expect("Something went wrong building the URL");
        let new_urls = get_git_urls(req).await.expect("msg");
        if new_urls.len() == 0 {
            break;
        }
        for url in new_urls.iter() {
            println!("{}", url);
        }
        git_urls.extend(new_urls);
        page = page + 1;
        println!();
    }

    // let res = req.send().await?.text().await?;
    // let js: Value = serde_json::from_str(&res).expect("This should just work");
    // let pretty = serde_json::to_string_pretty(&js).expect("This should just work");

    // write to file for a lil test
    // println!("{:#?}", req);
    // let mut file = File::create("out.txt").await.expect("Please work lol");
    // file.write_all(pretty.as_bytes())
    //     .await
    //     .expect("Failed to write to file");
    // println!("{}", &pretty);

    Ok(())
}
