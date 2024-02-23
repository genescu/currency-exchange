extern crate reqwest;

mod exchange_rate;
mod converter;

use std::env;
use exchange_rate::ApiExchangeRateProvider;
use converter::CurrencyConverter;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 5 {
        println!("Usage: exchange <amount> <from_currency> to <to_currency>");
        return;
    }

    let amount: f64 = match args[1].parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid amount");
            return;
        }
    };

    let from_currency = &args[2].to_uppercase();
    let to_currency = &args[4].to_uppercase();

    let api_url = "https://api.exchangerate-api.com/v4";
    let exchange_rate_provider = ApiExchangeRateProvider::new(api_url);
    let converter = CurrencyConverter::new(exchange_rate_provider);

    // Example: Convert 100 USD to EUR
    match converter.convert(amount, from_currency, to_currency).await {
        Ok(result) => println!("Converted amount: {:.2} {}", result, to_currency),
        Err(err) => eprintln!("Error: {}", err),
    }
}
