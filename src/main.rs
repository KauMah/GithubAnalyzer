use reqwest::{
    blocking::{Client, RequestBuilder},
    header::{HeaderMap, HeaderValue, ACCEPT, USER_AGENT},
};
use serde_json::Value;
use std::error::Error;
use std::fs;
use std::io;
use tokio::{self, process::Command};

struct Commit {
    hash: String,
    timestamp_utc: u32, // This should give me plenty of headroom for the next 100 years haha
    added_locs: u16,
    removed_locs: u16,
}

struct Repo {
    repo_url: String,
    commits: Vec<Commit>,
}

fn get_repo_page(token: &str, username: &str, page: i16) -> Result<RequestBuilder, Box<dyn Error>> {
    let api_url = format!(
        "https://api.github.com/users/{username}/repos?per_page=20&page={}",
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

fn get_user_identifiers(token: &str, username: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let mut out: Vec<String> = Vec::new();
    let api_url = format!("https://api.github.com/users/{username}");
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
    let res = Client::new()
        .get(&api_url)
        .headers(headers)
        .bearer_auth(token.trim_end())
        .send()
        .expect("Request to /users/<username> failed")
        .text()
        .expect("Conversion to text failed for user");

    let val: Value = serde_json::from_str(&res)
        .expect("Failed to parse JSON from response for GET users/{username}");
    // let pretty = serde_json::to_string_pretty(&val).expect("pls");
    // println!("{}", pretty);
    let email = val
        .get("email")
        .expect("Failed to parse string <email> from JSON")
        .to_string();
    let name = val
        .get("name")
        .expect("Failed to parse string <name> from JSON")
        .to_string();
    if name.ne("null") {
        out.push(name)
    };
    if email.ne("null") {
        out.push(email)
    };

    return Ok(out);
}

fn get_git_urls(rb: RequestBuilder) -> Result<Vec<String>, Box<dyn Error>> {
    let mut urls = Vec::new();
    let res = rb
        .send()
        .expect("request to /{user}/repos failed")
        .text()
        .expect("Conversion to String failed for respositories");
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
            get_repo_page(&token, &username, page).expect("Something went wrong building the URL");
        let new_urls = get_git_urls(req).expect("msg");
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

    let identifiers =
        get_user_identifiers(&token, &username).expect("function get_user_email failed");
    identifiers.iter().for_each(|id| println!("{}", id));

    //TODO: create list of name identifiers for user. this can be their name, email, or any alternative names provided by the user at runtime

    // TODO: This is the git log command that I can use to parse commit data - LOC's added, LOC's removed, date (UTC epoch)
    // git log --pretty=format:"%H%x09%ad%x09" --author="Kaushik Mahadevan" --no-merges --date=unix --numstat
    // TODO: Next step is to chunk each of these git hub repos and run ~5 processes at a time
    // let _blah = Command::new("ls")
    //     .arg("-hl")
    //     .spawn()
    //     .expect("This should just work);

    // these processes will clone the repo, read the condensed log, and create a struct for each of the commits made by the user as defined by their name/identifier

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
