use super::invoice_file_manager_error::InvoiceFileManagerError;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};

pub fn is_hidden_file(file_name: &OsStr) -> bool {
    file_name.to_string_lossy().chars().next().unwrap() == '.'
}

pub fn get_all_invoices_path(
    path: &Path,
) -> Result<Vec<PathBuf>, InvoiceFileManagerError> {
    match fs::read_dir(path) {
        Ok(dir_content) => {
            let mut paths: Vec<_> = dir_content
                .map(|file| -> PathBuf { file.unwrap().path() })
                .filter(|file| {
                    file.is_file() && !is_hidden_file(file.file_name().unwrap())
                })
                .collect();

            paths.sort_by_key(|filepath| filepath.to_owned());

            Ok(paths)
        }
        Err(e) => Err(InvoiceFileManagerError::UnableToReadPath(
            path.to_string_lossy().to_string(),
            e,
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_get_all_invoices_path() {
        let temp_dir_assert_fs = assert_fs::TempDir::new().unwrap();
        let temp_dir = temp_dir_assert_fs.path();

        [
            "2020010101.yaml".to_string(),
            "2020020101.yaml".to_string(),
            ".gitignore".to_string(),
            "2020030101.yaml".to_string(),
            "2021031001.yaml".to_string(),
        ]
        .iter()
        .for_each(|filename| {
            fs::write(temp_dir.to_owned().join(filename), "Just a unit test")
                .expect("Unable to write in temporary directory to test");
        });

        let all_invoices_path_result = get_all_invoices_path(temp_dir);

        assert!(all_invoices_path_result.is_ok());

        let all_invoices_path = all_invoices_path_result.unwrap();
        assert_eq!(all_invoices_path.len(), 4);

        println!("Now {:?} will print!", all_invoices_path);

        assert_eq!(
            all_invoices_path[0],
            temp_dir.to_owned().join("2020010101.yaml").as_path()
        );
        assert_eq!(
            all_invoices_path[1],
            temp_dir.to_owned().join("2020020101.yaml").as_path()
        );
        assert_eq!(
            all_invoices_path[2],
            temp_dir.to_owned().join("2020030101.yaml").as_path()
        );
        assert_eq!(
            all_invoices_path[3],
            temp_dir.to_owned().join("2021031001.yaml").as_path()
        );

        temp_dir_assert_fs.close().unwrap();
    }
}
