use std::path::Path;

#[derive(Clone)]
pub struct ContextParameters<'a> {
    pub(crate) invoice_manager_path: &'a Path,
    pub(crate) invoice_path: Option<&'a Path>,
    pub(crate) customer_file_path: Option<&'a Path>,
    pub(crate) config_file_path: Option<&'a Path>,
    pub(crate) build_path: Option<&'a Path>,
    pub(crate) target_path: Option<&'a Path>,
}

impl<'a> From<&'a Path> for ContextParameters<'a> {
    fn from(invoice_manager_path: &'a Path) -> Self {
        Self {
            invoice_manager_path,
            invoice_path: None,
            customer_file_path: None,
            config_file_path: None,
            build_path: None,
            target_path: None,
        }
    }
}