# Client Config Specification

## Generic specification files
- Coding rules of the project: `req\coding_rules.req.md`

## File Path
`src/config.rs`

## Data Structures

### `PlatformConfig`

Configuration structure for platform settings.

**Fields:**
- `addr: Option<String>` - Platform address (default: "127.0.0.1")
- `port: Option<u16>` - Platform port (default: 7447)

**Traits:** Debug, Serialize, Deserialize, Clone

### `SecurityConfig`

Configuration structure for security settings.

**Fields:**
- `disable: Option<bool>` To disable security -  (default: false)

**Traits:** Debug, Serialize, Deserialize, Clone

### `Config`

Main configuration structure containing all client settings.

**Fields:**
- `platform: Option<PlatformConfig>` - Broker configuration settings
- `security: Option<SecurityConfig>` - Security configuration settings

**Traits:** Debug, Serialize, Deserialize, Clone
