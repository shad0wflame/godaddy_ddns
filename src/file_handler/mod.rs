use std::fs::{create_dir, read_to_string, File};
use std::io::{Error, ErrorKind, Write};
use std::path::Path;

use crate::file_handler::path_handler::{
    get_application_folder_path, get_ip_file_path, get_records_file_path,
};

pub mod path_handler;

/// Sets up the application folder.
pub fn application_folder_setup() -> std::io::Result<()> {
    let app_folder_path = get_application_folder_path();

    if app_folder_path.is_none() {
        return Err(Error::from(ErrorKind::NotFound));
    }

    let path = app_folder_path.unwrap();

    if path.exists() {
        return Ok(());
    }

    create_dir(&path)
}

/// Gets the contents of the RECORDS_FILE.
pub fn get_records_file() -> String {
    let path = get_records_file_path().expect("Couldn't get RECORDS_FILE path.");

    read_file(&path).expect(&format!(
        "{} does not exist. Please create it.",
        path.display()
    ))
}

/// Gets the contents of the IP_FILE.
pub fn get_ip_file() -> Option<String> {
    let path = get_ip_file_path().expect("Couldn't get IP_FILE path.");

    read_file(&path).ok()
}

/// Stores the current IP value in the FILE_NAME.
///
/// # Arguments
///
/// * `content` - A &[u8] holding the current IP value.
pub fn set_ip_file(content: &[u8]) -> () {
    let path = get_ip_file_path().expect("Couldn't get IP_FILE path.");

    write_file(&path, content)
}

/// Reads a file and returns its contents as String if Ok().
///
/// # Arguments
///
/// * `path` - A &Path holding the path of the file to be read.
fn read_file(path: &Path) -> std::io::Result<String> {
    read_to_string(path)
}

/// Writes a file.
///
/// # Arguments
///
/// * `path` - A &Path holding the path of the file to be written.
///
/// * `content` - A &[u8] holding the info that will be written to the file.
fn write_file(path: &Path, content: &[u8]) -> () {
    let display = &path.display();

    let mut file =
        File::create(path).expect(&format!("Error opening or creating file: {}", display));

    file.write_all(content)
        .expect(&format!("Error writing file: {}", display))
}
