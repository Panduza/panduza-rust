use core::time;

use super::PanduzaWorld;

use cucumber::given;
use cucumber::when;
use panduza::AttributeBoolean;
use panduza::ReactorSettings;
use panduza::SyncReactor;

use std::thread::sleep;

#[given("A broker is running")]
fn a_broker_is_running(world: &mut PanduzaWorld) {
    // world.start_the_test_broker();
}

#[when("I connect to the broker with the reactor")]
fn i_connect_to_the_broker_with_the_reactor(world: &mut PanduzaWorld) {
    
    let settings = ReactorSettings::new("localhost", 1883);
    let reactor = SyncReactor::new(settings);

    reactor.run_in_thread();

    // wait for connection
    
    sleep(time::Duration::from_secs(5));

    println!("-----------");

    reactor.scan_platforms();

    let pp: AttributeBoolean = reactor.create_attribute_from_topic("ooo").into();

    
    pp.set(true);


    sleep(time::Duration::from_secs(60));

}


