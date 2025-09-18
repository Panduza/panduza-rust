# Executor Module
Provides tool functions for MCP (Model Context Protocol) server operations.

Responsabilites in the project:
- Provide tool functions for MCP server integration
- Handle reactor-based operations for test bench attributes
- Manage structure data retrieval and formatting
- Interface between Reactor and MCP server functionality

## General specifications
You have to read those rules before coding anything:
- Coding rules of the project: `req/coding_rules.req.md`

## File Path
This module must be coded into the following file:
- `src/executor.rs`

## Data Structures

### `Executor`
Main struct that provides tool functions for MCP server operations. Takes an initialized Reactor object and uses it for its actions.

**Fields:**
- `reactor: Reactor` - The initialized Reactor instance used for operations

**Traits:**
- Debug

**Methods:**
- `new(reactor: Reactor) -> Self`
  - **Purpose**: Constructor method to create a new Executor instance with the provided Reactor.

- `structure_get(&self) -> Result<String, Error>`
  - **Purpose**: Provide the JSON structure of all test bench attributes using the last structure received by the Structure attribute of the reactor.

- `attribute_boolean_get(&self, &topic: String) -> Result<bool, Error>`
  - **Purpose**: Return the value of the given boolean attribute topic

- `attribute_boolean_set(&self, &topic: String, value: bool) -> Result<(), Error>`
  - **Purpose**: Set the value of the given boolean attribute topic

## Specifics Points

### `Dependencies`
- use crate::Reactor for reactor integration

### `Structure Output Format`
The structure_get method must return a JSON structure in the following format:

```json
{
    "instance_1_device_name": {
        "_node": "instance",
        "class_foo": {
            "_node": "class",
            "attribute_1": {
                "_node": "attribute"
            }
        }
    },
    "instance_2_device_name": {
        "_node": "instance",
        "class_foo": {
            "_node": "class",
            "attribute_1": {
                "_node": "attribute"
            }
        }
    }
}
```

### `Public Interface`
- Executor must be a public struct
- Executor must be added as public use in the lib.rs

## Acceptance Tests
Do not generate unit tests yourself!

No specific tests required for this module.
