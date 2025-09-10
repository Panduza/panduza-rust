use crate::security::utils::get_default_certificate_paths;

use super::Reactor;
use anyhow::Result;
use serde_json::json;
use zenoh::Session as ZenohSession;

/// Builder pattern for creating Reactor instances
///
/// ReactorBuilder provides a flexible way to configure and create Reactor instances
/// with various connection parameters and security credentials.
#[derive(Debug, Clone)]
pub struct ReactorBuilder {
    /// Address of the platform
    pub address: Option<String>,
    /// Port number to connect to
    pub port: Option<u16>,
    /// Path to CA certificate file
    pub ca_certificate: Option<String>,
    /// Path to client certificate file
    pub connect_certificate: Option<String>,
    /// Path to client private key file
    pub connect_private_key: Option<String>,
    /// Namespace for the connection
    pub namespace: Option<String>,
}

impl Default for ReactorBuilder {
    /// Creates a new ReactorBuilder with default values
    fn default() -> Self {
        let (root_ca_path, client_cert_path, client_key_path) = get_default_certificate_paths();
        let root_ca_path = root_ca_path.into_os_string().into_string().unwrap();
        let client_cert_path = client_cert_path.into_os_string().into_string().unwrap();
        let client_key_path = client_key_path.into_os_string().into_string().unwrap();

        Self {
            address: None,
            port: None,
            ca_certificate: Some(root_ca_path),
            connect_certificate: Some(client_cert_path),
            connect_private_key: Some(client_key_path),
            namespace: None,
        }
    }
}

impl ReactorBuilder {
    /// Creates a new ReactorBuilder instance
    pub fn new() -> Self {
        Self::default()
    }

    // ----------------------------------------------------------------------------

    /// Sets the address to connect to
    ///
    /// # Arguments
    /// * `address` - The address as a string
    pub fn address(mut self, address: String) -> Self {
        self.address = Some(address);
        self
    }

    // ----------------------------------------------------------------------------

    /// Sets the port number to connect to
    ///
    /// # Arguments
    /// * `port` - The port number
    pub fn port(mut self, port: u16) -> Self {
        self.port = Some(port);
        self
    }

    // ----------------------------------------------------------------------------

    /// Sets the CA certificate file path
    ///
    /// # Arguments
    /// * `ca_certificate` - Path to the CA certificate file
    pub fn ca_certificate(mut self, ca_certificate: String) -> Self {
        self.ca_certificate = Some(ca_certificate);
        self
    }

    // ----------------------------------------------------------------------------

    /// Sets the client certificate file path
    ///
    /// # Arguments
    /// * `connect_certificate` - Path to the client certificate file
    pub fn connect_certificate(mut self, connect_certificate: String) -> Self {
        self.connect_certificate = Some(connect_certificate);
        self
    }

    // ----------------------------------------------------------------------------

    /// Sets the client private key file path
    ///
    /// # Arguments
    /// * `connect_private_key` - Path to the client private key file
    pub fn connect_private_key(mut self, connect_private_key: String) -> Self {
        self.connect_private_key = Some(connect_private_key);
        self
    }

    // ----------------------------------------------------------------------------

    /// Sets the namespace for the connection
    ///
    /// # Arguments
    /// * `namespace` - The namespace string
    pub fn namespace(mut self, namespace: String) -> Self {
        self.namespace = Some(namespace);
        self
    }

    // ----------------------------------------------------------------------------

    /// Builds and returns a Reactor instance
    ///
    /// Creates a Zenoh session with the configured parameters and uses it to create a Reactor.
    ///
    /// # Returns
    /// A Result containing the Reactor instance or an error
    pub async fn build(self) -> Result<Reactor> {
        let session = self.create_zenoh_session().await?;
        Ok(Reactor::new(session).await)
    }

    // ----------------------------------------------------------------------------

    /// Creates a Zenoh session with the configured parameters
    ///
    /// # Returns
    /// A Result containing the Zenoh session or an error
    async fn create_zenoh_session(&self) -> Result<ZenohSession> {
        let config_json = self.build_zenoh_config()?;
        let config = zenoh::Config::from_json5(&config_json)
            .map_err(|e| anyhow::anyhow!("Failed to parse Zenoh config: {}", e))?;
        let session = zenoh::open(config).await.map_err(|e| {
            anyhow::anyhow!(
                "Failed to open Zenoh session: {}. Configuration used: {}",
                e,
                config_json
            )
        })?;
        Ok(session)
    }

    // ----------------------------------------------------------------------------

    /// Builds the Zenoh configuration JSON
    ///
    /// # Returns
    /// A Result containing the configuration JSON string or an error
    fn build_zenoh_config(&self) -> Result<String> {
        let address = self
            .address
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Address is required"))?;
        let port = self
            .port
            .ok_or_else(|| anyhow::anyhow!("Port is required"))?;

        let ca_cert_path = self
            .ca_certificate
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("CA certificate path is required"))?;
        let connect_cert_path = self
            .connect_certificate
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Connect certificate path is required"))?;
        let connect_key_path = self
            .connect_private_key
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("Connect private key path is required"))?;

        // Use file paths directly as specified in requirements
        // Warning: fields refer to paths (not the content)
        let config = json!({
            "mode": "client",
            "connect": {
                "endpoints": [format!("quic/{}:{}", address, port)]
            },
            "transport": {
                "link": {
                    "tls": {
                        "root_ca_certificate": ca_cert_path,
                        "enable_mtls": true,
                        "connect_private_key": connect_key_path,
                        "connect_certificate": connect_cert_path
                    }
                }
            }
        });

        Ok(config.to_string())
    }
}
