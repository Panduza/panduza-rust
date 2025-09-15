// cargo test fbs::structure_buffer::builder

use super::StructureBufferBuilder;

#[test]
fn test_insert_node_simple() {
    let mut root = StructureBufferBuilder::default().with_name("root".to_string());

    let child_node = StructureBufferBuilder::default()
        .with_name("child".to_string())
        .with_node("Attribute".to_string());

    let class_node = StructureBufferBuilder::default()
        .with_name("tototot".to_string())
        .with_node("Class".to_string());

    // Insert node at path ["child"]
    root.insert_node(vec!["tototot".to_string()], class_node);
    root.insert_node(vec!["tototot".to_string(), "child".to_string()], child_node);

    println!("Root after insertions: {:?}", root);

    // Verify the child was inserted
    assert!(root.is_children_exists_with_name("tototot"));
    // Ensure that "child" is not a direct child of root, but of "tototot"
    assert!(!root.is_children_exists_with_name("child"));

    // Optionally, check that "child" is a child of "tototot"
    let tototot = root
        .get_child_by_name("tototot")
        .expect("tototot should exist");
    assert!(tototot.is_children_exists_with_name("tototot") == false);
    assert!(tototot.is_children_exists_with_name("child"));
}
