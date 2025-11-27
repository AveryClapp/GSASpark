use regex::Regex;
use reqwest::Client;
use std::collections::HashMap;
use std::time::Instant;

#[tokio::main]
async fn main() {
    let start = Instant::now();
    let url = "https://www.gsa-spark.com/speedtest/c5e6c92d-0688-4baa-8c72-5cf7b42ef606";

    let client = Client::new();
    let resp = client.get(url).send().await.unwrap();
    let html = resp.text().await.unwrap();

    println!("HTML length: {}", html.len());

    // Parse number1
    let re1 = Regex::new(r#"id="number1">(\d+)<"#).unwrap();
    let num1: i32 = re1.captures(&html).expect("Failed to find number1")[1]
        .parse()
        .unwrap();
    println!("Number1: {}", num1);

    // Parse number2
    let re2 = Regex::new(r#"id="number2">(\d+)<"#).unwrap();
    let num2: i32 = re2.captures(&html).expect("Failed to find number2")[1]
        .parse()
        .unwrap();
    println!("Number2: {}", num2);

    // Parse CSRF token - fix the pattern
    let re_token = Regex::new(r#"__RequestVerificationToken[^v]*value="([^"]+)""#).unwrap();
    let token = re_token.captures(&html).expect("Failed to find token")[1].to_string();
    println!("Token: {}", &token[..50]);

    // Solve
    let answer = num1 + num2;

    // Submit
    let mut params = HashMap::new();
    params.insert("answer", answer.to_string());
    params.insert("__RequestVerificationToken", token);

    let post_resp = client.post(url).form(&params).send().await.unwrap();

    let result = post_resp.text().await.unwrap();
    println!("Answer: {}", answer);
    println!("Response: {}", result);
    println!("Time: {:?}", start.elapsed());
}
