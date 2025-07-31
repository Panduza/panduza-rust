# Reactor Builder Specification

## Generic specification files
- Coding rules of the project: `req\coding_rules.req.md`

## File Path
`src/reactor/builder.rs`

## Dependencies and Imports

### External Crates
- `zenoh::Session`
- `anyhow::Result` for error handling
- `serde_json::json` for JSON configuration building

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
All security-related fields must be validated before use:
- `address`: Address is required
- `port`: Port is required  
- `ca_certificate`: CA certificate path is required
- `connect_certificate`: Connect certificate path is required
- `connect_private_key`: Connect private key path is required

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

