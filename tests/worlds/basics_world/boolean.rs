use std::time::Duration;

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
    let attribute = tokio::time::timeout(
        Duration::from_secs(5),
        attribute_builder.expect_boolean()
    ).await.unwrap().unwrap();

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
    world.boolean.att_ro.as_mut().unwrap().wait_for_value(expected_value.into_bool()).await.unwrap();
    let read_value = world.boolean.att_ro.as_mut().unwrap().get().unwrap();
    assert_eq!(read_value, expected_value.into_bool(), "read '{:?}' != expected '{:?}'", read_value, expected_value.into_bool());
}

#[given(expr = "the number attribute wo_counter {string}")]
async fn the_boolean_attribute_wo_counter(world: &mut BasicsWorld, s: String) {
    
    let attribute_builder = world.r.as_ref().unwrap().find_attribute(s).expect("Attribute not found");
    let attribute: panduza::SiAttribute = attribute_builder.expect_si().await.unwrap();

    world.boolean.att_wo_counter = Some(attribute);
}

#[given(expr = "the boolean attribute wo_counter_reset {string}")]
async fn the_boolean_attribute_wo_counter_reset(world: &mut BasicsWorld, s: String) {

    let attribute_builder = world.r.as_ref().unwrap().find_attribute(s).expect("Attribute not found");
    let attribute = attribute_builder.expect_boolean().await.unwrap();

    world.boolean.att_wo_counter_reset = Some(attribute);
}

#[given(expr = "the counter is reseted")]
async fn the_counter_is_reseted(world: &mut BasicsWorld, ) {
    world.boolean.att_wo_counter_reset.as_mut().unwrap().set(true).await.unwrap();
}


#[then(expr = "the counter attribute must indicate {int}")]
async fn the_counter_attribute_must_indicate(world: &mut BasicsWorld, expected_count: i32) {

    // // Sleep for 1 second to allow the counter to update
    // tokio::time::sleep(Duration::from_secs(2)).await;
    // // Get the counter value
    // let counter_value = world.boolean.att_wo_counter.as_ref().expect("att_wo_counter is not set").get().expect("Failed to get counter value");
    
    // // Convert to i32 for comparison
    // let counter_value_i32 = counter_value.try_into_f32().expect("Failed to convert counter value to f32") as i32;

    // Wait until counter value matches expected count
    let mut counter_value_i32 = 0;
    while counter_value_i32 != expected_count {
        // Get the counter value
        let counter_value = world
            .boolean
            .att_wo_counter
            .as_ref()
            .expect("att_wo_counter is not set")
            .get()
            .expect("Failed to get counter value");

        // Convert to i32 for comparison
        counter_value_i32 = counter_value
            .try_into_f32()
            .expect("Failed to convert counter value to f32") as i32;

        // Small delay to avoid busy waiting
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
    
    // Verify the counter value
    assert_eq!(
        counter_value_i32,
        expected_count,
        "Counter value '{}' doesn't match expected '{}'",
        counter_value_i32,
        expected_count
    );
}

#[when(expr = "wo boolean is toggled {int} times")]
async fn wo_boolean_is_toggled_times(world: &mut BasicsWorld, times: i32) {
    // Initialize to false
    world.boolean.att_wo.as_mut().unwrap().set(false).await.unwrap();

    // Toggle the boolean attribute the specified number of times
    let mut value = false;
    for _ in 0..times-1 { // -1 because we set it to false before
        value = !value;
        world.boolean.att_wo.as_mut().unwrap().set(value).await.unwrap();
    }
}
