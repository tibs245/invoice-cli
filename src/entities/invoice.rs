use std::fmt;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::entities::invoice_date::{deser_invoice_date, ser_invoice_date};
use crate::entities::product::Product;

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct InvoiceDayId(String);

impl InvoiceDayId {
    pub fn new(id: &str) -> Result<Self, String> {
        if id.len() <= 2 && id.chars().all(char::is_numeric) {
            let id_num = id.parse::<u8>().map_err(|_| "Invalid number".to_string())?;
            if id_num >= 1 && id_num <= 99 {
                if id.len() == 1 {
                    return Ok(InvoiceDayId("0".to_string() + id));
                } else {
                    return Ok(InvoiceDayId(id.to_string()));
                }
            }
        }
        Err("Invalid invoice id. Need only two digits maximum".to_string())
    }
}

impl From<u32> for InvoiceDayId {
    fn from(day: u32) -> InvoiceDayId {
        InvoiceDayId::new(&day.to_string()).unwrap()
    }
}

impl fmt::Display for InvoiceDayId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Invoice {
    #[serde(
    serialize_with = "ser_invoice_date",
    deserialize_with = "deser_invoice_date"
    )]
    pub date: NaiveDate,
    pub customer_id: String,
    pub title: String,
    pub invoice_day_id: Option<InvoiceDayId>,
    pub products: Vec<Product>,
}

impl Invoice {
    pub fn get_ref(&self) -> Option<String> {
        match &self.invoice_day_id {
            Some(invoice_day_id) => Some(
                self.date.format("%Y%m%d").to_string() + &(invoice_day_id.to_string()),
            ),
            None => None,
        }
    }
    pub fn get_total_price(&self) -> f32 {
        self.products
            .iter()
            .fold(0.0, |total, product| total + product.get_total_price())
    }
}

impl fmt::Display for Invoice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} - {} - {} €", self.get_ref().unwrap(), self.customer_id, self.get_total_price().to_string())
    }
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDate;

    use super::*;

    impl Invoice {
        pub fn generate_simple_invoice_example() -> Invoice {
            let simple_product = Product {
                description: "Product example".to_string(),
                quantity: 1.0,
                price: 350.0,
            };

            Invoice {
                invoice_day_id: Some(InvoiceDayId::new("01").unwrap()),
                date: NaiveDate::from_ymd_opt(2015, 3, 14).unwrap(),
                customer_id: "king".to_string(),
                title: "Test invoice for simple customer".to_string(),
                products: vec![simple_product],
            }
        }

        pub fn generate_simple_invoice_with_id_and_date_example(
            invoice_day_id: Option<InvoiceDayId>,
            date: NaiveDate,
        ) -> Invoice {
            let simple_product = Product {
                description: "Product example".to_string(),
                quantity: 1.0,
                price: 350.0,
            };

            Invoice {
                invoice_day_id,
                date,
                customer_id: "king".to_string(),
                title: "Test invoice for simple customer".to_string(),
                products: vec![simple_product],
            }
        }
    }

    #[test]
    fn invoice_to_string() {
        let invoice_example = Invoice::generate_simple_invoice_example();

        let yaml = serde_yaml::to_string(&invoice_example).unwrap();

        assert_eq!(
            yaml,
            "date:\n".to_owned()
                + "  day: '14'\n"
                + "  month: '03'\n"
                + "  year: '2015'\n"
                + "customer_id: king\n"
                + "title: Test invoice for simple customer\n"
                + "invoice_day_id: '01'\n"
                + "products:\n"
                + "- description: Product example\n"
                + "  quantity: 1.0\n"
                + "  price: 350.0\n"
        );
    }

    #[test]
    fn invoice_from_string() {
        let invoice_base_example = Invoice::generate_simple_invoice_example();

        let yaml_invoice_example = "date:\n".to_owned()
            + "  day: '14'\n"
            + "  month: '03'\n"
            + "  year: '2015'\n"
            + "customer_id: king\n"
            + "title: Test invoice for simple customer\n"
            + "invoice_day_id: '01'\n"
            + "products:\n"
            + "- description: Product example\n"
            + "  quantity: 1.0\n"
            + "  price: 350.0\n";

        let invoice_example: Invoice =
            serde_yaml::from_str(&yaml_invoice_example).unwrap();

        assert_eq!(invoice_base_example.date, invoice_example.date);
        assert_eq!(
            invoice_base_example.customer_id,
            invoice_example.customer_id
        );
        assert_eq!(invoice_base_example.title, invoice_example.title);
        assert_eq!(
            invoice_base_example.invoice_day_id,
            invoice_example.invoice_day_id
        );
        assert_eq!(
            invoice_base_example.products[0].description,
            invoice_example.products[0].description
        );
    }

    #[test]
    fn invoice_id_test() {
        assert_eq!(
            InvoiceDayId::new("1").unwrap().to_string(),
            "01".to_string()
        );
        assert_eq!(
            InvoiceDayId::new("01").unwrap().to_string(),
            "01".to_string()
        );
        assert_eq!(
            InvoiceDayId::new("10").unwrap().to_string(),
            "10".to_string()
        );
        assert_eq!(
            InvoiceDayId::new("99").unwrap().to_string(),
            "99".to_string()
        );
        assert_eq!(
            InvoiceDayId::new("999"),
            Err("Invalid invoice id. Need only two digits maximum".to_string())
        );
    }

    #[test]
    fn invoice_reference_test() {
        let invoice_example = Invoice::generate_simple_invoice_example();

        assert_eq!(invoice_example.get_ref(), Some("2015031401".to_string()));
    }

    #[test]
    fn invoice_total_price() {
        let simple_product = Product {
            description: "Product example".to_string(),
            quantity: 1.0,
            price: 350.0,
        };
        let second_simple_product = Product {
            description: "Product example".to_string(),
            quantity: 7.0,
            price: 75.0,
        };

        let invoice_example = Invoice {
            invoice_day_id: Some(InvoiceDayId::new("01").unwrap()),
            date: NaiveDate::from_ymd_opt(2015, 3, 14).unwrap(),
            customer_id: "king".to_string(),
            title: "Test invoice for simple customer".to_string(),
            products: vec![],
        };

        assert_eq!(invoice_example.get_total_price(), 0.0);

        let invoice_example = Invoice {
            invoice_day_id: Some(InvoiceDayId::new("01").unwrap()),
            date: NaiveDate::from_ymd_opt(2015, 3, 14).unwrap(),
            customer_id: "king".to_string(),
            title: "Test invoice for simple customer".to_string(),
            products: vec![simple_product.clone()],
        };

        assert_eq!(invoice_example.get_total_price(), 350.0);

        let invoice_example = Invoice {
            invoice_day_id: Some(InvoiceDayId::new("01").unwrap()),
            date: NaiveDate::from_ymd_opt(2015, 3, 14).unwrap(),
            customer_id: "king".to_string(),
            title: "Test invoice for simple customer".to_string(),
            products: vec![simple_product.clone(), second_simple_product.clone()],
        };

        assert_eq!(invoice_example.get_total_price(), 875.0);
    }

    #[test]
    fn test_invoice_display() {
        let invoice = Invoice::generate_simple_invoice_example();

        let output = format!("{}", invoice);

        assert_eq!(output, "2015031401 - king - 350");
    }
}
