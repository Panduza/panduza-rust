use cucumber::{given, then, when};
use panduza::{reactor::ReactorOptions, BooleanAttribute, JsonAttribute, Reactor};

use super::BasicsWorld;

///
/// 
#[then(expr = "the reactor is successfully connected to the platform")]
fn the_reactor_is_successfully_connected_to_the_platform(world: &mut BasicsWorld) {
    // Check if the reactor is connected to the platform
    assert!(world.r.is_some(), "Reactor is not connected to the platform");
}

///
/// 
#[given(expr = "a reactor trying to connect to an invalid platform")]
async fn a_reactor_trying_to_connect_to_an_invalid_platform(world: &mut BasicsWorld) {

    world.reactor.connection_failed = false;

    let options = ReactorOptions::new("pok", 5894);

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
#[then(expr = "the reactor returned an error")]
fn the_reactor_returned_an_error(world: &mut BasicsWorld) {
    // Check if the reactor is connected to the platform
    assert!(world.r.is_none(), "Reactor is connected to the platform");
    assert!(world.reactor.connection_failed, "Expected flag_failed to be true, but it was false");
}

