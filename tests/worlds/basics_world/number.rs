use super::BasicsWorld;
use cucumber::{given, then, when};
use std::time::Duration;

// ----------------------------------------------------------------------------

#[given(expr = "the number attribute rw {string}")]
async fn the_number_attribute_rw(world: &mut BasicsWorld, attribute_name: String) {
    let attribute_builder = world
        .r
        .as_ref()
        .unwrap()
        .find_attribute(attribute_name)
        .expect("Attribute not found");
    let attribute = attribute_builder.expect_number().await.unwrap();

    world.number.att_rw = Some(attribute);
}

// ----------------------------------------------------------------------------

#[given(expr = "the number attribute wo {string}")]
async fn the_number_attribute_wo(world: &mut BasicsWorld, attribute_name: String) {
    let attribute_builder = world
        .r
        .as_ref()
        .unwrap()
        .find_attribute(attribute_name)
        .expect("Attribute not found");
    let attribute = attribute_builder.expect_number().await.unwrap();

    world.number.att_wo = Some(attribute);
}

// ----------------------------------------------------------------------------

#[given(expr = "the number attribute ro {string}")]
async fn the_number_attribute_ro(world: &mut BasicsWorld, attribute_name: String) {
    let attribute_builder = world
        .r
        .as_ref()
        .unwrap()
        .find_attribute(attribute_name)
        .expect("Attribute not found");
    let attribute = attribute_builder.expect_number().await.unwrap();

    world.number.att_ro = Some(attribute);
}

// ----------------------------------------------------------------------------

#[when(expr = "I set rw number to {float}")]
async fn i_set_rw_number_to(world: &mut BasicsWorld, f: f32) {
    world
        .number
        .att_rw
        .as_mut()
        .unwrap()
        .set(f as f64)
        .await
        .unwrap();
}

// ----------------------------------------------------------------------------

#[when(expr = "I set wo number to {float}")]
async fn i_set_wo_number_to(world: &mut BasicsWorld, f: f32) {
    world
        .number
        .att_wo
        .as_mut()
        .unwrap()
        .set(f as f64)
        .await
        .unwrap();
}

// ----------------------------------------------------------------------------

#[then(expr = "the rw number value is {float}")]
async fn the_rw_number_value_is(world: &mut BasicsWorld, f: f32) {
    let read_value = world.number.att_rw.as_mut().unwrap().get().await.unwrap();
    assert_eq!(
        read_value.value().unwrap() as f32,
        f,
        "read '{:?}' != expected '{:?}'",
        read_value,
        f
    );
}

// ----------------------------------------------------------------------------

#[then(expr = "the ro number value is {float}")]
async fn the_ro_number_value_is(world: &mut BasicsWorld, expected_value: f32) {
    world
        .number
        .att_ro
        .as_mut()
        .unwrap()
        .wait_for_value(expected_value as f64, Some(Duration::from_secs(5)))
        .await
        .unwrap();
    let read_value = world.number.att_ro.as_mut().unwrap().get().await.unwrap();
    assert_eq!(
        read_value.value().unwrap(),
        expected_value as f64,
        "read '{:?}' != expected '{:?}'",
        read_value,
        expected_value
    );
}
