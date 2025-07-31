use cucumber::{given, then, when};
use panduza::Reactor;

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

    match Reactor::builder()
        .ip("pok".to_string())
        .port(5894)
        .ca_certificate("zaza.pem".to_string())
        .connect_certificate("cert.pem".to_string())
        .connect_private_key("key.pem".to_string())
        .build()
        .await
    {
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
#[given(expr = "a reactor trying to connect to a platform with a wrong certificate")]
async fn a_reactor_trying_to_connect_to_a_platform_with_a_wrong_certificate(world: &mut BasicsWorld) {
    world.reactor.connection_failed = false;

    match Reactor::builder()
        .ip("127.0.0.1".to_string())
        .port(7447)
        .ca_certificate("credentials/certificates/root_ca_certificate.pem".to_string())
        .connect_certificate("credentials/certificates/bad_client_certificate.pem".to_string())
        .connect_private_key("credentials/keys/bad_client_private_key.pem".to_string())
        .build()
        .await
    {
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
#[given(expr = "a reactor trying to connect to a platform with an expired certificate")]
async fn a_reactor_trying_to_connect_to_a_platform_with_an_expired_certificate(world: &mut BasicsWorld) {
    world.reactor.connection_failed = false;

    match Reactor::builder()
        .ip("127.0.0.1".to_string())
        .port(7447)
        .ca_certificate("credentials/certificates/root_ca_certificate.pem".to_string())
        .connect_certificate("credentials/certificates/expired_client_certificate.pem".to_string())
        .connect_private_key("credentials/keys/expired_client_private_key.pem".to_string())
        .build()
        .await
    {
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
async fn the_reactor_find_function_is_called_with_the_previously_given_attribute_name(
    world: &mut BasicsWorld,
) {
    if let Some(attribute_name) = &world.reactor.att_name {
        let result = world.r.as_ref().unwrap().find_attribute(attribute_name).await;
        world.reactor.find_result = Some(result);
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
