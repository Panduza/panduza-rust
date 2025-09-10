# Reactor Specification

## Generic specification files
- Coding rules of the project: `req\coding_rules.req.md`

## File Path
`src/reactor.rs`

## Module Overview
This module provides a main entry point for user of this client.

## Imports

- `mod builder`
- `builder::ReactorBuilder`

## Core Structure

### Reactor
**Derive Attributes**: `Clone, Debug`

- PartialEq must be manually implemented just checking if both session zid are equals.

**Fields** (all public):
```rust
// The zenoh session
session: Session,

// The structure attribute
structure: StructureAttribute,
```

## Implementation Methods

### Constructor

- `new(session: Session) -> Self`
  - **Async**: Yes
  - **Purpose**: Creates a new StructureAttribute instance
  - **Logic**: Wraps `StdObjAttribute<StructureBuffer>::new()` call

### Builder getter

- `builder()`
  - **Async**: No
  - **Purpose**: Static function to return the builder of this object

### Generic attribute getters

- `find_attribute(Into<String>) -> AttributeBuilder`
  - **Async**: No
  - **Purpose**: Create an attribute builder finding metadata in structure attribute.

- `get_structure_attribute() -> StructureAttribute`
  - **Async**: Yes
  - **Purpose**: Clone the inner structure attibute instanciated in the reactor

- `new_status_attribute() -> StatusAttribute`
  - **Async**: Yes
  - **Purpose**: Create a new status attribute on "pza/_/status"

- `new_notification_attribute() -> NotificationAttribute`
  - **Async**: Yes
  - **Purpose**: Create a new notification attribute on "pza/_/notifications"

