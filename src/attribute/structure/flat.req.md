# Structure Flat Transformation Specification

## Generic specification files
- Coding rules of the project: `req\coding_rules.req.md`
- Panduza topic structuration: `req\panduza_topics.req.md`

## File Path
`src\attribute\structure\flat.rs`

## Module Overview

This module propose an HashMap representation of the StructureBuffer (`HashMap<String, AttributeMetadata>`).

## Data Structure

The main structure of this module is `FlatStructure`
This structure can be build by passing a StructureBuffer.

**derive traits**: Serialize

## Coonversion Rules

- Insert a entry in the map only if StructureBuffer node contains a `mode`, it means that it is a leaf of the tree and a valid attribute to insert.
- Each key is a complete Panduza topic without the final [cmd|att] see `req\panduza_topics.req.md` for more information
- To build a topic take root elements as instance of the topics
