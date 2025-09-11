//! Path utilities for Panduza standardized file system locations
//!
//! This module provides handy functions to access all standardized paths of Panduza on systems.
//! It works cross-platform (Windows, Linux, Mac).

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_root_dir_structure() {
        if let Some(root) = user_root_dir() {
            // Should end with .panduza
            assert!(root.ends_with(".panduza"));
        }
    }

    #[test]
    fn test_platform_config_file_structure() {
        if let Some(config) = platform_config_file() {
            // Should end with platform.json5
            assert!(config.ends_with("platform.json5"));
            // Should be in .panduza directory
            assert!(config.parent().unwrap().ends_with(".panduza"));
        }
    }

    #[test]
    fn test_credential_directories_structure() {
        if let Some(cred_dir) = credential_dir() {
            assert!(cred_dir.ends_with("credential"));
        }

        if let Some(client_dir) = client_credential_dir() {
            assert!(client_dir.ends_with("client"));
        }

        if let Some(platform_dir) = platform_credential_dir() {
            assert!(platform_dir.ends_with("platform"));
        }
    }

    #[test]
    fn test_client_credential_files() {
        let files = [
            (client_platform_root_ca_cert(), "platform_root_ca.pem"),
            (client_root_ca_key(), "client_root_ca_key.pem"),
            (client_root_ca_cert(), "client_root_ca.pem"),
            (client_key(), "client_key.pem"),
            (client_cert(), "client.pem"),
        ];

        for (path_opt, filename) in files {
            if let Some(path) = path_opt {
                assert!(path.ends_with(filename));
                assert!(path.parent().unwrap().ends_with("client"));
            }
        }
    }

    #[test]
    fn test_platform_credential_files() {
        let files = [
            (platform_client_root_ca_cert(), "client_root_ca.pem"),
            (platform_root_ca_key(), "platform_root_ca_key.pem"),
            (platform_root_ca_cert(), "platform_root_ca.pem"),
            (platform_key(), "platform_key.pem"),
            (platform_cert(), "platform.pem"),
        ];

        for (path_opt, filename) in files {
            if let Some(path) = path_opt {
                assert!(path.ends_with(filename));
                assert!(path.parent().unwrap().ends_with("platform"));
            }
        }
    }
}
