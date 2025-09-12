use serde::{Deserialize, Serialize};

/// Configuration structure for platform settings.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EndpointConfig {
    /// Platform address (default: "127.0.0.1")
    pub addr: Option<String>,
    /// Platform port (default: 7447)
    pub port: Option<u16>,
}

impl Default for EndpointConfig {
    fn default() -> Self {
        Self {
            addr: Some("127.0.0.1".to_string()),
            port: Some(7447),
        }
    }
}

/// Configuration structure for security settings.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SecurityConfig {
    /// To disable security (default: false)
    pub disable: Option<bool>,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            disable: Some(false),
        }
    }
}

/// Main configuration structure containing all client settings.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    /// Broker configuration settings
    pub platform: Option<EndpointConfig>,
    /// Security configuration settings
    pub security: Option<SecurityConfig>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            platform: Some(EndpointConfig::default()),
            security: Some(SecurityConfig::default()),
        }
    }
}

impl Config {
    /// Create a new Config with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Get the platform address, using default if not set
    pub fn get_platform_addr(&self) -> String {
        self.platform
            .as_ref()
            .and_then(|p| p.addr.as_ref())
            .cloned()
            .unwrap_or_else(|| "127.0.0.1".to_string())
    }

    /// Get the platform port, using default if not set
    pub fn get_platform_port(&self) -> u16 {
        self.platform.as_ref().and_then(|p| p.port).unwrap_or(7447)
    }

    /// Check if security is disabled
    pub fn is_security_disabled(&self) -> bool {
        self.security
            .as_ref()
            .and_then(|s| s.disable)
            .unwrap_or(false)
    }
}
