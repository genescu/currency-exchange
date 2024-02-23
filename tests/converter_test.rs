#[cfg(test)]
mod converter_test {
    use crate::converter; // Import the convert module from src/convert.rs

    #[test]
    fn test_invalid_input() {
        let converter = CurrencyConverter::new();
        let converted_amount = converter.convert(100.0, "INVALID", "EUR");
        assert_eq!(converted_amount, 0.0); // Expected converted amount for invalid input
    }

    #[test]
    fn test_edge_cases() {
        let converter = CurrencyConverter::new();
        let converted_amount = converter.convert(0.0, "USD", "EUR");
        assert_eq!(converted_amount, 0.0); // Expected converted amount for 0 input
    }

    #[test]
    fn test_unsupported_currencies() {
        let converter = CurrencyConverter::new();
        let converted_amount = converter.convert(100.0, "USD", "UNSUPPORTED");
        assert_eq!(converted_amount, 0.0); // Expected converted amount for unsupported currency
    }
}
