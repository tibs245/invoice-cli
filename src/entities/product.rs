use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Product {
    pub description: String,
    pub quantity: f32,
    pub price: f32,
}
impl Product {
    pub fn get_total_price(&self) -> f32 {
        self.quantity * self.price
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn product_to_string() {
        let product_example = Product {
            description: "Product example".to_string(),
            quantity: 1.0,
            price: 350.0,
        };

        let yaml = serde_yaml::to_string(&product_example).unwrap();

        assert_eq!(
            yaml,
            "description: Product example\nquantity: 1.0\nprice: 350.0\n"
        );
    }

    #[test]
    fn product_from_string() {
        let yaml_product_example = "description: Product example\n".to_owned()
            + "quantity: 1\n"
            + "price: 350\n";

        let product_example: Product =
            serde_yaml::from_str(&yaml_product_example).unwrap();

        assert_eq!("Product example".to_string(), product_example.description);

        assert_eq!(350.0, product_example.price);

        assert_eq!(1.0, product_example.quantity);
    }

    #[test]
    fn product_total_price_simple() {
        let product_example = Product {
            description: "Product example".to_string(),
            quantity: 1.0,
            price: 350.0,
        };

        assert_eq!(product_example.get_total_price(), 350.0);
    }

    #[test]
    fn product_total_price_zero() {
        let product_example = Product {
            description: "Product example".to_string(),
            quantity: 0.0,
            price: 350.0,
        };

        assert_eq!(product_example.get_total_price(), 0.0);
    }

    #[test]
    fn product_total_price_multiple() {
        let product_example = Product {
            description: "Product example".to_string(),
            quantity: 14.0,
            price: 100_000.0,
        };

        assert_eq!(product_example.get_total_price(), 1_400_000.0);
    }

    #[test]
    fn product_total_price_half() {
        let product_example = Product {
            description: "Product example".to_string(),
            quantity: 1.5,
            price: 350.0,
        };

        assert_eq!(product_example.get_total_price(), 525.0);
    }
}
