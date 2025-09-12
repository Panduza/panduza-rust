# Structure Attribute Module Specification

## Generic specification files
- Coding rules of the project: `req\coding_rules.req.md`
- Panduza topic structuration: `req\panduza_topics.req.md`

## File Path
`src/attribute/structure.rs`

## Module Overview
This module provides a high-level wrapper for managing `StructureBuffer` attributes in the Panduza system. It encapsulates the generic `StdObjAttribute<StructureBuffer>` to provide structure-specific functionality.

## Dependencies and Imports

### External Crates
- `zenoh::Session` (for Zenoh communication)
- `std::time::Duration` (for timeout handling)
- `std::pin::Pin` (for async callback futures)
- `std::future::Future` (for async operations)

### Internal Modules
- `super::std_obj::StdObjAttribute` (generic attribute implementation)
- `super::CallbackId` (callback identifier type)
- `crate::fbs::StructureBuffer` (FlatBuffers structure buffer)
- `crate::AttributeMetadata` (attribute metadata)
- `crate::fbs::PzaBuffer` (If you need to use trait functions)
- `super::structure::flat::FlatStructure` (flattened structure representation)

## Core Structure

### StructureAttribute
**Derive Attributes**: `Clone, Debug`

**Purpose**: High-level wrapper for managing structure attributes with tree-like data representation.

**Fields**:
```rust
/// Flat version of the structure to ease find algorithms
pub flat: FlatStructure,

// Internal generic implementation based on an already design manager
pub inner: StdObjAttribute<StructureBuffer>,
```

## Specific management of `flat`

Field `flat` is a `FlatStructure` instance that contains a flattened representation of the hierarchical StructureBuffer data to ease operations of finding metadata based on topic input.

- Each time we receive a StructureBuffer, `flat` must be updated using `FlatStructure::update_from_buffer()` (you can use add_callback for this)
- The `FlatStructure` automatically handles the flattening logic:
  - Only inserts entries for nodes that contain both 'type' and 'mode' fields (indicating valid attribute leaves)
  - Each key is a complete Panduza topic without the final [cmd|att] suffix (see `req\panduza_topics.req.md` for more information)
  - Recursively processes the structure tree to build topic paths from root elements as instance topics
- Use `FlatStructure` methods like `get()`, `find_attributes()`, and `get_topics()` for attribute lookup operations

## Implementation Methods

### Constructor
- `new(session: Session, metadata: AttributeMetadata) -> Self`
  - **Async**: Yes
  - **Purpose**: Creates a new StructureAttribute instance
  - **Logic**: Wraps `StdObjAttribute<StructureBuffer>::new()` call

### Value Operations
- `wait_for_value<F>(predicate: F, timeout: Option<Duration>) -> Result<(), String>`
  - **Async**: Yes
  - **Generic**: `F: Fn(&StructureBuffer) -> bool + Send + Sync + 'static`
  - **Purpose**: Waits for a specific StructureBuffer value matching predicate
  - **Returns**: Unit result, discarding the actual value

### Callback Management
- `add_callback<F, C>(callback: F, condition: Option<C>) -> CallbackId`
  - **Async**: Yes
  - **Generic**: 
    - `F: Fn(StructureBuffer) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync + 'static`
    - `C: Fn(&StructureBuffer) -> bool + Send + Sync + 'static`
  - **Purpose**: Registers a callback triggered on StructureBuffer reception
  - **Returns**: Callback identifier for later removal

- `remove_callback(callback_id: CallbackId) -> bool`
  - **Async**: Yes
  - **Purpose**: Removes a callback by its ID
  - **Returns**: Success boolean

### Metadata Access
- `metadata() -> &AttributeMetadata`
  - **Async**: No
  - **Purpose**: Provides read-only access to attribute metadata
  - **Returns**: Reference to metadata

- `get() -> Option<StructureBuffer>`
  - **Async**: Yes
  - **Purpose**: Return the last structure value recieved

- `get_as_json_string() -> Option<String>`
  - **Async**: Yes
  - **Purpose**: Return the last structure value recieved as json string

- `find_attribute<A: Into<String>>(&self, pattern: A) -> Option<&AttributeMetadata>`
  - **Async**: No
  - **Purpose**: Use the flat field to find attributes matching the `pattern` using `FlatStructure::find_attributes()`
  - **Returns**: Reference to metadata if found

- `get_attribute_by_topic(&self, topic: &str) -> Option<&AttributeMetadata>`
  - **Async**: No
  - **Purpose**: Get a specific attribute by its exact topic path using `FlatStructure::get()`
  - **Returns**: Reference to metadata if found

- `get_all_topics(&self) -> Vec<&String>`
  - **Async**: No
  - **Purpose**: Get all available attribute topics using `FlatStructure::get_topics()`
  - **Returns**: Vector of all topic paths

- `flat_structure_len(&self) -> usize`
  - **Async**: No
  - **Purpose**: Get the number of flattened attributes using `FlatStructure::len()`
  - **Returns**: Number of attributes in the flat structure

## Design Patterns

### Delegation Pattern
All methods delegate to the inner `StdObjAttribute<StructureBuffer>` implementation, providing a type-safe interface for structure-specific operations.

### Async-First Design
All operations except metadata access are asynchronous, following Zenoh's async communication model.

### Generic Callback System
Supports flexible callback registration with optional conditions, enabling reactive programming patterns for structure data changes.

### Flat Structure Integration
The `FlatStructure` type encapsulates the flattening logic and provides a clean API for:
- Automatic hierarchy traversal and topic path construction
- Efficient attribute lookup by topic or pattern matching
- Deref traits for direct HashMap-like access when needed
- Type-safe operations on the flattened representation

## Usage Context
This module is part of the Panduza attribute system, specifically handling tree-structured data represented as FlatBuffers. It's typically used for device discovery, capability advertisement, and hierarchical data representation in IoT systems.
