use serde::{Deserialize, Serialize};

use crate::entities::siren::Siren;

#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct Enterprise {
    pub name: String,
    pub siren: Siren,
    pub email: String,
    pub address: String,
    pub city: String,
    pub postal: String,
    pub phone: String,
    pub title: String,
    pub tva: String,
}

#[derive(Serialize, Deserialize)]
#[cfg_attr(test, derive(Debug, PartialEq))]
pub struct Settings {
    pub enterprise: Enterprise,
    pub law_rules: String,
    pub politeness: String,
}

#[cfg(test)]
mod tests {
    use serde_test::{assert_tokens, Token};

    use super::*;
    impl Enterprise {
        pub fn generate_simple_enterprise() -> Enterprise {
            Enterprise {
                name: "Example Enterprise".into(),
                siren: Siren::new("123456789").unwrap(),
                email: "contact@example.com".into(),
                address: "123 Example Street".into(),
                city: "Example City".into(),
                postal: "12345".into(),
                phone: "123-456-7890".into(),
                title: "CEO".into(),
                tva: "".into(),
            }
        }
    }

    impl Settings {
        pub fn generate_simple_settings() -> Settings {
            Settings {
                enterprise: Enterprise::generate_simple_enterprise(),
                law_rules: "Example Law".into(),
                politeness: "Kind Regards".into(),
            }
        }
    }

    #[test]
    fn test_enterprise_serialization() {
        let enterprise = Enterprise::generate_simple_enterprise();

        assert_tokens(
            &enterprise,
            &[
                Token::Struct { name: "Enterprise", len: 9 },
                Token::Str("name"),
                Token::Str("Example Enterprise"),
                Token::Str("siren"),
                Token::NewtypeStruct { name: "Siren" },
                Token::Str("123456789"),
                Token::Str("email"),
                Token::Str("contact@example.com"),
                Token::Str("address"),
                Token::Str("123 Example Street"),
                Token::Str("city"),
                Token::Str("Example City"),
                Token::Str("postal"),
                Token::Str("12345"),
                Token::Str("phone"),
                Token::Str("123-456-7890"),
                Token::Str("title"),
                Token::Str("CEO"),
                Token::Str("tva"),
                Token::Str(""),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn test_settings_serialization() {
        let settings = Settings::generate_simple_settings();

        assert_tokens(
            &settings,
            &[
                Token::Struct { name: "Settings", len: 3 },
                Token::Str("enterprise"),
                Token::Struct { name: "Enterprise", len: 9 },
                Token::Str("name"),
                Token::Str("Example Enterprise"),
                Token::Str("siren"),
                Token::NewtypeStruct { name: "Siren" },
                Token::Str("123456789"),
                Token::Str("email"),
                Token::Str("contact@example.com"),
                Token::Str("address"),
                Token::Str("123 Example Street"),
                Token::Str("city"),
                Token::Str("Example City"),
                Token::Str("postal"),
                Token::Str("12345"),
                Token::Str("phone"),
                Token::Str("123-456-7890"),
                Token::Str("title"),
                Token::Str("CEO"),
                Token::Str("tva"),
                Token::Str(""),
                Token::StructEnd,
                Token::Str("law_rules"),
                Token::Str("Example Law"),
                Token::Str("politeness"),
                Token::Str("Kind Regards"),
                Token::StructEnd,
            ],
        );
    }

    #[test]
    fn invoice_to_string() {
        let settings_example = Settings::generate_simple_settings();

        let yaml = serde_yaml::to_string(&settings_example).unwrap();

        assert_eq!(
            yaml,
            "enterprise:\n".to_owned()
                + "  name: Example Enterprise\n"
                + "  siren: '123456789'\n"
                + "  email: contact@example.com\n"
                + "  address: 123 Example Street\n"
                + "  city: Example City\n"
                + "  postal: '12345'\n"
                + "  phone: 123-456-7890\n"
                + "  title: CEO\n"
                + "  tva: ''\n"
                + "law_rules: Example Law\n"
                + "politeness: Kind Regards\n"
        );
    }

    #[test]
    fn invoice_from_string() {
        let settings_base_example = Settings::generate_simple_settings();

        let yaml_settings_example = "enterprise:\n".to_owned()
            + "  name: Example Enterprise\n"
            + "  siren: '123456789'\n"
            + "  email: contact@example.com\n"
            + "  address: 123 Example Street\n"
            + "  city: Example City\n"
            + "  postal: '12345'\n"
            + "  phone: 123-456-7890\n"
            + "  title: CEO\n"
            + "  tva: ''\n"
            + "law_rules: Example Law\n"
            + "politeness: Kind Regards\n";

        let settings_example: Settings =
            serde_yaml::from_str(&yaml_settings_example).unwrap();

        assert_eq!(
            settings_base_example.enterprise.name,
            settings_example.enterprise.name
        );
        assert_eq!(settings_base_example.law_rules, settings_example.law_rules);
        assert_eq!(
            settings_base_example.politeness,
            settings_example.politeness
        );
        assert_eq!(
            settings_base_example.enterprise.postal,
            settings_example.enterprise.postal
        );
        assert_eq!(
            settings_base_example.enterprise.phone,
            settings_example.enterprise.phone
        );
        assert_eq!(
            settings_base_example.enterprise.tva,
            settings_example.enterprise.tva
        );
    }
}
