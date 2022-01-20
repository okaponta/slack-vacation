use clap::{App, Arg};
use reqwest::header;

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    let matches = App::new("slack-vacation")
        .version("1.0.0")
        .author("okaponta")
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
            back_from_vacation(token);
        } else {
            match matches.value_of("date") {
                Some(date) => go_to_vacation(token, date).await,
                None => go_to_vacation(token, today()).await,
            }
        }
    } else {
        println!("No token specified");
        return;
    }
}

fn today() -> &'static str {
    "2020"
}

const SET_URI: &str = "https://slack.com/api/users.profile.set";

async fn go_to_vacation(token: &str, date: &str) {
    println!("I'm going to vacation: {} {}", token, date);
    // let client = slack_api::default_client().unwrap();
    // let mut request = SetRequest::default();
    // request.name = Some("display_name");
    // request.value = Some("oka");
    // let res = slack_api::users_profile::set(&client, &token, &request).await;
    // println!("{:?}", res);

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

    //let q = "[("foo", "a"), ("foo", "b")]";

    let response = reqwest::ClientBuilder::new()
        .default_headers(headers)
        .build()
        .unwrap()
        .post(SET_URI)
        .query(&[("name", "display_name"), ("value", "okaponta")])
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    println!("{}", response);
}

fn back_from_vacation(token: &str) {
    println!("I'm back from vacation: {}", token);
}
