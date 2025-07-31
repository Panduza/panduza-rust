# StructureBuffer Specification

## Generic specification files
- Coding rules of the project: `req\coding_rules.req.md`

## Overview

The `StructureBuffer` is a specialized buffer implementation that handles FlatBuffer-based structure messages in the Panduza framework. It provides serialization and deserialization capabilities for tree-structured data with nodes and attributes.

## Purpose

The `StructureBuffer` is designed to:
- Store and manage FlatBuffer serialized structure data
- Provide conversion between Zenoh bytes and internal buffer format
- Extract metadata from message headers (source, sequence)
- Support tree-structured data representation with nodes and attributes

## Data Structure

### StructureBuffer

```rust
#[derive(Default, Clone, Debug, PartialEq)]
pub struct StructureBuffer {
    pub raw_data: Bytes,
}
```

**Fields:**
- `raw_data: Bytes` - The raw FlatBuffer serialized data containing the structure message

## Trait Implementations

### PzaBuffer Trait

The `StructureBuffer` implements the `PzaBuffer` trait, providing standardized buffer operations:

#### Required Methods

##### `from_zbytes(zbytes: ZBytes) -> Self`
- **Purpose**: Creates a `StructureBuffer` from Zenoh bytes
- **Input**: `zbytes` - Zenoh bytes received from network
- **Output**: New `StructureBuffer` instance
- **Behavior**: Converts ZBytes to internal Bytes format and wraps in StructureBuffer

##### `to_zbytes(self) -> ZBytes`
- **Purpose**: Converts the buffer to Zenoh bytes for transmission
- **Input**: `self` - The StructureBuffer instance
- **Output**: ZBytes ready for network transmission
- **Behavior**: Converts internal Bytes to ZBytes format

##### `size(&self) -> usize`
- **Purpose**: Returns the size of the buffer in bytes
- **Input**: `&self` - Reference to the buffer
- **Output**: Size in bytes
- **Behavior**: Returns the length of the internal raw_data

##### `source(&self) -> Option<u16>`
- **Purpose**: Extracts the source identifier from the message header
- **Input**: `&self` - Reference to the buffer
- **Output**: Optional source ID
- **Behavior**: Deserializes the message and extracts source from header

##### `sequence(&self) -> Option<u16>`
- **Purpose**: Extracts the sequence number from the message header
- **Input**: `&self` - Reference to the buffer
- **Output**: Optional sequence number
- **Behavior**: Deserializes the message and extracts sequence from header

##### `as_message(&self) -> Message`
- **Purpose**: Deserializes the FlatBuffer data into a Message object
- **Input**: `&self` - Reference to the buffer
- **Output**: Deserialized Message object
- **Behavior**: Uses flatbuffers::root to deserialize raw_data
- **Error Handling**: Panics with "STRUCTURE: Failed to deserialize Message from raw_data" if deserialization fails

##### `has_same_message_value<B: PzaBuffer>(&self, _other_buffer: &B) -> bool`
- **Purpose**: Compares message values between buffers
- **Input**: `&self` - Reference to this buffer, `_other_buffer` - Reference to other buffer
- **Output**: Boolean indicating if messages have same value
- **Current Implementation**: Always returns `true` (placeholder implementation)
- **Note**: This is a placeholder and should be implemented properly for actual value comparison

## Instance Methods

### `builder() -> StructureBufferBuilder`
- **Purpose**: Creates a new StructureBufferBuilder for constructing StructureBuffer instances
- **Input**: None (static method)
- **Output**: Default StructureBufferBuilder instance
- **Usage**: Used to build StructureBuffer with specific configurations

### `as_json() -> serde_json::Value`
- **Purpose**: Create a json version of the structure

The json output must follow the given format

```json
{
    "instance_1_device_name": {
        "_node": "instance",
        "class_foo" {
            "_node": "class",
            "tags": [ "tag1", "tag2" ],
            "attribute_1": {
                "_node": "attribute",
                "type": "string",
                "mode": "RW"
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

## Dependencies

The module depends on:
- `super::StructureBufferBuilder` - Builder pattern implementation
- `crate::fbs::panduza_generated::panduza::Message` - FlatBuffer generated message types
- `crate::PzaBuffer` - Core buffer trait
- `bytes::Bytes` - Efficient byte storage
- `zenoh::bytes::ZBytes` - Zenoh networking byte format

## Usage Patterns

### Creating a StructureBuffer from network data
```rust
let zbytes = /* received from Zenoh */;
let buffer = StructureBuffer::from_zbytes(zbytes);
```

### Building a new StructureBuffer
```rust
let buffer = StructureBuffer::builder()
    .with_name("node_name")
    .with_type("sensor")
    .build();
```

### Extracting metadata
```rust
let source = buffer.source();
let sequence = buffer.sequence();
let size = buffer.size();
```

### Converting for transmission
```rust
let zbytes = buffer.to_zbytes();
// Send zbytes over Zenoh
```

## Error Handling

- **Deserialization Errors**: The `as_message()` method will panic if the raw_data cannot be deserialized as a valid FlatBuffer Message
- **Data Integrity**: No explicit validation is performed on the raw_data format beyond FlatBuffer deserialization

## Performance Considerations

- Uses `Bytes` for efficient memory management and zero-copy operations where possible
- FlatBuffer format provides efficient serialization/deserialization
- Clone operations are relatively cheap due to `Bytes` reference counting

