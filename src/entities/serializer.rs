pub(crate) fn serializer(input: &str) -> String {
    let allowed_chars: [char; 38] = [
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P',
        'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '_', '-', '0', '1', '2', '3',
        '4', '5', '6', '7', '8', '9',
    ];

    input
        .chars()
        .filter_map(|c| {
            if c == ' ' {
                Some('_')
            } else if allowed_chars.contains(&c.to_ascii_uppercase()) {
                Some(c.to_ascii_lowercase())
            } else {
                None
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_serializer() {
        assert_eq!(serializer("Exemple de Nom 123"), "exemple_de_nom_123");
        assert_eq!(serializer("Avec-Symboles!@#"), "avec-symboles");
        assert_eq!(serializer("1234567890"), "1234567890");
        assert_eq!(serializer("NormalName"), "normalname");
        assert_eq!(serializer(""), "");
        assert_eq!(serializer("Avec des espaces"), "avec_des_espaces");
        assert_eq!(serializer("_Déjà_Valide_"), "_dj_valide_");
    }
}
