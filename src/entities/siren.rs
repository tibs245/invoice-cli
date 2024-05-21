use std::fmt::Formatter;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
pub struct Siren(String);

impl Siren {
    pub fn new(siren: &str) -> Result<Self, String> {
        if siren.len() == 9 && siren.chars().all(char::is_numeric) {
            return Ok(Siren(siren.to_string()));
        }
        Err("Invalid siren (Only 9 digits characters)".to_string())
    }
}

impl From<u32> for Siren {
    fn from(siren: u32) -> Siren {
        Siren::new(&siren.to_string()).unwrap()
    }
}

impl std::fmt::Display for Siren {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use crate::entities::siren::Siren;

    #[test]
    fn test_new_siren() {
        assert_eq!(
            Siren::new(""),
            Err("Invalid siren (Only 9 digits characters)".to_string())
        );
        assert_eq!(
            Siren::new("123"),
            Err("Invalid siren (Only 9 digits characters)".to_string())
        );
        assert_eq!(
            Siren::new("12336478383766"),
            Err("Invalid siren (Only 9 digits characters)".to_string())
        );
        assert_eq!(
            Siren::new("abcdefhij"),
            Err("Invalid siren (Only 9 digits characters)".to_string())
        );
        assert_eq!(Siren::new("123456789").unwrap().0, "123456789");
    }
}
