use crate::config::Config;
use thiserror::Error as ThisError;
use zenoh::{open, Config as ZenohConfig, Session};

/// Error types for connection operations
#[derive(ThisError, Debug, Clone)]
pub enum ConnectionError {
    #[error("Failed to create Zenoh configuration: {cause}")]
    ConfigError { cause: String },

    #[error("Failed to establish Zenoh session: {cause}")]
    SessionError { cause: String },

    #[error("Invalid configuration: {message}")]
    InvalidConfig { message: String },
}

/// Creates a Zenoh client session based on the provided configuration
///
/// This function takes a client configuration and returns a Zenoh session.
/// If the config has the option `security.disable` set to true, the connection
/// must use TCP endpoints only.
///
/// # Arguments
/// * `config` - The client configuration containing platform and security settings
///
/// # Returns
/// A Result containing the Zenoh session or a ConnectionError
///
/// # Example
/// ```rust
/// use panduza::config::Config;
/// use panduza::connection::create_client_connection;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let mut config = Config::new();
///     // Configure for TCP connection with security disabled
///     if let Some(ref mut security) = config.security {
///         security.disable = Some(true);
///     }
///     
///     let session = create_client_connection(config).await?;
///     Ok(())
/// }
/// ```
pub async fn create_client_connection(config: Config) -> Result<Session, ConnectionError> {
    let zenoh_config = build_zenoh_config(&config)?;

    match open(zenoh_config).await {
        Ok(session) => Ok(session),
        Err(e) => Err(ConnectionError::SessionError {
            cause: e.to_string(),
        }),
    }
}

/// Builds a Zenoh configuration from the client config
fn build_zenoh_config(config: &Config) -> Result<ZenohConfig, ConnectionError> {
    let addr = config.get_platform_addr();
    let port = config.get_platform_port();
    let security_disabled = config.is_security_disabled();

    let zenoh_config_json = if security_disabled {
        // When security is disabled, use TCP endpoints only
        format!(
            r#"{{
                "mode": "client",
                "connect": {{
                    "endpoints": ["tcp/{}:{}"]
                }}
            }}"#,
            addr, port
        )
    } else {
        // When security is enabled, use QUIC with TLS
        // Note: This is a basic implementation. In a real scenario, you would
        // need to configure the actual certificate paths and security settings
        format!(
            r#"{{
                "mode": "client",
                "connect": {{
                    "endpoints": ["quic/{}:{}"]
                }},
                "transport": {{
                    "link": {{
                        "tls": {{
                            "root_ca_certificate": "./credentials/certificates/root_ca_certificate.pem",
                            "enable_mtls": true,
                            "connect_private_key": "./credentials/keys/writer_private_key.pem",
                            "connect_certificate": "./credentials/certificates/writer_certificate.pem"
                        }}
                    }}
                }}
            }}"#,
            addr, port
        )
    };

    ZenohConfig::from_json5(&zenoh_config_json).map_err(|e| ConnectionError::ConfigError {
        cause: format!("Failed to parse Zenoh configuration JSON: {}", e),
    })
}
