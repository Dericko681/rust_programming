use std::env;
use reqwest::blocking::Client;
use reqwest::header::AUTHORIZATION;
use regex::Regex;

// mod fibonacci;

fn main() {
    // Read inputs from environment variables
    let token = env::var("GITHUB_TOKEN").expect("GITHUB_TOKEN not set");
    let pr_number: u32 = env::var("INPUT_PR_NUMBER")
        .expect("INPUT_PR_NUMBER not set")
        .parse()
        .expect("Invalid PR number");

    let pr_body = fetch_pr_body(pr_number, &token).expect("Failed to fetch PR body");
    let numbers = extract_numbers(&pr_body);

    if !numbers.is_empty() {
        let results: Vec<(u32, u32)> = numbers.iter().map(|&n| (n, fibonacci::fibonacci(n))).collect();

        // Print results (this will be the comment you post later)
        for (num, fib) in results {
            println!("Fibonacci of {} is {}", num, fib);
        }
    } else {
        println!("No numbers found in PR body.");
    }
}

fn fetch_pr_body(pr_number: u32, token: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();
    let url = format!("https://github.com/Dericko681/fibbot/pulls/{}", pr_number);

    let response = client.get(&url)
        .header(AUTHORIZATION, format!("token {}", token))
        .header("User-Agent", "FibonacciBot")
        .send()?;

    let json: serde_json::Value = response.json()?;
    let body = json["body"].as_str().unwrap_or("").to_string();

    Ok(body)
}

fn extract_numbers(text: &str) -> Vec<u32> {
    let re = Regex::new(r"\b\d+\b").unwrap();
    re.captures_iter(text)
        .filter_map(|cap| cap[0].parse::<u32>().ok())
        .collect()
}
