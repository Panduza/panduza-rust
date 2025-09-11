use std::path::PathBuf;

/// This module provides handy functions to access to all standardized paths of Panduza on systems.
/// This module works for any OS (Windows, Linux, Mac)

// ----------------------------------------------------------------------------

/// Returns the user root directory path for Panduza.
/// This path points to the `.panduza` directory inside the user's home directory.
///
/// # Returns
///
/// A `Result<PathBuf, std::io::Error>` containing:
/// - `Ok(PathBuf)`: The path to the `.panduza` directory in the user's home
/// - `Err(std::io::Error)`: If the home directory cannot be determined
///
/// # Examples
///
/// ```rust
/// use panduza::path::user_root;
///
/// match user_root() {
///     Ok(path) => println!("Panduza user root: {:?}", path),
///     Err(e) => eprintln!("Error getting user root: {}", e),
/// }
/// ```
pub fn user_root() -> Result<PathBuf, std::io::Error> {
    let home_dir = dirs::home_dir().ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Unable to determine home directory",
        )
    })?;

    Ok(home_dir.join(".panduza"))
}

// ----------------------------------------------------------------------------

/// Ensures that the user root directory exists.
/// Creates the `.panduza` directory in the user's home if it doesn't exist.
///
/// # Returns
///
/// A `Result<PathBuf, std::io::Error>` containing:
/// - `Ok(PathBuf)`: The path to the created/existing `.panduza` directory
/// - `Err(std::io::Error)`: If the directory cannot be created or accessed
///
/// # Examples
///
/// ```rust
/// use panduza::path::ensure_user_root;
///
/// match ensure_user_root() {
///     Ok(path) => println!("Panduza user root ready: {:?}", path),
///     Err(e) => eprintln!("Error ensuring user root: {}", e),
/// }
/// ```
pub fn ensure_user_root() -> Result<PathBuf, std::io::Error> {
    let root_path = user_root()?;

    if !root_path.exists() {
        std::fs::create_dir_all(&root_path)?;
    }

    Ok(root_path)
}

// ----------------------------------------------------------------------------

/// Returns the path to the configuration directory within the Panduza user root.
/// This directory is used to store configuration files.
///
/// # Returns
///
/// A `Result<PathBuf, std::io::Error>` containing:
/// - `Ok(PathBuf)`: The path to the configuration directory
/// - `Err(std::io::Error)`: If the user root cannot be determined
pub fn config_dir() -> Result<PathBuf, std::io::Error> {
    let root = user_root()?;
    Ok(root.join("config"))
}

// ----------------------------------------------------------------------------

/// Returns the path to the logs directory within the Panduza user root.
/// This directory is used to store log files.
///
/// # Returns
///
/// A `Result<PathBuf, std::io::Error>` containing:
/// - `Ok(PathBuf)`: The path to the logs directory
/// - `Err(std::io::Error)`: If the user root cannot be determined
pub fn logs_dir() -> Result<PathBuf, std::io::Error> {
    let root = user_root()?;
    Ok(root.join("logs"))
}

// ----------------------------------------------------------------------------

/// Returns the path to the cache directory within the Panduza user root.
/// This directory is used to store temporary cache files.
///
/// # Returns
///
/// A `Result<PathBuf, std::io::Error>` containing:
/// - `Ok(PathBuf)`: The path to the cache directory
/// - `Err(std::io::Error)`: If the user root cannot be determined
pub fn cache_dir() -> Result<PathBuf, std::io::Error> {
    let root = user_root()?;
    Ok(root.join("cache"))
}

// ----------------------------------------------------------------------------

/// Ensures that all standard Panduza directories exist.
/// Creates the user root, config, logs, and cache directories if they don't exist.
///
/// # Returns
///
/// A `Result<(), std::io::Error>` indicating:
/// - `Ok(())`: All directories have been created/verified successfully
/// - `Err(std::io::Error)`: If any directory cannot be created
pub fn ensure_all_dirs() -> Result<(), std::io::Error> {
    ensure_user_root()?;

    let config = config_dir()?;
    if !config.exists() {
        std::fs::create_dir_all(&config)?;
    }

    let logs = logs_dir()?;
    if !logs.exists() {
        std::fs::create_dir_all(&logs)?;
    }

    let cache = cache_dir()?;
    if !cache.exists() {
        std::fs::create_dir_all(&cache)?;
    }

    Ok(())
}
