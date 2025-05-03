use cucumber::{given, then, when};

use super::BasicsWorld;



#[given(expr = "the string attribute rw {string}")]
fn the_string_attribute_rw(world: &mut BasicsWorld, s: String) {
    // Write code here that turns the phrase above into concrete actions
}

#[then(expr = "the rw string value is {string}")]
fn the_rw_string_value_is(world: &mut BasicsWorld, s: String) {
    // Write code here that turns the phrase above into concrete actions
}

#[when(expr = "I set rw string to {string}")]
fn i_set_rw_string_to(world: &mut BasicsWorld, s: String) {
    // Write code here that turns the phrase above into concrete actions
}

#[when(expr = "I set wo string to {string}")]
fn i_set_wo_string_to(world: &mut BasicsWorld, s: String) {
    // Write code here that turns the phrase above into concrete actions
}

#[given(expr = "the string attribute ro {string}")]
fn the_string_attribute_ro(world: &mut BasicsWorld, s: String) {
    // Write code here that turns the phrase above into concrete actions
}

#[given(expr = "the string attribute wo {string}")]
fn the_string_attribute_wo(world: &mut BasicsWorld, s: String) {
    // Write code here that turns the phrase above into concrete actions
}

#[then(expr = "the ro string value is {string}")]
fn the_ro_string_value_is(world: &mut BasicsWorld, s: String) {
    // Write code here that turns the phrase above into concrete actions
}
