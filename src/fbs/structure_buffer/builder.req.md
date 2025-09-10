# Structure Buffer Module Specification

## File Path
`src/fbs/structure_buffer/builder.rs`

## Module Overview
This module provides a builder for `StructureBuffer` in file `src/fbs/structure_buffer/buffer.rs`.

## Dependencies and Imports

### External Crates
- `bytes::Bytes`
- `flatbuffers` (for serialization)
- `rand` (for random sequence generation)

### Internal Modules
- `crate::fbs::generate_timestamp`
- `crate::fbs::PzaBufferBuilder`
- Multiple FlatBuffers generated types from `panduza_generated::panduza`

## Core Structure

### StructureBufferBuilder
**Derive Attributes**: `Default, Clone, PartialEq`

**Fields** (all public):
```rust
pub name: Option<String>,          // Name of this node

pub node: Option<String>,          // Node type

// Child nodes, because the structure represent a tree
pub children: Option<Vec<StructureBufferBuilder>>, 

pub tags: Vec<String>,             // Node tags

pub r#type: Option<String>,        // Attribute type
pub mode: Option<String>,          // Attribute mode (read/write)

pub source: Option<u16>,           // Message source
pub sequence: Option<u16>,         // Message sequence
```

## Trait Implementations

### PzaBufferBuilder<StructureBuffer> for StructureBufferBuilder

**Required Methods**:
- `with_source(self, source: u16) -> Self`
- `with_sequence(self, sequence: u16) -> Self`
- `with_random_sequence(self) -> Self` (uses `rand::random()`)
- `build(self) -> Result<StructureBuffer, String>`

**Build Method Logic**:
1. Create FlatBufferBuilder
2. Generate timestamp
3. Serialize name (optional string)
4. Serialize tags vector
5. Serialize children recursively (commented out attributes handling)
6. Create Structure with StructureArgs
7. Create Header with timestamp, source, sequence
8. Create Message with header and Structure payload
9. Finish buffer and wrap in StructureBuffer

## Builder-Specific Methods

### StructureBufferBuilder Methods

- One method to set each field not already set by `PzaBufferBuilder` implementation

- `with_tag(self, tag: String) -> Self` (appends to tags vector)
- `with_tags(self, tags: Vec<String>) -> Self` (replaces tags vector)
- `build_wip_offset<'a>(&self, builder: &mut FlatBufferBuilder<'a>) -> WIPOffset<Structure<'a>>`
- `insert_child(&mut self, child: StructureBufferBuilder)`
- `is_children_exists_with_name(&self, name: &str) -> bool`
- `insert_node(&mut self, path: Vec<String>, node: StructureBufferBuilder)` (recursive tree insertion)
