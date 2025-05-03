use cucumber::{given, then, when};

use super::{BasicsWorld, Boolean};

///
/// 
#[given(expr = "the boolean attribute rw {string}")]
async fn given_the_attribute_rw(world: &mut BasicsWorld, attribute_name: String) {
    world.boolean.topic_rw = Some(attribute_name.clone());

    let attribute_builder = world.r.as_ref().unwrap().find_attribute(attribute_name).expect("Attribute not found");
    let attribute: panduza::BooleanAttribute = attribute_builder.expect_boolean().await.unwrap();

    world.boolean.att_rw = Some(attribute);
}

///
/// 
#[given(expr = "the boolean attribute wo {string}")]
async fn given_the_attribute_wo(world: &mut BasicsWorld, attribute_name: String) {
    world.boolean.topic_wo = Some(attribute_name.clone());

    let attribute_builder = world.r.as_ref().unwrap().find_attribute(attribute_name).expect("Attribute not found");
    let attribute: panduza::BooleanAttribute = attribute_builder.expect_boolean().await.unwrap();

    world.boolean.att_wo = Some(attribute);
}

///
/// 
#[given(expr = "the boolean attribute ro {string}")]
async fn given_the_attribute_ro(world: &mut BasicsWorld, attribute_name: String) {
    world.boolean.topic_ro = Some(attribute_name.clone());

    let attribute_builder = world.r.as_ref().unwrap().find_attribute(attribute_name).expect("Attribute not found");
    let attribute: panduza::BooleanAttribute = attribute_builder.expect_boolean().await.unwrap();

    world.boolean.att_ro = Some(attribute);
}

///
/// 
#[then(expr = "the rw boolean value is {boolean}")]
async fn the_rw_boolean_value_is(world: &mut BasicsWorld, expected_value: Boolean) {
    let read_value = world.boolean.att_rw.as_mut().unwrap().get().unwrap();
    assert_eq!(read_value, expected_value.into_bool(), "read '{:?}' != expected '{:?}'", read_value, expected_value.into_bool() );
}

///
/// 
#[then(expr = "the ro boolean value is {boolean}")]
async fn the_ro_boolean_value_is(world: &mut BasicsWorld, expected_value: Boolean) {
    let timeout = std::time::Duration::from_secs(3);
    let start_time = std::time::Instant::now();

    loop {
        let read_value = world.boolean.att_ro.as_ref().unwrap().get().unwrap();
        if read_value == expected_value.into_bool() {
            break;
        }
        if start_time.elapsed() >= timeout {
            panic!(
                "Timeout reached: read '{:?}' != expected '{:?}'",
                read_value,
                expected_value.into_bool()
            );
        }
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }
    let read_value = world.boolean.att_ro.as_ref().unwrap().get().unwrap();
    assert_eq!(read_value, expected_value.into_bool(), "read '{:?}' != expected '{:?}'", read_value, expected_value.into_bool());
}

///
/// 
#[when(expr = "I set rw boolean to {boolean}")]
async fn i_set_rw_boolean(world: &mut BasicsWorld, value: Boolean) {
    world.boolean.att_rw.as_mut().unwrap().set(value.into_bool()).await.unwrap();
}

///
/// 
#[when(expr = "I set wo boolean to {boolean}")]
async fn i_set_wo_boolean(world: &mut BasicsWorld, value: Boolean) {
    world.boolean.att_wo.as_mut().unwrap().set(value.into_bool()).await.unwrap();
}
