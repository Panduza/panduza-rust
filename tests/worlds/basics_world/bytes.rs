use std::time::Duration;

use bytes::Bytes;
use cucumber::{given, then, when};

use super::BasicsWorld;

#[given(expr = "the bytes attribute rw {string}")]
async fn the_bytes_attribute_rw(world: &mut BasicsWorld, attribute_name: String) {
    let attribute_builder = world
        .r
        .as_ref()
        .unwrap()
        .find_attribute(attribute_name)
        .expect("Attribute not found");
    let attribute = attribute_builder.expect_bytes().await.unwrap();

    world.bytes.att_rw = Some(attribute);
}

#[given(expr = "the bytes attribute ro {string}")]
async fn the_bytes_attribute_ro(world: &mut BasicsWorld, attribute_name: String) {
    let attribute_builder = world
        .r
        .as_ref()
        .unwrap()
        .find_attribute(attribute_name)
        .expect("Attribute not found");
    let attribute = attribute_builder.expect_bytes().await.unwrap();

    world.bytes.att_ro = Some(attribute);
}

#[given(expr = "the bytes attribute wo {string}")]
async fn the_bytes_attribute_wo(world: &mut BasicsWorld, attribute_name: String) {
    let attribute_builder = world
        .r
        .as_ref()
        .unwrap()
        .find_attribute(attribute_name)
        .expect("Attribute not found");
    let attribute = attribute_builder.expect_bytes().await.unwrap();

    world.bytes.att_wo = Some(attribute);
}

#[when(expr = "I set rw bytes to {string}")]
async fn i_set_rw_bytes_to(world: &mut BasicsWorld, b: String) {
    let bytes: Bytes = Bytes::from(b);
    world
        .bytes
        .att_rw
        .as_mut()
        .unwrap()
        .set(bytes)
        .await
        .unwrap();
}

#[when(expr = "I set wo bytes to {string}")]
async fn i_set_wo_bytes_to(world: &mut BasicsWorld, b: String) {
    let bytes: Bytes = Bytes::from(b);
    world
        .bytes
        .att_wo
        .as_mut()
        .unwrap()
        .set(bytes)
        .await
        .unwrap();
}

#[then(expr = "the rw bytes value is {string}")]
async fn the_rw_bytes_value_is(world: &mut BasicsWorld, b: String) {
    let bytes: Bytes = Bytes::from(b);
    let read_value = world.bytes.att_rw.as_mut().unwrap().get().await.unwrap();
    assert_eq!(
        read_value.value().unwrap(),
        bytes,
        "read '{:?}' != expected '{:?}'",
        read_value,
        bytes
    );
}

#[then(expr = "the ro bytes value is {string}")]
async fn the_ro_bytes_value_is(world: &mut BasicsWorld, expected_value: String) {
    let bytes: Bytes = Bytes::from(expected_value.clone());
    world
        .bytes
        .att_ro
        .as_mut()
        .unwrap()
        .wait_for_value(bytes.clone(), Some(Duration::from_secs(5)))
        .await
        .unwrap();
    let read_value = world.bytes.att_ro.as_mut().unwrap().get().await.unwrap();
    assert_eq!(
        read_value.value().unwrap(),
        bytes,
        "read '{:?}' != expected '{:?}'",
        read_value,
        expected_value
    );
}
