use super::settings_file_manager_error::SettingsFileManagerError;
use crate::entities::settings::Settings;
use std::fs;
use std::path::Path;

pub fn save_settings(
    settings_file_path: &Path,
    settings: &Settings,
) -> Result<(), SettingsFileManagerError> {
    match fs::write(
        settings_file_path,
        serde_yaml::to_string(&settings).unwrap(),
    ) {
        Ok(()) => Ok(()),
        Err(error) => Err(SettingsFileManagerError::UnableToWriteFile(
            settings_file_path.to_string_lossy().to_string(),
            error,
        )),
    }
}

#[cfg(test)]
mod tests {
    use crate::entities::settings::Settings;
    use crate::file_manager::settings::get_settings::get_settings;
    use crate::file_manager::settings::save_settings::save_settings;

    #[test]
    pub fn test_save_config() {
        let temp_dir_assert_fs = assert_fs::TempDir::new().unwrap();
        let temp_settings_file_path = temp_dir_assert_fs.path().join("enterprise.yaml");

        let settings_base_example = Settings::generate_simple_settings();

        save_settings(&temp_settings_file_path, &settings_base_example).unwrap();

        let settings_readed =
            get_settings(&temp_settings_file_path).expect("Unable read customer created");

        assert_eq!(
            settings_base_example.enterprise.name,
            settings_readed.enterprise.name
        );
        assert_eq!(settings_base_example.law_rules, settings_readed.law_rules);
        assert_eq!(settings_base_example.politeness, settings_readed.politeness);
        assert_eq!(
            settings_base_example.enterprise.postal,
            settings_readed.enterprise.postal
        );
        assert_eq!(
            settings_base_example.enterprise.phone,
            settings_readed.enterprise.phone
        );

        temp_dir_assert_fs.close().unwrap();
    }
}
