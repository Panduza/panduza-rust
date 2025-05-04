use cucumber::{given, then, when};

use super::BasicsWorld;

#[given(expr = "the string attribute rw {string}")]
fn the_string_attribute_rw(world: &mut BasicsWorld, s: String) {
    assert!(false, "Not implemented");
}

#[then(expr = "the rw string value is {string}")]
fn the_rw_string_value_is(world: &mut BasicsWorld, s: String) {
    assert!(false, "Not implemented");
}

#[when(expr = "I set rw string to {string}")]
fn i_set_rw_string_to(world: &mut BasicsWorld, s: String) {
    assert!(false, "Not implemented");
}

#[when(expr = "I set wo string to {string}")]
fn i_set_wo_string_to(world: &mut BasicsWorld, s: String) {
    assert!(false, "Not implemented");
}

#[given(expr = "the string attribute ro {string}")]
fn the_string_attribute_ro(world: &mut BasicsWorld, s: String) {
    assert!(false, "Not implemented");
}

#[given(expr = "the string attribute wo {string}")]
fn the_string_attribute_wo(world: &mut BasicsWorld, s: String) {
    assert!(false, "Not implemented");
}

#[then(expr = "the ro string value is {string}")]
fn the_ro_string_value_is(world: &mut BasicsWorld, s: String) {
    assert!(false, "Not implemented");
}
