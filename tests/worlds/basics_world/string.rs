use cucumber::{given, then, when};
use std::time::Duration;

use super::BasicsWorld;

#[given(expr = "the string attribute rw {string}")]
async fn the_string_attribute_rw(world: &mut BasicsWorld, attribute_name: String) {
    let attribute_builder = world
        .r
        .as_ref()
        .unwrap()
        .find_attribute(attribute_name)
        .await;
    let attribute = attribute_builder.try_into_string().await.unwrap();

    world.string.att_rw = Some(attribute);
}

#[given(expr = "the string attribute ro {string}")]
async fn the_string_attribute_ro(world: &mut BasicsWorld, attribute_name: String) {
    let attribute_builder = world
        .r
        .as_ref()
        .unwrap()
        .find_attribute(attribute_name)
        .await;
    let attribute = attribute_builder.try_into_string().await.unwrap();

    world.string.att_ro = Some(attribute);
}

#[given(expr = "the string attribute wo {string}")]
async fn the_string_attribute_wo(world: &mut BasicsWorld, attribute_name: String) {
    let attribute_builder = world
        .r
        .as_ref()
        .unwrap()
        .find_attribute(attribute_name)
        .await;
    let attribute = attribute_builder.try_into_string().await.unwrap();

    world.string.att_wo = Some(attribute);
}

#[when(expr = "I set rw string to {string}")]
async fn i_set_rw_string_to(world: &mut BasicsWorld, s: String) {
    world.string.att_rw.as_mut().unwrap().set(s).await.unwrap();
}

#[when(expr = "I set wo string to {string}")]
async fn i_set_wo_string_to(world: &mut BasicsWorld, s: String) {
    world.string.att_wo.as_mut().unwrap().set(s).await.unwrap();
}

#[then(expr = "the rw string value is {string}")]
async fn the_rw_string_value_is(world: &mut BasicsWorld, s: String) {
    let read_value = world.string.att_rw.as_mut().unwrap().get().await.unwrap();
    assert_eq!(
        read_value.value(),
        Some(s.as_str()),
        "read '{:?}' != expected '{:?}'",
        read_value,
        s
    );
}

#[then(expr = "the ro string value is {string}")]
async fn the_ro_string_value_is(world: &mut BasicsWorld, expected_value: String) {
    let expected_value_clone = expected_value.clone();
    world
        .string
        .att_ro
        .as_mut()
        .unwrap()
        .wait_for_value(expected_value_clone, Some(Duration::from_secs(5)))
        .await
        .unwrap();
    let read_value = world.string.att_ro.as_mut().unwrap().get().await.unwrap();
    assert_eq!(
        read_value.value(),
        Some(expected_value.as_str()),
        "read '{:?}' != expected '{:?}'",
        read_value,
        expected_value
    );
}
