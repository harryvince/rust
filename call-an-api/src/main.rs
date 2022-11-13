use serde::Deserialize;
use reqwest::Error;

#[derive(Deserialize, Debug)]
struct Quote {
    quote: String,
    person: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let request_url: String = "https://motivational-quote-api.herokuapp.com/quotes/random".to_string();
    println!("Fetching -> {}", request_url);
    let response = reqwest::get(&request_url).await?;

    let quote: Quote = response.json().await?;
    println!("Quote: {}", quote.quote);
    println!("Author: {}", quote.person);
    Ok(())
}
