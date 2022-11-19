use core::panic;
use std::{collections::HashMap, io, env};
use reqwest::Error;
use std::process;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let mut ip_to_locate = String::new();
    if args.len() < 2 {
        println!("Enter a value to check:");
        let mut value = String::new();
        io::stdin()
            .read_line(&mut value)
            .expect("Failed to read input");

        let trimmed = value.trim();
        ip_to_locate = trimmed.to_string();
    } else {
        ip_to_locate = args[1].to_string();
    }
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
    println!("{{\n  \"IP Address\": {},\n  \"Company\": {},\n  \"ISP\": {},\n  \"City\": {},\n  \"Country\": {},\n  \"Latitude\": {},\n  \"Longitude\": {}\n}}",
    json_response["ip"], json_response["company"], json_response["isp"],
    json_response["city"], json_response["country_name"],
    json_response["lat"], json_response["lng"]);
    Ok(())
}