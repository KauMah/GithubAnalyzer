use core::time;
use reqwest::{
    blocking::{Client, RequestBuilder},
    header::{HeaderMap, HeaderValue, ACCEPT, USER_AGENT},
};
use serde::__private::from_utf8_lossy;
use serde_json::Value;
use std::{env, fs, sync::mpsc};
use std::{error::Error, process::Command};
use std::{
    io,
    path::{Path, PathBuf},
};
use tempfile::TempDir;

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
        out.push(name.trim_matches('"').to_owned())
    };
    if email.ne("null") {
        out.push(email.trim_matches('"').to_owned())
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

fn create_working_dir<P: AsRef<Path>>(path: P) -> Result<PathBuf, std::io::Error> {
    let path = path.as_ref();
    if path.exists() {
        if path.is_dir() {
            Err(std::io::Error::new(
                io::ErrorKind::AlreadyExists,
                "Path already exists, aborting",
            ))
        } else {
            Err(std::io::Error::new(
                io::ErrorKind::AlreadyExists,
                "Path already exists as a file",
            ))
        }
    } else {
        fs::create_dir_all(path)?;
        Ok(path.to_owned())
    }
}

fn main() -> Result<(), reqwest::Error> {
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
        git_urls.extend(new_urls);
        page = page + 1;
        println!();
    }

    let identifiers =
        get_user_identifiers(&token, &username).expect("function get_user_email failed");
    identifiers.iter().for_each(|id| println!("{}", id));
    if identifiers.len() < 1 {
        panic!("No identifiers found for user for parsing repos, consider using option --searchName [<names>...]");
        // TODO: add the ability to use --searchName when running. This is such a later problem
    }

    let dir_path = TempDir::new().expect("Failed to create a temporary directory");
    let loc_dir = env::current_dir().expect("Failed to get current directory");
    let path = dir_path.path().join("ghAnalyze");

    let identifier_str = format!("--author={}", identifiers.join("|"));
    let output = Command::new("git")
        .arg("--no-pager")
        .arg("log")
        .arg("--pretty=format:\"%H%x09%ad%x09\"")
        .arg("--perl-regexp")
        .arg("--invert-grep")
        .arg(identifier_str)
        .arg("--no-merges")
        .arg("--date=unix")
        .arg("--numstat")
        .output()
        .expect("Failed to show git log");

    let output = String::from_utf8_lossy(&output.stdout);
    // println!("{:?}", output);
    let lines = output.lines();
    let mut num_files = 0;
    let mut num_lines_added = 0;
    let mut num_lines_deleted = 0;
    let mut timestamp: &str;
    let mut hash: &str;
    for line in lines {
        let words = line.split_whitespace().collect::<Vec<_>>();
        let words = words
            .into_iter()
            .map(|st| st.trim_matches('"'))
            .collect::<Vec<_>>();
        let first = words.get(0);
        match first {
            Some(term) => {
                if term.len() == 40 {
                    hash = term;
                    timestamp = words.get(1).unwrap();
                    println!("Hash: {}, ts: {}", hash, timestamp);
                } else {
                    num_files += 1;
                    num_lines_added += term.parse::<i32>().expect("This should be an integer");
                    num_lines_deleted += words
                        .get(1)
                        .unwrap()
                        .parse::<i32>()
                        .expect("This should be an integer");
                }
            }
            None => {
                println!(
                    "Files: {}, added: {}, removed: {}",
                    num_files, num_lines_added, num_lines_deleted
                );
                num_files = 0;
                num_lines_added = 0;
                num_lines_deleted = 0;
            }
        };
    }
    println!(
        "Files: {}, added: {}, removed: {}",
        num_files, num_lines_added, num_lines_deleted
    );

    Ok(())
}
