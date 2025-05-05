use super::BasicsWorld;
use cucumber::{given, then, when};
use panduza::fbs::number::NumberBuffer;

#[given(expr = "the si attribute rw {string}")]
async fn the_si_attribute_rw(world: &mut BasicsWorld, attribute_name: String) {
    let attribute_builder = world
        .r
        .as_ref()
        .unwrap()
        .find_attribute(attribute_name)
        .expect("Attribute not found");
    let attribute = attribute_builder.expect_si().await.unwrap();

    world.si.att_rw = Some(attribute);
}

#[given(expr = "the si attribute wo {string}")]
async fn the_si_attribute_wo(world: &mut BasicsWorld, attribute_name: String) {
    let attribute_builder = world
        .r
        .as_ref()
        .unwrap()
        .find_attribute(attribute_name)
        .expect("Attribute not found");
    let attribute = attribute_builder.expect_si().await.unwrap();

    world.si.att_wo = Some(attribute);
}

#[given(expr = "the si attribute ro {string}")]
async fn the_si_attribute_ro(world: &mut BasicsWorld, attribute_name: String) {
    let attribute_builder = world
        .r
        .as_ref()
        .unwrap()
        .find_attribute(attribute_name)
        .expect("Attribute not found");
    let attribute = attribute_builder.expect_si().await.unwrap();

    world.si.att_ro = Some(attribute);
}

#[when(expr = "I set rw si to {float}")]
async fn i_set_rw_si_to(world: &mut BasicsWorld, f: f32) {
    world.si.att_rw.as_mut().unwrap().set(
        NumberBuffer::from_float_with_decimals(f, 2)
     ).await.unwrap();
}

#[when(expr = "I set wo si to {float}")]
async fn i_set_wo_si_to(world: &mut BasicsWorld, f: f32) {
    world.si.att_wo.as_mut().unwrap().set(
        NumberBuffer::from_float_with_decimals(f, 2)
     ).await.unwrap();
}


#[then(expr = "the rw si value is {float}")]
async fn the_rw_si_value_is(world: &mut BasicsWorld, f: f32) {
    let read_value = world.si.att_rw.as_mut().unwrap().get().unwrap();
    assert_eq!(read_value.try_into_f32().unwrap(), f, "read '{:?}' != expected '{:?}'", read_value, f );
}


#[then(expr = "the ro si value is {float}")]
async fn the_ro_si_value_is(world: &mut BasicsWorld, f: f32) {
    let timeout = std::time::Duration::from_secs(3);
    let start_time = std::time::Instant::now();
    let expected_value = NumberBuffer::from_float_with_decimals(f, 2);

    loop {
        let read_value = world.si.att_ro.as_ref().unwrap().get().unwrap();
        if read_value == expected_value {
            break;
        }
        if start_time.elapsed() >= timeout {
            panic!(
                "Timeout reached: read '{:?}' != expected '{:?}'",
                read_value,
                expected_value
            );
        }
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }
    let read_value = world.si.att_ro.as_ref().unwrap().get().unwrap();
    assert_eq!(read_value, expected_value, "read '{:?}' != expected '{:?}'", read_value, expected_value);
}
