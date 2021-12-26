use std::path::PathBuf;

const APP_DIR: &'static str = ".godaddy-ddns";
const IP_FILE_NAME: &'static str = "ddns_ip";
const RECORDS_FILE_NAME: &'static str = "records.json";

/// Gets the application folder path and returns it if found.
pub fn get_application_folder_path() -> Option<PathBuf> {
    let home = dirs::home_dir();

    if home.is_none() {
        return None;
    }

    Some(PathBuf::from(format!(
        "{}/{}",
        home.unwrap().display(),
        APP_DIR
    )))
}

/// Gets the IP_FILE path and returns it if found.
pub fn get_ip_file_path() -> Option<PathBuf> {
    let app_folder = get_application_folder_path();

    if app_folder.is_none() {
        return None;
    }

    Some(PathBuf::from(format!(
        "{}/{}",
        app_folder.unwrap().display(),
        IP_FILE_NAME
    )))
}

/// Gets the RECORDS_FILE path and returns it if found.
pub fn get_records_file_path() -> Option<PathBuf> {
    let app_folder = get_application_folder_path();

    if app_folder.is_none() {
        return None;
    }

    Some(PathBuf::from(format!(
        "{}/{}",
        app_folder.unwrap().display(),
        RECORDS_FILE_NAME
    )))
}
