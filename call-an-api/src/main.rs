use serde::Deserialize;
use reqwest::Error;

#[derive(Deserialize, Debug)]
struct Quote {
    quote: String,
    person: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let request_url = "https://motivational-quote-api.herokuapp.com/quotes/random";
    println!("Fetching -> {}", request_url);
    let response = reqwest::get(request_url).await?;

    let quote: Quote = response.json().await?;
    println!("Quote: {}", quote.quote);
    println!("Author: {}", quote.person);
    Ok(())
}
