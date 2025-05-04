use cucumber::{given, then, when};

use super::BasicsWorld;

#[given(expr = "the enum attribute rw {string}")]
async fn the_enum_attribute_rw(world: &mut BasicsWorld, attribute_name: String) {
    let attribute_builder = world
        .r
        .as_ref()
        .unwrap()
        .find_attribute(attribute_name)
        .expect("Attribute not found");
    let attribute = attribute_builder.expect_enum().await.unwrap();

    world.r#enum.att_rw = Some(attribute);
}

#[given(expr = "the enum attribute ro {string}")]
async fn the_enum_attribute_ro(world: &mut BasicsWorld, attribute_name: String) {
    let attribute_builder = world
        .r
        .as_ref()
        .unwrap()
        .find_attribute(attribute_name)
        .expect("Attribute not found");
    let attribute = attribute_builder.expect_enum().await.unwrap();

    world.r#enum.att_ro = Some(attribute);
}

#[given(expr = "the enum attribute wo {string}")]
async fn the_enum_attribute_wo(world: &mut BasicsWorld, attribute_name: String) {
    let attribute_builder = world
        .r
        .as_ref()
        .unwrap()
        .find_attribute(attribute_name)
        .expect("Attribute not found");
    let attribute = attribute_builder.expect_enum().await.unwrap();

    world.r#enum.att_wo = Some(attribute);
}

#[when(expr = "I set rw enum to {string}")]
async fn i_set_rw_enum_to(world: &mut BasicsWorld, value: String) {
    world.r#enum.att_rw.as_mut().unwrap().set(value).await.unwrap();
}

#[when(expr = "I set wo enum to {string}")]
async fn i_set_wo_enum_to(world: &mut BasicsWorld, value: String) {
    world.r#enum.att_wo.as_mut().unwrap().set(value).await.unwrap();
}

#[then(expr = "the rw enum value is {string}")]
async fn the_rw_enum_value_is(world: &mut BasicsWorld, expected_value: String) {
    let read_value = world.r#enum.att_rw.as_mut().unwrap().get().unwrap();
    assert_eq!(
        read_value, expected_value,
        "read '{:?}' != expected '{:?}'",
        read_value, expected_value
    );
}

#[then(expr = "the ro enum value is {string}")]
async fn the_ro_enum_value_is(world: &mut BasicsWorld, expected_value: String) {
    let timeout = std::time::Duration::from_secs(3);
    let start_time = std::time::Instant::now();

    loop {
        let read_value = world.r#enum.att_ro.as_ref().unwrap().get().unwrap();
        if read_value == expected_value {
            break;
        }
        if start_time.elapsed() >= timeout {
            panic!(
                "Timeout reached: read '{:?}' != expected '{:?}'",
                read_value, expected_value
            );
        }
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }
    let read_value = world.r#enum.att_ro.as_ref().unwrap().get().unwrap();
    assert_eq!(
        read_value, expected_value,
        "read '{:?}' != expected '{:?}'",
        read_value, expected_value
    );
}
