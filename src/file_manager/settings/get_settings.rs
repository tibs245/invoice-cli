use std::fs;
use std::path::Path;

use crate::entities::settings::Settings;
use crate::file_manager::settings::settings_file_manager_error::SettingsFileManagerError;

pub fn get_settings(
    settings_file_path: &Path,
) -> Result<Settings, SettingsFileManagerError> {
    match fs::read_to_string(settings_file_path) {
        Ok(invoice_data) => Ok(serde_yaml::from_str(&invoice_data).unwrap()),
        Err(e) => Err(SettingsFileManagerError::UnableToReadPath(
            settings_file_path.to_string_lossy().to_string(),
            e,
        )),
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use assert_fs::prelude::PathChild;

    use crate::file_manager::settings::get_settings::get_settings;

    #[test]
    fn customer_map_from_string() {
        let settings_content = "enterprise:\n".to_owned()
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

        let temp_dir = assert_fs::TempDir::new().unwrap();
        let temp_settings_file = temp_dir.child("settings.yaml");

        fs::write(temp_settings_file.path(), settings_content).unwrap();

        let settings = get_settings(temp_settings_file.path())
            .expect("Unable to read settings file example");

        assert_eq!(settings.enterprise.title, "CEO");
        assert_eq!(settings.enterprise.postal, "12345");
        assert_eq!(settings.law_rules, "Example Law");
        assert_eq!(settings.politeness, "Kind Regards");

        temp_dir.close().unwrap();
    }
}
