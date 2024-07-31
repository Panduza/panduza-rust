mod pza_world;

use cucumber::World;
use futures::executor::block_on;
use pza_world::PanduzaWorld;



fn main() {


    block_on(
    PanduzaWorld::cucumber()
    .init_tracing()
    // .after(|_feature, _rule, _scenario, _ev, _world| {
    //     if let Some(w) = _world {
    //         if w.serial_stream.is_some() {
    //             tracing::info!("Closing serial connection");
    //             w.serial_stream.as_mut().unwrap().clear_line().unwrap();
    //             w.serial_stream = None;
    //         }
    //     }
    //     sleep(Duration::from_millis(1000)).boxed_local()
    // })
    .run("features/mqtt_connection.feature"));

}
