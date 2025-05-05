use cucumber::{given, then, when};

use super::BasicsWorld;

#[given(expr = "the string attribute rw {string}")]
async fn the_string_attribute_rw(world: &mut BasicsWorld, attribute_name: String) {
    let attribute_builder = world
        .r
        .as_ref()
        .unwrap()
        .find_attribute(attribute_name)
        .expect("Attribute not found");
    let attribute = attribute_builder.expect_string().await.unwrap();

    world.string.att_rw = Some(attribute);
}

#[given(expr = "the string attribute ro {string}")]
async fn the_string_attribute_ro(world: &mut BasicsWorld, attribute_name: String) {
    let attribute_builder = world
        .r
        .as_ref()
        .unwrap()
        .find_attribute(attribute_name)
        .expect("Attribute not found");
    let attribute = attribute_builder.expect_string().await.unwrap();

    world.string.att_ro = Some(attribute);
}

#[given(expr = "the string attribute wo {string}")]
async fn the_string_attribute_wo(world: &mut BasicsWorld, attribute_name: String) {
    let attribute_builder = world
        .r
        .as_ref()
        .unwrap()
        .find_attribute(attribute_name)
        .expect("Attribute not found");
    let attribute = attribute_builder.expect_string().await.unwrap();

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
    let read_value = world.string.att_rw.as_mut().unwrap().get().unwrap();
    assert_eq!(
        read_value, s,
        "read '{:?}' != expected '{:?}'",
        read_value, s
    );
}

#[then(expr = "the ro string value is {string}")]
async fn the_ro_string_value_is(world: &mut BasicsWorld, s: String) {
    let timeout = std::time::Duration::from_secs(3);
    let start_time = std::time::Instant::now();
    let expected_value = s;

    loop {
        let read_value = world.string.att_ro.as_ref().unwrap().get().unwrap();
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
    let read_value = world.string.att_ro.as_ref().unwrap().get().unwrap();
    assert_eq!(
        read_value, expected_value,
        "read '{:?}' != expected '{:?}'",
        read_value, expected_value
    );
}
