use chrono::{Duration, Local};
use clap::{App, Arg};
use regex::Regex;
use reqwest::header;
use serde::Deserialize;

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
    run().await;
}

async fn run() {
    let matches = App::new("slack-vacation")
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
        .get_matches();

    if let Some(token) = matches.value_of("token") {
        println!("token specified: {}", token);
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

const SET_URI: &str = "https://slack.com/api/users.profile.set";
const GET_URI: &str = "https://slack.com/api/users.profile.get";

async fn go_to_vacation(token: &str, date: &str) {
    println!("I'm going to vacation: {} {}", token, date);

    let prev = get_username(token).await;
    let next = prev + "(" + date + "休)";

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

    let response = reqwest::ClientBuilder::new()
        .default_headers(headers)
        .build()
        .unwrap()
        .post(SET_URI)
        .query(&[("name", "display_name"), ("value", &next)])
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    println!("{}", response);
}

async fn back_from_vacation(token: &str) {
    println!("I'm back from vacation: {}", token);
    let prev = get_username(token).await;
    let re = Regex::new(r"\(.*休\)").unwrap();
    let next = re.replace_all(&prev, "").to_string();
    println!("{}", next);
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

    let response = reqwest::ClientBuilder::new()
        .default_headers(headers)
        .build()
        .unwrap()
        .post(SET_URI)
        .query(&[("name", "display_name"), ("value", &next)])
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    println!("{}", response);
}

async fn get_username(token: &str) -> String {
    println!("I'm going to vacation: {}", token);

    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_str("application/x-www-form-urlencoded").unwrap(),
    );
    headers.insert(
        header::AUTHORIZATION,
        header::HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
    );

    let response = reqwest::ClientBuilder::new()
        .default_headers(headers)
        .build()
        .unwrap()
        .get(GET_URI)
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    println!("{}", response);

    let response_struct: SlackResponse = serde_json::from_str(&response).unwrap();
    let disp_name = response_struct.profile.unwrap().display_name;

    disp_name
}
