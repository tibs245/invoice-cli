use std::path::Path;

pub struct ContextParameters<'a> {
    pub(crate) invoice_manager_path: &'a Path,
    pub(crate) invoice_path: Option<&'a Path>,
    pub(crate) customer_file_path: Option<&'a Path>,
    pub(crate) config_file_path: Option<&'a Path>,
}