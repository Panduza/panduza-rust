# Client Config Specification

## Generic specification files
- Coding rules of the project: `req\coding_rules.req.md`

## File Path
`src/connection.rs`

## Module overview

This module provides easy function to create Zenoh session for Panduza purpose. 

## Functions

### create_client_connection

This function take a client config `create::Config` and return a Zenoh session as a Result.

If the config has the option `security.disable` to true, connection must be a TCP endpoint.

For example:

```json
"mode": "client",
"connect": {
    "endpoints": ["tcp/192.168.1.1:7447"],
},
```
