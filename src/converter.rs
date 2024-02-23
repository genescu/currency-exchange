use crate::exchange_rate::ExchangeRateProvider;

pub struct CurrencyConverter<T: ExchangeRateProvider> {
    pub rate_provider: T,
}

impl<T: ExchangeRateProvider> CurrencyConverter<T> {
    pub fn new(provider: T) -> Self {
        CurrencyConverter {
            rate_provider: provider,
        }
    }

    pub async fn convert(&self, amount: f64, from: &str, to: &str) -> Result<f64, String> {
        let exchange_rate_result = self.rate_provider.get_exchange_rate(from, to).await;
        let exchange_rate = match exchange_rate_result {
            Ok(rate) => rate,
            Err(err) => return Err(format!("Error getting exchange rate: {}", err)),
        };
        Ok(amount * exchange_rate)
    }

}
