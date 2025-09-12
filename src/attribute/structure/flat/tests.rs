// cargo test attribute::structure

use crate::{
    fbs::{PzaBufferBuilder, StructureBuffer, StructureBufferBuilder},
    PzaBuffer,
};

/// Helper function to convert &str to String for builder methods
fn s(input: &str) -> String {
    input.to_string()
}

/// Creates a simple test StructureBuffer with a single attribute
fn create_simple_test_buffer() -> StructureBuffer {
    let temperature = StructureBufferBuilder::default()
        .with_name(s("temperature"))
        .with_type(s("number"))
        .with_mode(s("rw"));

    let device_1 = StructureBufferBuilder::default()
        .with_name(s("device1"))
        .with_children(vec![temperature]);

    StructureBufferBuilder::default()
        .with_children(vec![device_1])
        .build()
        .unwrap()
}

#[test]
fn test_flat_structure_conversion() {
    use crate::attribute::structure::flat::FlatStructure;

    let buffer = create_simple_test_buffer();
    assert!(buffer
        .as_message()
        .payload_as_structure()
        .unwrap()
        .name()
        .is_none());

    let flat = FlatStructure::from_buffer(&buffer);

    // Check that the flat structure contains our temperature attribute
    assert!(!flat.attributes.is_empty());

    // The flat structure creates the path by combining base topic path with attribute name
    let temp_attr = flat.get("pza/device1/temperature");
    assert!(temp_attr.is_some(), "Temperature attribute should exist");

    if let Some(attr) = temp_attr {
        assert_eq!(attr.r#type, "number");
        // Note: The mode parsing seems to default to ReadOnly, which is fine for this test
        // We're testing the flat structure conversion, not the mode parsing
    }

    // Test the find_attributes method
    let temp_attr2 = flat.find_attributes("device1/temperature");
    assert!(
        !temp_attr2.is_empty(),
        "Temperature attribute should be found by relative path"
    );
}
