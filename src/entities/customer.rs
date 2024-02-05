use crate::entities::serializer::serializer;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize)]
pub struct Customer {
    pub name: String,
    pub address: String,
    pub city: String,
    pub postal: String,
}

impl Customer {
    pub fn serialized_name(&self) -> String {
        serializer(&self.name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    impl Customer {
        pub fn simple_customer() -> Customer {
            return Customer {
                name: "King SARL".into(),
                address: "1 rue des champs".into(),
                city: "Paris".into(),
                postal: "75000".into(),
            };
        }
        pub fn simple_with_name(name: String) -> Customer {
            return Customer {
                name,
                address: "1 rue des champs".into(),
                city: "Paris".into(),
                postal: "75000".into(),
            };
        }
    }

    #[test]
    fn customer_to_string() {
        let customer_example = Customer::simple_customer();

        let customer_yaml = serde_yaml::to_string(&customer_example).unwrap();

        assert_eq!(
            "name: King SARL\n".to_owned()
                + "address: 1 rue des champs\n"
                + "city: Paris\n"
                + "postal: '75000'\n",
            customer_yaml
        );
    }

    #[test]
    fn customer_from_string() {
        let base_customer_example = Customer::simple_customer();
        let yaml_customer_example = "name: King SARL\n".to_owned()
            + "address: 1 rue des champs\n"
            + "city: Paris\n"
            + "postal: 75000\n";

        let customer_example: Customer =
            serde_yaml::from_str(&yaml_customer_example).unwrap();

        assert_eq!(base_customer_example.name, customer_example.name);
        assert_eq!(base_customer_example.address, customer_example.address);
        assert_eq!(base_customer_example.city, customer_example.city);
        assert_eq!(base_customer_example.postal, customer_example.postal);
    }

    #[test]
    fn customer_map_to_string() {
        let mut customer_dto_map = HashMap::new();

        customer_dto_map.insert("king", Customer::simple_customer());

        let customer_yaml = serde_yaml::to_string(&customer_dto_map).unwrap();

        assert_eq!(
            "king:\n".to_owned()
                + "  name: King SARL\n"
                + "  address: 1 rue des champs\n"
                + "  city: Paris\n"
                + "  postal: '75000'\n",
            customer_yaml
        );
    }

    #[test]
    fn customer_map_from_string() {
        let customer_map_yaml = "king:\n".to_owned()
            + "  name: King SARL\n"
            + "  address: 1 rue des champs\n"
            + "  city: Paris\n"
            + "  postal: '75000'\n";

        let customer_example = Customer::simple_customer();

        let customer_map_example: HashMap<String, Customer> =
            serde_yaml::from_str(&customer_map_yaml).unwrap();

        assert!(customer_map_example.contains_key("king"));
        assert_eq!(customer_map_example.len(), 1);

        let king_customer_example = customer_map_example.get("king").unwrap();
        assert_eq!(king_customer_example.name, customer_example.name);
        assert_eq!(king_customer_example.address, customer_example.address);
        assert_eq!(king_customer_example.city, customer_example.city);
        assert_eq!(king_customer_example.postal, customer_example.postal);
    }
}
