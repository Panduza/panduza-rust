# Executor Specification

## Generic specification files
- Coding rules of the project: `req\coding_rules.req.md`

## File Path
`src\executor.rs`

## Imports
- `use crate::Reactor`

## Goal

The executor module must provides tool functions for a MCP server

The executor must take an intialized Reactor object and use it for its actions. Reactor is part of this project.

## Structure

- 'Executor' must be a public struct.
- 'Executor' must be added as public use in the lib.rs.

## Tool Functions

### structure_get

Provide the json structure of all the test bench attribute.
It must use the last structure recieved by the Structure attribute of the reactor.

```json
{
    "instance_1_device_name": {
        "_node": "instance",
        "class_foo" {
            "_node": "class",
            "attribute_1": {
                "_node": "attribute"
            }
        }
    },
    "instance_2_device_name": {
        "_node": "instance",
        "class_foo" {
            "_node": "class",
            "attribute_1": {
                "_node": "attribute",
            }
        }
    },
}
```

## Tests

No tests on this module
