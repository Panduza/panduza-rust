use crate::Config;
use crate::Reactor;

/// Builder for creating Reactor instances
///
/// Provides a fluent interface for configuring and building Reactor instances
/// using the provided Config to establish the client session.
pub struct ReactorBuilder {
    /// Configuration for the reactor
    config: Config,
}

impl ReactorBuilder {
    /// Creates a new ReactorBuilder with default configuration
    ///
    /// # Returns
    /// A new ReactorBuilder instance with default Config
    pub fn new() -> Self {
        Self {
            config: Config::default(),
        }
    }

    // ------------------------------------------------------------------------

    /// Sets the configuration for the reactor
    ///
    /// # Arguments
    /// * `config` - The configuration to use for building the reactor
    ///
    /// # Returns
    /// Self for method chaining
    pub fn with_config(mut self, config: Config) -> Self {
        self.config = config;
        self
    }

    // ------------------------------------------------------------------------

    /// Sets the platform address
    ///
    /// # Arguments
    /// * `addr` - The platform address to connect to
    ///
    /// # Returns
    /// Self for method chaining
    pub fn with_platform_addr<S: Into<String>>(mut self, addr: S) -> Self {
        if self.config.platform.is_none() {
            self.config.platform = Some(crate::config::EndpointConfig::default());
        }
        if let Some(ref mut platform) = self.config.platform {
            platform.addr = Some(addr.into());
        }
        self
    }

    // ------------------------------------------------------------------------

    /// Sets the platform port
    ///
    /// # Arguments
    /// * `port` - The platform port to connect to
    ///
    /// # Returns
    /// Self for method chaining
    pub fn with_platform_port(mut self, port: u16) -> Self {
        if self.config.platform.is_none() {
            self.config.platform = Some(crate::config::EndpointConfig::default());
        }
        if let Some(ref mut platform) = self.config.platform {
            platform.port = Some(port);
        }
        self
    }

    // ------------------------------------------------------------------------

    /// Disables security for the connection
    ///
    /// # Returns
    /// Self for method chaining
    pub fn disable_security(mut self) -> Self {
        if self.config.security.is_none() {
            self.config.security = Some(crate::config::SecurityConfig::default());
        }
        if let Some(ref mut security) = self.config.security {
            security.disable = Some(true);
        }
        self
    }

    // ------------------------------------------------------------------------

    /// Enables security for the connection
    ///
    /// # Returns
    /// Self for method chaining
    pub fn enable_security(mut self) -> Self {
        if self.config.security.is_none() {
            self.config.security = Some(crate::config::SecurityConfig::default());
        }
        if let Some(ref mut security) = self.config.security {
            security.disable = Some(false);
        }
        self
    }

    // ------------------------------------------------------------------------

    /// Builds the Reactor instance using the configured settings
    ///
    /// Creates a Zenoh session based on the configuration and uses it
    /// to create a new Reactor instance.
    ///
    /// # Returns
    /// A Result containing the configured Reactor instance or an error
    ///
    /// # Errors
    /// Returns an error if the Zenoh session cannot be created
    pub async fn build(self) -> Result<Reactor, zenoh::Error> {
        // Create JSON5 configuration string for Zenoh
        let config_json = if self.config.is_security_disabled() {
            // Simple TCP connection without security
            format!(
                r#"{{
                    "mode": "client",
                    "connect": {{
                        "endpoints": ["tcp/{}:{}"]
                    }}
                }}"#,
                self.config.get_platform_addr(),
                self.config.get_platform_port()
            )
        } else {
            // TODO: Add TLS configuration when security is enabled
            // This would involve setting up certificates and keys from the credentials folder
            format!(
                r#"{{
                    "mode": "client",
                    "connect": {{
                        "endpoints": ["tcp/{}:{}"]
                    }}
                }}"#,
                self.config.get_platform_addr(),
                self.config.get_platform_port()
            )
        };

        // Parse the JSON5 configuration
        let zenoh_config = zenoh::Config::from_json5(&config_json)?;

        // Open the Zenoh session
        let session = zenoh::open(zenoh_config).await?;

        // Create and return the Reactor
        Ok(Reactor::new(session).await)
    }
}

impl Default for ReactorBuilder {
    fn default() -> Self {
        Self::new()
    }
}
