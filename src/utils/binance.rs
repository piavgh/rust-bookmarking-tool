extern crate percent_encoding;

use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');

pub fn construct_binance_url(query: &str) -> String {
    if query == "bn nien" {
        let binance_url = "https://binance.com/en/futures-activity/leaderboard/user?uid=C6435963A5B142B39E651DC621C5E91F";

        binance_url.to_string()
    } else {
        let binance_dotcom = "https://binance.com";

        binance_dotcom.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_construct_binance_url_with_bn() {
        let fake_query = "bn";

        assert_eq!(construct_binance_url(fake_query), "https://binance.com");
    }

    #[test]
    fn test_construct_binance_url_with_bn_nien() {
        let fake_query = "bn nien";

        assert_eq!(construct_binance_url(fake_query), "https://binance.com/en/futures-activity/leaderboard/user?uid=C6435963A5B142B39E651DC621C5E91F");
    }

    #[test]
    fn test_construct_binance_url_with_bn_and_random_text() {
        let fake_query = "bn random_text";

        assert_eq!(construct_binance_url(fake_query), "https://binance.com");
    }
}
