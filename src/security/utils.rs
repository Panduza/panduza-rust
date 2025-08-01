use rcgen::KeyPair;
use std::fs;
use std::path::PathBuf;
use time::{Duration, OffsetDateTime};

pub enum PanduzaFileType {
    Key,
    Certificate,
    Csr,
}

/// Ensure the .panduza directory exist or create it with .panduza/keys and .panduza/certificate directories
pub fn ensure_panduza_dirs() -> (PathBuf, PathBuf) {
    let home_dir = dirs::home_dir().expect("Unable to find user folder");
    let panduza_dir = home_dir.join(".panduza");
    let keys_dir = panduza_dir.join("keys");
    let cert_dir = panduza_dir.join("certificate");

    fs::create_dir_all(&keys_dir).expect("Fail to create key folder");
    fs::create_dir_all(&cert_dir).expect("Fail to create certificate folder");

    (keys_dir, cert_dir)
}

/// Write a file in the panduza directories depending on the file type
pub fn write_panduza_file(
    file_type: PanduzaFileType,
    filename: &str,
    content: &str,
) -> std::io::Result<PathBuf> {
    let (keys_dir, cert_dir) = ensure_panduza_dirs();
    let path = match file_type {
        PanduzaFileType::Key => keys_dir.join(filename),
        PanduzaFileType::Certificate => cert_dir.join(filename),
        PanduzaFileType::Csr => cert_dir.join(filename),
    };
    std::fs::write(path.clone(), content)?;
    Ok(path)
}

/// Get the path to the panduza directories depending on the file type
pub fn get_panduza_dir(file_type: PanduzaFileType) -> PathBuf {
    let (keys_dir, cert_dir) = ensure_panduza_dirs();
    match file_type {
        PanduzaFileType::Key => keys_dir,
        PanduzaFileType::Certificate => cert_dir,
        PanduzaFileType::Csr => cert_dir,
    }
}

/// Check if a key exists in the panduza directories
pub fn key_exists(filename: &str) -> bool {
    let (keys_dir, _) = ensure_panduza_dirs();
    let key_path = keys_dir.join(filename);
    key_path.exists()
}

/// Check if a certificate exists in the panduza directories
pub fn certificate_exists(filename: &str) -> bool {
    let (_, cert_dir) = ensure_panduza_dirs();
    let cert_path = cert_dir.join(filename);
    cert_path.exists()
}

/// Get the default credentials paths of a user to create a Reactor
pub fn get_default_certificate_paths() -> (PathBuf, PathBuf, PathBuf) {
    let (keys_dir, cert_dir) = ensure_panduza_dirs();
    let root_ca_path = cert_dir.join("root_ca_certificate.pem");
    let client_cert_path = cert_dir.join("client_certificate.pem");
    let client_key_path = keys_dir.join("client_private_key.pem");
    (root_ca_path, client_cert_path, client_key_path)
}

/// Generate a validity period
pub fn validity_period(days: i32) -> (OffsetDateTime, OffsetDateTime) {
    let day = Duration::new(86400, 0);
    let yesterday = OffsetDateTime::now_utc().checked_sub(day).unwrap();
    let after = OffsetDateTime::now_utc().checked_add(day * days).unwrap();
    (yesterday, after)
}

/// Load a key rcgen struct KeyPair from a .pem file
pub fn load_key_from_pem(key_path: &str) -> Result<KeyPair, Box<dyn std::error::Error>> {
    let key_pem = fs::read_to_string(key_path)?;
    let key = KeyPair::from_pem(&key_pem)?;

    Ok(key)
}

/// Generate a key rcgen struct KeyPair
pub fn generate_key() -> KeyPair {
    KeyPair::generate().expect("Fail to generate key")
}
