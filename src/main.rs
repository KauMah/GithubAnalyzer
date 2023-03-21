use core::time;

use std::{thread, io};

use reqwest::{Request, Method, Url};

fn main() {
    println!("Github Analyzer - Initializing");
    let sleep_time = time::Duration::from_millis(2000);
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    // clear terminal
    println!("{}[2J", 27 as char); // Figure out how this works fool
    thread::sleep(sleep_time);
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // Not sure how this works look into me
    // end clear terminal


    let api_url = format!("https://api.github.com/users/{username}/repos", username = &buffer.trim_end());
    println!("{}", api_url);

    // Now lets jump into some API hijinx

    let req = Request::new(Method::GET, Url::parse(&api_url).unwrap());

    println!("{:?}", req);



}
