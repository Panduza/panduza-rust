use std::fs;
use std::path::PathBuf;

pub fn ensure_panduza_dirs() -> (PathBuf, PathBuf) {
    let home_dir = dirs::home_dir().expect("Unable to find user folder");
    let panduza_dir = home_dir.join(".panduza");
    let keys_dir = panduza_dir.join("keys");
    let cert_dir = panduza_dir.join("certificate");

    fs::create_dir_all(&keys_dir).expect("Fail to create key folder");
    fs::create_dir_all(&cert_dir).expect("Fail to create certificate folder");

    (keys_dir, cert_dir)
}

pub enum PanduzaFileType {
    Key,
    Certificate,
    Csr,
}

pub fn write_panduza_file(file_type: PanduzaFileType, filename: &str, content: &str) -> std::io::Result<PathBuf> {
    let (keys_dir, cert_dir) = ensure_panduza_dirs();
    let path = match file_type {
        PanduzaFileType::Key => keys_dir.join(filename),
        PanduzaFileType::Certificate => cert_dir.join(filename),
        PanduzaFileType::Csr => cert_dir.join(filename),
    };
    std::fs::write(path.clone(), content)?;
    Ok(path)
} 

pub fn get_panduza_dir(file_type: PanduzaFileType) -> PathBuf {
    let (keys_dir, cert_dir) = ensure_panduza_dirs();
    match file_type {
        PanduzaFileType::Key => keys_dir,
        PanduzaFileType::Certificate => cert_dir,
        PanduzaFileType::Csr => cert_dir,
    }
}

pub fn key_exists(filename: &str) -> bool {
    let (keys_dir, _) = ensure_panduza_dirs();
    let key_path = keys_dir.join(filename);
    key_path.exists()
}

pub fn certificate_exists(filename: &str) -> bool {
    let (_, cert_dir) = ensure_panduza_dirs();
    let cert_path = cert_dir.join(filename);
    cert_path.exists()
}

pub fn list_existing_keys() -> std::io::Result<Vec<String>> {
    let (keys_dir, _) = ensure_panduza_dirs();
    let mut keys = Vec::new();
    
    if keys_dir.exists() {
        for entry in fs::read_dir(keys_dir)? {
            let entry = entry?;
            if entry.file_type()?.is_file() {
                if let Some(filename) = entry.file_name().to_str() {
                    keys.push(filename.to_string());
                }
            }
        }
    }
    
    Ok(keys)
} 

pub fn get_default_certificate_paths() -> (PathBuf, PathBuf, PathBuf) {
    let (keys_dir, cert_dir) = ensure_panduza_dirs();
    let root_ca_path = cert_dir.join("root_ca_certificate.pem");
    let client_cert_path = cert_dir.join("client_certificate.pem");
    let client_key_path = keys_dir.join("client_private_key.pem");
    (root_ca_path, client_cert_path, client_key_path)
}

pub fn get_certificate_or_key_path(filename: &str) -> PathBuf {
    let (keys_dir, cert_dir) = ensure_panduza_dirs();
    
    if filename.ends_with("certificate.pem") {
        cert_dir.join(filename)
    } else if filename.ends_with("key.pem") {
        keys_dir.join(filename)
    } else {
        let home_dir = dirs::home_dir().expect("Unable to find user folder");
        home_dir.join(".panduza").join(filename)
    }
}
