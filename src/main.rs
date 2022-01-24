use chrono::{Duration, Local};
use clap::{App, Arg, ArgMatches};
use regex::Regex;
use reqwest::header::{self, HeaderMap};
use serde::Deserialize;

const SET_URI: &str = "https://slack.com/api/users.profile.set";
const GET_URI: &str = "https://slack.com/api/users.profile.get";

#[derive(Deserialize)]
pub struct SlackProfile {
    pub email: String,
    pub display_name: String,
    pub status_emoji: String,
    pub status_expiration: i32,
}

#[derive(Deserialize)]
struct SlackResponse {
    profile: Option<SlackProfile>,
}

#[tokio::main]
async fn main() {
    run(app_info()).await;
}

fn app_info() -> ArgMatches {
    App::new("slack-vacation")
        .version("1.0.0")
        .author("@okaponta")
        .about("reveals you're in vacation in slack!!")
        .arg(
            Arg::new("token")
                .help("Sets your slack app token")
                .value_name("TOKEN")
                .short('t')
                .long("token")
                .required(true),
        )
        .arg(
            Arg::new("back")
                .help("you're back from vacation")
                .short('b')
                .long("back")
                .required(false),
        )
        .arg(
            Arg::new("date")
                .help("enter date you're in vacation")
                .value_name("DATE")
                .short('d')
                .long("date")
                .required(false),
        )
        .get_matches()
}

async fn run(matches: ArgMatches) {
    if let Some(token) = matches.value_of("token") {
        if matches.is_present("back") {
            back_from_vacation(token).await;
        } else {
            match matches.value_of("date") {
                Some(date) => go_to_vacation(token, date).await,
                None => go_to_vacation(token, &tomorrow()).await,
            }
        }
    } else {
        println!("No token specified");
        return;
    }
}

fn tomorrow() -> String {
    let dt = Local::now();
    let tom = dt + Duration::days(1);
    tom.format("%m/%d").to_string()
}

async fn go_to_vacation(token: &str, date: &str) {
    println!("I'm going to vacation: {} {}", token, date);

    let next = add_vacation(&get_username(token).await, date);
    set_slack_username(token, &next).await;
}

async fn back_from_vacation(token: &str) {
    println!("I'm back from vacation: {}", token);

    let next = remove_vacation(&get_username(token).await);
    set_slack_username(token, &next).await;
}

fn add_vacation(prev: &str, date: &str) -> String {
    prev.to_string() + "(" + date + "休)"
}

fn remove_vacation(prev: &str) -> String {
    Regex::new(r"\(.*休\)")
        .unwrap()
        .replace_all(&prev, "")
        .to_string()
}

async fn get_username(token: &str) -> String {
    let response = reqwest::ClientBuilder::new()
        .default_headers(get_headers(token))
        .build()
        .unwrap()
        .get(GET_URI)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    let response_struct: SlackResponse = serde_json::from_str(&response).unwrap();
    response_struct.profile.unwrap().display_name
}

async fn set_slack_username(token: &str, next: &str) {
    reqwest::ClientBuilder::new()
        .default_headers(post_headers(token))
        .build()
        .unwrap()
        .post(SET_URI)
        .query(&[("name", "display_name"), ("value", &next)])
        .send()
        .await
        .unwrap();
}

fn get_headers(token: &str) -> HeaderMap {
    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_str("application/x-www-form-urlencoded").unwrap(),
    );
    headers.insert(
        header::AUTHORIZATION,
        header::HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
    );
    headers
}

fn post_headers(token: &str) -> HeaderMap {
    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_str("application/json; charset=utf-8").unwrap(),
    );
    headers.insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_str("application/x-www-form-urlencoded").unwrap(),
    );
    headers.insert(
        header::AUTHORIZATION,
        header::HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
    );
    headers
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_vacation() {
        let before = "hoge";
        let date = "01/23";
        let after = add_vacation(before, date);

        assert_eq!(after, "hoge(01/23休)");
    }

    #[test]
    fn test_remove_vacation() {
        let before = "hoge(01/23休)";
        let after = remove_vacation(before);

        assert_eq!(after, "hoge");
    }
}
