use reqwest::blocking::Client;
use reqwest::header::AUTHORIZATION;
use serde_json::json;

fn post_comment(pr_number: u32, comment: &str, token: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let url = format!("https://github.com/Dericko681/fibbot/issues/{}/comments", pr_number);

    let res = client.post(&url)
        .header(AUTHORIZATION, format!("token {}", token))
        .header("User-Agent", "FibonacciBot")
        .json(&json!({ "body": comment }))
        .send()?;

    if res.status().is_success() {
        println!("Comment posted successfully!");
    } else {
        println!("Failed to post comment: {}", res.status());
    }
    Ok(())
}
