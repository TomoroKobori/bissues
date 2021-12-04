use clap::{App, Arg};
use serde::Deserialize;
use reqwest::{Error};

#[derive(Deserialize, Debug)]
struct User {
    login: String,
    id: u32
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let matches = App::new("bissues")
        .version("1.0.0")
        .author("tomoro")
        .about("Deal github issues for milestone")
        .arg(Arg::new("action")
            .value_name("ACTION")
            .about("select actions for [ls, vel]")
            .index(1)
            .required(true))
        .arg(Arg::new("milestone")
            .value_name("MILESTONE")
            .about("select github milestones")
            .index(2)
            .required(true))
        .get_matches();
    if let Some(milestone) = matches.value_of("milestone") {
        if let Some(action) = matches.value_of("action") {
            if action == "ls" {
                ls_issues(String::from(milestone)).await?;
            }
            if action == "vel" {
                let velocity = fetch_velocity(String::from(milestone));
                println!("velocity resurt: {}", velocity)
            }
        }
    }
    Ok(())
}

async fn ls_issues(milestone: String) -> Result<(), Error> {
    let request_url = format!("https://api.github.com/repos/{owner}/{repo}/stargazers",
                              owner = "rust-lang-nursery",
                              repo = "rust-cookbook");
    println!("{}", request_url);
    let client = reqwest::Client::new();
    let response = client.get(&request_url)
        .header("User-Agent", "request")
        .send()
        .await?;
    let users: Vec<User> = response.json().await?;
    println!("{:?}", users);
    println!("{}", milestone);
    Ok(())
}

fn fetch_velocity(velocity: String) -> String {
    String::from(velocity)
}