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

# Unit tests
DO NOT GENERATE UNIT TESTS
