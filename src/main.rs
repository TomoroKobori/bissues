use clap::{App, Arg};
use serde::Deserialize;
use reqwest::{Error};
use std::env;

#[derive(Deserialize, Debug)]
struct Issue {
    number: i32,
    title: String,
    state: String,
    labels: Vec<Label>,
    milestone: Milestone
}

impl Issue {
    fn estimate_label(&self) -> &Label {
        self.labels.iter().find(|label| label.color == Config::new().github_estimate_color_code).unwrap()
    }
}

#[derive(Deserialize, Debug)]
struct Label {
    name: String,
    color: String
}

#[derive(Deserialize, Debug)]
struct Milestone {
    title: String
}

struct Config {
    github_owner: String,
    github_repo: String,
    github_estimate_color_code: String,
    github_token: String
}

impl Config {
    pub fn new() -> Self {
        let token = match env::var("GITHUB_ESTIMATE_COLOR_CODE") {
            Ok(token) => token,
            _ => String::from("hoge")
        };
        Self {
            github_owner: env::var("GITHUB_OWNER").unwrap(),
            github_repo: env::var("GITHUB_REPO").unwrap(),
            github_estimate_color_code: env::var("GITHUB_ESTIMATE_COLOR_CODE").unwrap(),
            github_token: token
        }
    }
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
        .arg(Arg::new("milestone_number")
            .value_name("MILESTONE_NUMBER")
            .about("select github milestone_number")
            .index(2)
            .required(true))
        .get_matches();
    if let Some(milestone_number) = matches.value_of("milestone_number") {
        if let Some(action) = matches.value_of("action") {
            if action == "ls" {
                let issues = fetch_issues(String::from(milestone_number)).await?;
                println!("milestone title: {}", issues[0].milestone.title);
                println!("number,title,estimate,state");
                for issue in issues {
                  let estimate_label = issue.estimate_label();
                  println!("{},{},{},{}", issue.number, issue.title, estimate_label.name, issue.state)
                };
            }
            if action == "vel" {
                let velocity = fetch_velocity(String::from(milestone_number)).await?;
                println!("velocity: {}", velocity);
            }
        }
    }
    Ok(())
}

async fn fetch_issues(milestone_number: String) -> Result<Vec<Issue>, Error> {
    let request_url = format!("https://api.github.com/repos/{owner}/{repo}/issues",
                              owner = Config::new().github_owner,
                              repo = Config::new().github_repo);
    println!("{}", request_url);
    let client = reqwest::Client::new();
    let response = client.get(&request_url)
        .header("User-Agent", "request")
        // privateリポジトリを指定する場合
        // .header("Authorization", format!("token {}", Config::new().github_token))
        .query(&[("state", "all"), ("milestone", &milestone_number)])
        .send()
        .await?;
    let issues: Vec<Issue> = response.json().await?;
    Ok(issues)
}

async fn fetch_velocity(milestone_number: String) -> Result<String, Error> {
    let issues = fetch_issues(String::from(milestone_number)).await?;
    let mut total = 0;
    for issue in issues {
        let estimate_point: i32 = issue.estimate_label().name.parse().unwrap();
        total += estimate_point;
    }
    Ok(total.to_string())
}