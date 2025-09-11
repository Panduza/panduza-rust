# Reactor Specification

## Generic specification files
- Coding rules of the project: `req\coding_rules.req.md`

## File Path
`src/path.rs`

## Module Overview
This module provides handy functions to access to all standardized paths of Panduza on systems.
This module must work for any OS (Windows, Linux, Mac).

## Standardized Paths

### User root directory
This path must bring into the directory called `.panduza` inside the home of the user.

### Platform configuration file
This file is a json5 file called `platform.json5`. It is located directly in the user root dir.

### Credential directory
This directory called `credential` must be located directly in the user root dir.

In this directory there is 2 sub-directory
- client: for client credentials
- platform: for platform credentials

Client directory can contain the following standart files:
- platform_root_ca.pem (to validate platform validity)
- client_root_ca_key.pem (to create a client root ca)
- client_root_ca.pem
- client_key.pem (to auth to the platform)
- client.pem

Platform directory can contain the following standart files:
- client_root_ca.pem (to validate client validity)
- platform_root_ca_key.pem
- platform_root_ca.pem
- platform_key.pem
- platform.pem

# Unit tests
DO NOT GENERATE UNIT TESTS
