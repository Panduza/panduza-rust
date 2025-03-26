use cucumber::{given, then, when};
use panduza::reactor::ReactorOptions;

use super::BasicsWorld;

///
///
#[then(expr = "the reactor is successfully connected to the platform")]
fn the_reactor_is_successfully_connected_to_the_platform(world: &mut BasicsWorld) {
    // Check if the reactor is connected to the platform
    assert!(
        world.r.is_some(),
        "Reactor is not connected to the platform"
    );
}

///
///
#[given(expr = "a reactor trying to connect to an invalid platform")]
async fn a_reactor_trying_to_connect_to_an_invalid_platform(world: &mut BasicsWorld) {
    world.reactor.connection_failed = false;

    let options = ReactorOptions::new("pok", 5894, "minica.pem");

    match panduza::new_reactor(options).await {
        Ok(reactor) => {
            world.r = Some(reactor);
        }
        Err(_) => {
            world.reactor.connection_failed = true;
            world.r = None;
        }
    }
}

///
///
#[given(expr = "an attribute name {string}")]
fn an_attribute_name(world: &mut BasicsWorld, s: String) {
    world.reactor.att_name = Some(s);
}

///
///
#[then(expr = "the reactor returned an error")]
fn the_reactor_returned_an_error(world: &mut BasicsWorld) {
    // Check if the reactor is connected to the platform
    assert!(world.r.is_none(), "Reactor is connected to the platform");
    assert!(
        world.reactor.connection_failed,
        "Expected flag_failed to be true, but it was false"
    );
}

///
///
#[then(expr = "the reactor must return a success")]
fn the_reactor_must_return_a_success(world: &mut BasicsWorld) {
    assert!(
        world.reactor.find_result.is_some(),
        "Expected find_result to be Some, but it was None"
    );
}

///
///
#[when(expr = "the reactor find function is called with the previously given attribute name")]
fn the_reactor_find_function_is_called_with_the_previously_given_attribute_name(
    world: &mut BasicsWorld,
) {
    if let Some(attribute_name) = &world.reactor.att_name {
        let result = world.r.as_ref().unwrap().find_attribute(attribute_name);
        world.reactor.find_result = result;
    } else {
        panic!("Attribute name is not set");
    }
}

///
///
#[then(expr = "the reactor must return a null value")]
fn the_reactor_must_return_a_null_value(world: &mut BasicsWorld) {
    assert!(
        world.reactor.find_result.is_none(),
        "Expected find_result to be None, but it was Some"
    );
}
