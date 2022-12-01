use std::{collections::HashMap, io, env};
use reqwest::Error;
use std::process;
use std::io::Write;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let ip_to_locate = check_args(args);
    let request_url = "https://iplocation.com";
    let mut params = HashMap::new();
    params.insert("ip", ip_to_locate);
    let client = reqwest::Client::new();
    let response = client.post(request_url)
        .form(&params)
        .send()
        .await?;
    let ip_data = response.text().await?;
    let json_response: serde_json::Value = serde_json::from_str(&ip_data).unwrap_or_else(|_| {
        println!("Unable to complete lookup!");
        process::exit(1);
    });
    println!("{}", serde_json::to_string_pretty(&json_response).unwrap());
    Ok(())
}

fn check_args(args: Vec<String>) -> String {
    if args.len() < 2 {
        print!("Enter a value to check: ");
        io::stdout().flush().unwrap();
        let mut value = String::new();
        io::stdin()
            .read_line(&mut value)
            .expect("Failed to read input");

        let trimmed = value.trim();
        return trimmed.to_string();
    } else {
        return args[1].to_string();
    }
}