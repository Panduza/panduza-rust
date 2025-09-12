use super::StructureBufferBuilder;

#[test]
fn test_insert_node_simple() {
    let mut root = StructureBufferBuilder::default()
        .with_name("root".to_string());
    
    let child_node = StructureBufferBuilder::default()
        .with_name("child".to_string())
        .with_node("Attribute".to_string());
    
    // Insert node at path ["child"]
    root.insert_node(vec!["child".to_string()], child_node);
    
    // Verify the child was inserted
    assert!(root.is_children_exists_with_name("child"));
}
