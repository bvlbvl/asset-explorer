use chrono::{Datelike, Duration, NaiveDate};
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct Instrument {
    pub isin: String,
    pub symbol: String,
    pub name: String,
    pub exchange: String,
    pub currency: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct DailyClose {
    pub date: NaiveDate,
    pub close: f64,
}

#[derive(Clone, Debug, Serialize)]
pub struct MonthlyHistory {
    pub isin: String,
    pub symbol: String,
    pub name: String,
    pub exchange: String,
    pub currency: Option<String>,
    pub from: NaiveDate,
    pub to: NaiveDate,
    pub points: Vec<DailyClose>,
}

pub fn normalize_isin(value: &str) -> String {
    value.trim().to_ascii_uppercase()
}

pub fn is_valid_isin(value: &str) -> bool {
    let isin = normalize_isin(value);
    let bytes = isin.as_bytes();
    if bytes.len() != 12
        || !bytes[..2].iter().all(u8::is_ascii_alphabetic)
        || !bytes[2..11].iter().all(u8::is_ascii_alphanumeric)
        || !bytes[11].is_ascii_digit()
    {
        return false;
    }

    let expanded: String = isin
        .chars()
        .flat_map(|character| match character.to_digit(10) {
            Some(digit) => char::from_digit(digit, 10).into_iter().collect::<Vec<_>>(),
            None => (character as u32 - 55).to_string().chars().collect(),
        })
        .collect();

    let mut double_digit = false;
    let sum: u32 = expanded
        .chars()
        .rev()
        .filter_map(|character| character.to_digit(10))
        .map(|digit| {
            let value = if double_digit { digit * 2 } else { digit };
            double_digit = !double_digit;
            value / 10 + value % 10
        })
        .sum();

    sum % 10 == 0
}

pub fn resolve_known_instrument(isin: &str) -> Option<Instrument> {
    let isin = normalize_isin(isin);
    let (symbol, name, exchange, currency) = match isin.as_str() {
        "IE00B0M62Q58" => (
            "IQQW.XETRA",
            "iShares MSCI World UCITS ETF (Dist)",
            "Xetra",
            Some("EUR"),
        ),
        "IE00BKM4GZ66" => (
            "EIMI.XSWX",
            "iShares Core MSCI Emerging Markets IMI UCITS ETF",
            "SIX Swiss Exchange",
            None,
        ),
        _ => return None,
    };

    Some(Instrument {
        isin,
        symbol: symbol.to_owned(),
        name: name.to_owned(),
        exchange: exchange.to_owned(),
        currency: currency.map(str::to_owned),
    })
}

pub fn previous_calendar_month(today: NaiveDate) -> (NaiveDate, NaiveDate) {
    let first_of_this_month = NaiveDate::from_ymd_opt(today.year(), today.month(), 1)
        .expect("The first day of a calendar month is valid.");
    let last_of_previous_month = first_of_this_month - Duration::days(1);
    let first_of_previous_month = NaiveDate::from_ymd_opt(
        last_of_previous_month.year(),
        last_of_previous_month.month(),
        1,
    )
    .expect("The first day of a calendar month is valid.");

    (first_of_previous_month, last_of_previous_month)
}

#[cfg(test)]
mod tests {
    use super::{is_valid_isin, normalize_isin, previous_calendar_month};
    use chrono::NaiveDate;

    #[test]
    fn normalizes_and_validates_known_isin() {
        assert_eq!(normalize_isin("  ie00b0m62q58 "), "IE00B0M62Q58");
        assert!(is_valid_isin("IE00B0M62Q58"));
        assert!(!is_valid_isin("IE00B0M62Q59"));
    }

    #[test]
    fn finds_previous_calendar_month_across_year_boundary() {
        let today = NaiveDate::from_ymd_opt(2026, 1, 3).unwrap();
        let (from, to) = previous_calendar_month(today);

        assert_eq!(from, NaiveDate::from_ymd_opt(2025, 12, 1).unwrap());
        assert_eq!(to, NaiveDate::from_ymd_opt(2025, 12, 31).unwrap());
    }
}
