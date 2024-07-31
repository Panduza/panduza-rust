use std::{thread::sleep, time};

use panduza::syncv::Reactor;
use panduza::ReactorSettings;

fn main() {
    let settings = ReactorSettings::new("localhost", 1883);
    let reactor = Reactor::new(settings);

    reactor.run_in_thread();

    // wait for connection

    // sleep(time::Duration::from_secs(5));

    println!("-----------");

    // reactor.scan_platforms();

    let pp = reactor.attribute_from_topic("ooo").unwrap().into_att_bool();

    pp.set(true);

    sleep(time::Duration::from_secs(60));
}
