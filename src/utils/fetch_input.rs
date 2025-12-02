use std::fs;

use reqwest::{header::{self, HeaderMap, HeaderValue}};

pub async fn get_input_data(day: i8) -> Option<String> {
    //let from_file = false;
    let from_file = true;
    if (from_file) {
        get_input_data_from_file(day)
    } else {
        get_input_data_from_api(day).await
    }
}

pub fn get_input_data_from_file(day: i8) -> Option<String> {
    let file_path = "./test.txt";
    let content = fs::read_to_string(file_path);
    if let Err(e) = content {
        panic!("Error occured: {e}")
    }
    fs::read_to_string(file_path).ok()
}

pub async fn get_input_data_from_api(day: i8) -> Option<String> {
    let current_cookie = "session=53616c7465645f5f69451cdf5cbb3ea9d9a86e7468aee5f4bcfae0bf2dea81219a0cd4d54f0b7bd7742d72b2284d1823258f10da11a81a8708a1f35710a56658";
    let mut headers = HeaderMap::new();
    headers.insert(header::COOKIE, HeaderValue::from_str(current_cookie).unwrap());
    let url = format!("https://adventofcode.com/2025/day/{day}/input");
    let response = reqwest::Client::new()
        .get(url)
        .headers(headers)
        .send()
        .await.ok()?
        .text()
        .await.ok()?;
    Some(response)
}