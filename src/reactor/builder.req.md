# Reactor Builder Specification

!!! TODO
change ca_certificate into platform_root_ca, this certificate used to validate the platform validity
connect_certificate into client_certificate
connect_private_key into client_private_key

## Generic specification files
- Coding rules of the project: `req\coding_rules.req.md`

## File Path
`src/reactor/builder.rs`

## Dependencies and Imports

### External Crates
- `zenoh::Session`
- `anyhow::Result` for error handling
- `serde_json::json` for JSON configuration building

### Internal Dependencies
- `crate::security::utils::get_default_certificate_paths` for default certificate paths

## Core Structure

ReactorBuilder is a builder pattern for Reactor in `src\reactor.rs`.

**Fields** (all public):
```rust
/// Address of the platform
address: Option<String>

/// Port of the platform
port: Option<u16>

/// Path to the file
ca_certificate: Option<String>

/// Path to the file
connect_certificate: Option<String>

/// Path to the file
connect_private_key: Option<String>

namespace: Option<String>
```

Special warning on the fact that fields about certificate and private key refers to path (not the content).

## Default Implementation

### Default Certificate Paths
The `Default` trait implementation automatically configures the builder with default certificate paths using `get_default_certificate_paths()` from the security utils module.

**Default behavior:**
- `address`: `None` (must be set by user)
- `port`: `None` (must be set by user)
- `ca_certificate`: Automatically set to default CA certificate path from user's `.panduza/certificate/` directory
- `connect_certificate`: Automatically set to default client certificate path from user's `.panduza/certificate/` directory  
- `connect_private_key`: Automatically set to default client private key path from user's `.panduza/keys/` directory
- `namespace`: `None` (optional)

**Implementation details:**
```rust
impl Default for ReactorBuilder {
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
```

**Usage:**
- `ReactorBuilder::new()` creates a builder with default certificate paths
- Users only need to set `address` and `port` for basic usage
- Certificate paths can be overridden if needed using the respective setter methods

## Manage Zenoh configuration

To create the Zenoh session, build the json config with this template

```json
{
    "mode": "client",
    "connect": {
        "endpoints": ["quic/{}:{}"]
    },
    "transport": {
        "link": {
            "tls": {
                "root_ca_certificate": "{}",
                "enable_mtls": true,
                "connect_private_key": "{}",
                "connect_certificate": "{}"
            }
        }
    }
}
```

## Validation Requirements

### Required Fields Validation
Security-related fields must be validated before use, but note that certificate fields now have default values:
- `address`: Address is required (no default)
- `port`: Port is required (no default)
- `ca_certificate`: CA certificate path is required (default provided)
- `connect_certificate`: Connect certificate path is required (default provided)
- `connect_private_key`: Connect private key path is required (default provided)

**Note:** With the default implementation, only `address` and `port` typically need to be set by the user, as certificate paths are automatically configured to default locations.

### Error Messages
Error messages should be descriptive and indicate which field is missing or what operation failed.

## Error Handling Requirements

### Zenoh Integration Error Handling
When working with Zenoh APIs, avoid using `map_err(anyhow::Error::from)` directly as Zenoh errors may not implement the required traits for automatic conversion.

**Required Pattern:**
```rust
// Instead of: .map_err(anyhow::Error::from)
// Use: .map_err(|e| anyhow::anyhow!("Descriptive message: {}", e))
```

**Specific Cases:**
1. **Zenoh Config Parsing:**
   ```rust
   zenoh::Config::from_json5(&config_json)
       .map_err(|e| anyhow::anyhow!("Failed to parse Zenoh config: {}", e))?
   ```

2. **Zenoh Session Creation:**
   ```rust
   zenoh::open(config).await
       .map_err(|e| anyhow::anyhow!("Failed to open Zenoh session: {}", e))?
   ```

**During Zenoh Connection**
Zenoh connection is an important step, if it fails I need in the error the fully json string of the configuration.

### File System Operations
File reading operations should use the `?` operator as `std::io::Error` implements the required traits for `anyhow::Error` conversion.

