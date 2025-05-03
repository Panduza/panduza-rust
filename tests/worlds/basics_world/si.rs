use cucumber::{given, then, when};
use super::BasicsWorld;

#[when(expr = "I set rw si to {float}")]
fn i_set_rw_si_to(world: &mut BasicsWorld, f: f32) {
    assert!(false, "Not implemented yet");
}

#[then(expr = "the rw si value is {float}")]
fn the_rw_si_value_is(world: &mut BasicsWorld, f: f32) {
    assert!(false, "Not implemented yet");
}

#[given(expr = "the si attribute rw {string}")]
fn the_si_attribute_rw(world: &mut BasicsWorld, s: String) {
    assert!(false, "Not implemented yet");
}

#[when(expr = "I set wo si to {float}")]
fn i_set_wo_si_to(world: &mut BasicsWorld, f: f32) {
    assert!(false, "Not implemented yet");
}

#[given(expr = "the si attribute wo {string}")]
fn the_si_attribute_wo(world: &mut BasicsWorld, attribute_name: String) {
    world.si.topic_rw = Some(attribute_name.clone());

    let attribute_builder = world.r.as_ref().unwrap().find_attribute(attribute_name).expect("Attribute not found");
    let attribute: panduza::BooleanAttribute = attribute_builder.ex().await.unwrap();

    world.si.att_rw = Some(attribute);
}

#[then(expr = "the ro si value is {float}")]
fn the_ro_si_value_is(world: &mut BasicsWorld, f: f32) {
    assert!(false, "Not implemented yet");
}

#[given(expr = "the si attribute ro {string}")]
fn the_si_attribute_ro(world: &mut BasicsWorld, s: String) {
    assert!(false, "Not implemented yet");
}
