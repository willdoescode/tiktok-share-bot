use futures::future::try_join_all;
use std::{future::Future, time::Duration};

use rand::seq::SliceRandom;
use reqwest::header::{self, HeaderMap};
use std::io;

const API_DOMAINS: [&str; 4] = [
    "api19.tiktokv.com",
    "api.toutiao50.com",
    "api19.toutiao50.com",
    "api19-core-c-alisg.tiktokv.com",
];

const USER_AGENT: &str = "com.zhiliaoapp.musically.go/220400 (Linux; U; Android 10; en_US; SM-G9250; Build/MMB25K.G9250ZTU5DPC5; Cronet/TTNetVersion:5f9540e5 2021-05-20 QuicVersion:47555d5a 2020-10-15)";
const CONTENT_TYPE: &str = "application/x-www-form-urlencoded; charset=UTF-8";

fn add_view(
    vid_id: &str,
) -> Result<impl Future<Output = Result<reqwest::Response, reqwest::Error>>, reqwest::Error> {
    let mut headers = HeaderMap::new();
    headers.insert(
        header::USER_AGENT,
        header::HeaderValue::from_static(USER_AGENT),
    );

    headers.insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static(CONTENT_TYPE),
    );

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap();

    let url = format!(
        "https://{}/aweme/v1/aweme/stats/?channel=googleplay&device_type=SM-G9250&device_id{}&os_version=10&version_code=220400&app_name=musically_go&device_platform=android&aid=1988",
         API_DOMAINS.choose(
             &mut rand::thread_rng()
            ).unwrap(),
          rand::random::<u128>()
        );

    let data = format!("item_id={}&share_delta=1", vid_id);

    Ok(client.post(&url).body(data).send())
}

#[tokio::main]
async fn main() {
    println!("Enter video id: ");
    let mut video_id = String::new();
    io::stdin().read_line(&mut video_id).unwrap();
    let video_id = video_id.trim();

    // let vid_id = "7088909151497145642";

    println!("Enter number of shares to add every 5 sec: ");
    let mut num = String::new();
    io::stdin().read_line(&mut num).unwrap();

    let shares_to_add = num.trim().parse::<i32>().unwrap();

    loop {
        let mut share_futures = Vec::new();
        for _ in 0..shares_to_add {
            share_futures.push(add_view(video_id).unwrap());
        }

        match try_join_all(share_futures).await {
            Ok(_) => println!("Added {} shares", shares_to_add),
            Err(e) => println!("error: {}", e),
        }

        std::thread::sleep(Duration::from_secs(5));
    }
}
