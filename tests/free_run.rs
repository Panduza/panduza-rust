use panduza::Reactor;
use std::time::Duration;
use tokio::time::sleep;

const PLAFORM_LOCALHOST: &str = "127.0.0.1";
const PLAFORM_PORT: u16 = 7447;

#[tokio::main]
async fn main() {
    println!("Running free run test");

    print!("Connecting to {}:{}...", PLAFORM_LOCALHOST, PLAFORM_PORT);

    let reactor = Reactor::builder()
        .with_platform_addr(PLAFORM_LOCALHOST.to_string())
        .with_platform_port(PLAFORM_PORT)
        .disable_security()
        .build()
        .await
        .expect("Failed to create reactor");

    // Print the structure as pretty JSON
    {
        let flat_guard = reactor.structure.flat.lock().await;
        println!("{}", serde_json::to_string_pretty(&*flat_guard).unwrap());
    }

    println!(" ok!");

    // Get the status attribute from the reactor and store it in the world

    print!("Getting status attribute...");
    let platform_status = reactor.new_status_attribute().await;
    println!(" ok!");

    // Get the notification attribute from the reactor and store it in the world

    // Create a pack to store notifications
    print!("Getting notification attribute...");
    let platform_notifications = reactor.new_notification_attribute().await;
    println!(" ok!");

    // Get the notification attribute from the reactor and store it in the world

    // Create a pack to store notifications

    println!(" ok!");
    platform_status
        .wait_for_all_instances_to_be_running(Duration::from_secs(15))
        .await
        .expect("Error while waiting for instance to be in running state");

    sleep(Duration::from_secs(5)).await;

    // Print the structure as pretty JSON after waiting
    {
        let flat_guard = reactor.structure.flat.lock().await;
        println!("{}", serde_json::to_string_pretty(&*flat_guard).unwrap());
    }
}
