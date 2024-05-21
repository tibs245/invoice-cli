use std::fmt;

use chrono::{Datelike, NaiveDate};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct DayString(String);

impl DayString {
    pub fn new(day: &str) -> Result<Self, String> {
        if day.len() <= 2 && day.chars().all(char::is_numeric) {
            let day_num = day.parse::<u8>().map_err(|_| "Invalid number".to_string())?;
            if day_num >= 1 && day_num <= 31 {
                if day.len() == 1 {
                    return Ok(DayString("0".to_string() + day));
                } else {
                    return Ok(DayString(day.to_string()));
                }
            }
        }
        Err("Invalid day".to_string())
    }
}

impl From<u32> for DayString {
    fn from(day: u32) -> DayString {
        DayString::new(&day.to_string()).unwrap()
    }
}

impl fmt::Display for DayString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub(crate) struct MonthString(String);

impl MonthString {
    pub fn new(month: &str) -> Result<Self, String> {
        if month.len() <= 2 && month.chars().all(char::is_numeric) {
            let month_num =
                month.parse::<u8>().map_err(|_| "Invalid number".to_string())?;
            if month_num >= 1 && month_num <= 12 {
                if month.len() == 1 {
                    return Ok(MonthString("0".to_string() + month));
                } else {
                    return Ok(MonthString(month.to_string()));
                }
            }
        }
        Err("Invalid month".to_string())
    }
}

impl From<u32> for MonthString {
    fn from(month: u32) -> MonthString {
        MonthString::new(&month.to_string()).unwrap()
    }
}

impl fmt::Display for MonthString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
struct YearString(String);

impl YearString {
    pub fn new(day: &str) -> Result<Self, String> {
        if day.chars().all(char::is_numeric) {
            return Ok(YearString(day.to_string()));
        }
        Err("Invalid year".to_string())
    }
}

impl From<i32> for YearString {
    fn from(year: i32) -> YearString {
        YearString::new(&year.to_string()).unwrap()
    }
}

impl fmt::Display for YearString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct InvoiceDate {
    day: DayString,
    month: MonthString,
    year: YearString,
}

impl InvoiceDate {
    pub fn new(day: u32, month: u32, year: i32) -> Self {
        InvoiceDate {
            day: day.into(),
            month: month.into(),
            year: year.into(),
        }
    }
}

impl From<&NaiveDate> for InvoiceDate {
    fn from(date: &NaiveDate) -> InvoiceDate {
        InvoiceDate::new(date.day(), date.month(), date.year())
    }
}

impl Into<NaiveDate> for InvoiceDate {
    fn into(self) -> NaiveDate {
        NaiveDate::from_ymd_opt(
            self.year.to_string().parse::<i32>().unwrap(),
            self.month.to_string().parse::<u32>().unwrap(),
            self.day.to_string().parse::<u32>().unwrap(),
        )
            .unwrap()
    }
}

pub(crate) fn ser_invoice_date<S>(
    date: &NaiveDate,
    serializer: S,
) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
{
    InvoiceDate::from(date).serialize(serializer)
}

pub(crate) fn deser_invoice_date<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
    where
        D: serde::Deserializer<'de>,
{
    let date: NaiveDate = InvoiceDate::deserialize(deserializer)?.into();
    Ok(date)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn invoice_date_constructor() {
        let invoice_date = InvoiceDate::new(1, 1, 2020);

        assert_eq!(invoice_date.day, DayString::new("01").unwrap());
        assert_eq!(invoice_date.month, MonthString::new("01").unwrap());
        assert_eq!(invoice_date.year, YearString::new("2020").unwrap());

        let invoice_date = InvoiceDate::new(22, 12, 14);

        assert_eq!(invoice_date.day, DayString::new("22").unwrap());
        assert_eq!(invoice_date.month, MonthString::new("12").unwrap());
        assert_eq!(invoice_date.year, YearString::new("14").unwrap());
    }

    #[test]
    fn invoice_date_into_naive_date() {
        let invoice_date = InvoiceDate {
            day: DayString::new("01").unwrap(),
            month: MonthString::new("1").unwrap(),
            year: YearString::new("1970").unwrap(),
        };

        assert_eq!(
            NaiveDate::from_ymd_opt(1970, 1, 1).unwrap(),
            invoice_date.into()
        );
    }

    #[test]
    fn day_string_parameters() {
        assert_eq!(DayString::new("1").unwrap().to_string(), "01".to_string());
        assert_eq!(DayString::new("01").unwrap().to_string(), "01".to_string());
        assert_eq!(DayString::new("10").unwrap().to_string(), "10".to_string());
        assert_eq!(DayString::new("20").unwrap().to_string(), "20".to_string());
        assert_eq!(DayString::new("31").unwrap().to_string(), "31".to_string());

        assert_eq!(DayString::new("32"), Err("Invalid day".to_string()));
        assert_eq!(DayString::new("0"), Err("Invalid day".to_string()));
        assert_eq!(DayString::new("100"), Err("Invalid day".to_string()));
        assert_eq!(DayString::new("p1"), Err("Invalid day".to_string()));
    }

    #[test]
    fn month_string_parameters() {
        assert_eq!(MonthString::new("1").unwrap().to_string(), "01".to_string());
        assert_eq!(
            MonthString::new("01").unwrap().to_string(),
            "01".to_string()
        );
        assert_eq!(
            MonthString::new("10").unwrap().to_string(),
            "10".to_string()
        );
        assert_eq!(
            MonthString::new("12").unwrap().to_string(),
            "12".to_string()
        );

        assert_eq!(MonthString::new("13"), Err("Invalid month".to_string()));
        assert_eq!(MonthString::new("0"), Err("Invalid month".to_string()));
        assert_eq!(MonthString::new("100"), Err("Invalid month".to_string()));
        assert_eq!(MonthString::new("p1"), Err("Invalid month".to_string()));
    }

    #[test]
    fn year_string_parameters() {
        assert_eq!(YearString::new("0").unwrap().to_string(), "0".to_string());
        assert_eq!(YearString::new("01").unwrap().to_string(), "01".to_string());
        assert_eq!(YearString::new("10").unwrap().to_string(), "10".to_string());
        assert_eq!(
            YearString::new("1000").unwrap().to_string(),
            "1000".to_string()
        );
        assert_eq!(
            YearString::new("2020").unwrap().to_string(),
            "2020".to_string()
        );
        assert_eq!(
            YearString::new("10000").unwrap().to_string(),
            "10000".to_string()
        );

        assert_eq!(YearString::new("-10"), Err("Invalid year".to_string()));
        assert_eq!(
            YearString::new("deuxmilledeux"),
            Err("Invalid year".to_string())
        );
        assert_eq!(YearString::new("2 0 0 2"), Err("Invalid year".to_string()));
    }
}
