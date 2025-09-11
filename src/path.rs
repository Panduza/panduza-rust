//! Path utilities for Panduza standardized file system locations
//!
//! This module provides handy functions to access all standardized paths of Panduza on systems.
//! It works cross-platform (Windows, Linux, Mac).

use std::fs;
use std::io;
use std::path::PathBuf;

/// Get the user root directory for Panduza
///
/// Returns the path to the `.panduza` directory inside the user's home directory.
///
/// # Returns
///
/// `Some(PathBuf)` containing the path to `~/.panduza`, or `None` if the home directory cannot be determined.
pub fn user_root_dir() -> Option<PathBuf> {
    dirs::home_dir().map(|home| home.join(".panduza"))
}

/// Get the path to the platform configuration file
///
/// Returns the path to `platform.json5` located in the user root directory.
///
/// # Returns
///
/// `Some(PathBuf)` containing the path to the platform configuration file, or `None` if the home directory cannot be determined.
pub fn platform_config_file() -> Option<PathBuf> {
    user_root_dir().map(|root| root.join("platform.json5"))
}

/// Get the path to the credential directory
///
/// Returns the path to the `credential` directory located in the user root directory.
///
/// # Returns
///
/// `Some(PathBuf)` containing the path to the credential directory, or `None` if the home directory cannot be determined.
pub fn credential_dir() -> Option<PathBuf> {
    user_root_dir().map(|root| root.join("credential"))
}

/// Get the path to the client credential directory
///
/// Returns the path to the `credential/client` directory.
///
/// # Returns
///
/// `Some(PathBuf)` containing the path to the client credential directory, or `None` if the home directory cannot be determined.
pub fn client_credential_dir() -> Option<PathBuf> {
    credential_dir().map(|cred| cred.join("client"))
}

/// Get the path to the platform credential directory
///
/// Returns the path to the `credential/platform` directory.
///
/// # Returns
///
/// `Some(PathBuf)` containing the path to the platform credential directory, or `None` if the home directory cannot be determined.
pub fn platform_credential_dir() -> Option<PathBuf> {
    credential_dir().map(|cred| cred.join("platform"))
}

// Client credential standard files

/// Get the path to the platform root CA certificate for client validation
pub fn client_platform_root_ca_cert() -> Option<PathBuf> {
    client_credential_dir().map(|dir| dir.join("platform_root_ca.pem"))
}

/// Get the path to the client root CA private key
pub fn client_root_ca_key() -> Option<PathBuf> {
    client_credential_dir().map(|dir| dir.join("client_root_ca_key.pem"))
}

/// Get the path to the client root CA certificate
pub fn client_root_ca_cert() -> Option<PathBuf> {
    client_credential_dir().map(|dir| dir.join("client_root_ca.pem"))
}

/// Get the path to the client private key for platform authentication
pub fn client_key() -> Option<PathBuf> {
    client_credential_dir().map(|dir| dir.join("client_key.pem"))
}

/// Get the path to the client certificate for platform authentication
pub fn client_cert() -> Option<PathBuf> {
    client_credential_dir().map(|dir| dir.join("client.pem"))
}

// Platform credential standard files

/// Get the path to the client root CA certificate for client validation
pub fn platform_client_root_ca_cert() -> Option<PathBuf> {
    platform_credential_dir().map(|dir| dir.join("client_root_ca.pem"))
}

/// Get the path to the platform root CA private key
pub fn platform_root_ca_key() -> Option<PathBuf> {
    platform_credential_dir().map(|dir| dir.join("platform_root_ca_key.pem"))
}

/// Get the path to the platform root CA certificate
pub fn platform_root_ca_cert() -> Option<PathBuf> {
    platform_credential_dir().map(|dir| dir.join("platform_root_ca.pem"))
}

/// Get the path to the platform private key
pub fn platform_key() -> Option<PathBuf> {
    platform_credential_dir().map(|dir| dir.join("platform_key.pem"))
}

/// Get the path to the platform certificate
pub fn platform_cert() -> Option<PathBuf> {
    platform_credential_dir().map(|dir| dir.join("platform.pem"))
}

// Directory and file management functions

/// Ensure that the user root directory exists
///
/// Creates the `.panduza` directory in the user's home directory if it doesn't exist.
///
/// # Returns
///
/// `Ok(())` if the directory exists or was created successfully, or an `io::Error` if creation failed.
pub fn ensure_user_root_dir_exists() -> io::Result<()> {
    if let Some(dir) = user_root_dir() {
        fs::create_dir_all(dir)
    } else {
        Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Unable to determine home directory",
        ))
    }
}

/// Ensure that the credential directory structure exists
///
/// Creates the `credential`, `credential/client`, and `credential/platform` directories
/// if they don't exist.
///
/// # Returns
///
/// `Ok(())` if all directories exist or were created successfully, or an `io::Error` if creation failed.
pub fn ensure_credential_dir_structure_exists() -> io::Result<()> {
    // First ensure user root directory exists
    ensure_user_root_dir_exists()?;

    // Create credential directory
    if let Some(cred_dir) = credential_dir() {
        fs::create_dir_all(&cred_dir)?;
    } else {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Unable to determine credential directory path",
        ));
    }

    // Create client credential directory
    if let Some(client_dir) = client_credential_dir() {
        fs::create_dir_all(&client_dir)?;
    } else {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Unable to determine client credential directory path",
        ));
    }

    // Create platform credential directory
    if let Some(platform_dir) = platform_credential_dir() {
        fs::create_dir_all(&platform_dir)?;
    } else {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Unable to determine platform credential directory path",
        ));
    }

    Ok(())
}

/// Check if a file exists at the given path
///
/// # Arguments
///
/// * `path` - The path to check
///
/// # Returns
///
/// `true` if the file exists, `false` otherwise
pub fn file_exists(path: &PathBuf) -> bool {
    path.exists() && path.is_file()
}

/// Check if a directory exists at the given path
///
/// # Arguments
///
/// * `path` - The path to check
///
/// # Returns
///
/// `true` if the directory exists, `false` otherwise
pub fn dir_exists(path: &PathBuf) -> bool {
    path.exists() && path.is_dir()
}

/// Ensure that a directory exists
///
/// Creates the directory and all parent directories if they don't exist.
///
/// # Arguments
///
/// * `path` - The path to the directory to ensure exists
///
/// # Returns
///
/// `Ok(())` if the directory exists or was created successfully, or an `io::Error` if creation failed.
pub fn ensure_dir_exists(path: &PathBuf) -> io::Result<()> {
    fs::create_dir_all(path)
}
