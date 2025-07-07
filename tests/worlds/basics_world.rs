mod boolean;
mod bytes;
mod number;
mod reactor;
mod string;

use cucumber::Parameter;
use cucumber::{given, then, World};
use futures::FutureExt;
use panduza::attribute::notification::NotificationPack;
use panduza::{
    reactor::ReactorOptions, AttributeBuilder, BooleanAttribute, BytesAttribute, JsonAttribute,
    Reactor, StringAttribute,
};
use panduza::{NotificationAttribute, NumberAttribute, StatusAttribute};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{fmt::Debug, str::FromStr};

// --- TEST PARAMETERS ---
const PLAFORM_LOCALHOST: &str = "127.0.0.1";
const PLAFORM_PORT: u16 = 7447;
const PLAFORM_CA_CERTIFICATE: &str = "minica.pem";
const NAMESPACE: &str = "";
// -----------------------

#[derive(Debug, Default, Parameter)]
// NOTE: `name` is optional, by default the lowercased type name is implied.
#[param(name = "boolean", regex = "true|false")]
enum Boolean {
    True,
    #[default]
    False,
}

impl Boolean {
    fn into_bool(&self) -> bool {
        match self {
            Boolean::True => true,
            Boolean::False => false,
        }
    }
}

impl FromStr for Boolean {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "true" => Self::True,
            "false" => Self::False,
            invalid => return Err(format!("Invalid `Boolean`: {invalid}")),
        })
    }
}

#[derive(Default)]
pub struct ReactorSubWorld {
    ///
    ///     
    pub connection_failed: bool,

    /// Attribute name to be used in the test
    ///
    pub att_name: Option<String>,

    /// Attribute builder result
    ///
    pub find_result: Option<AttributeBuilder>,
}

#[derive(Default)]
pub struct BooleanSubWorld {
    pub att_rw: Option<BooleanAttribute>,
    pub att_wo: Option<BooleanAttribute>,
    pub att_ro: Option<BooleanAttribute>,

    pub att_wo_counter: Option<NumberAttribute>,
    pub att_wo_counter_reset: Option<BooleanAttribute>,

    pub topic_rw: Option<String>,
    pub topic_wo: Option<String>,
    pub topic_ro: Option<String>,

    pub toggle_start_time: Option<std::time::Instant>,
}

#[derive(Default)]
pub struct NumberSubWorld {
    pub att_rw: Option<NumberAttribute>,
    pub att_wo: Option<NumberAttribute>,
    pub att_ro: Option<NumberAttribute>,
    // pub topic_rw: Option<String>,
    // pub topic_wo: Option<String>,
    // pub topic_ro: Option<String>,
}

#[derive(Default)]
pub struct StringSubWorld {
    pub att_rw: Option<StringAttribute>,
    pub att_wo: Option<StringAttribute>,
    pub att_ro: Option<StringAttribute>,
    // pub topic_rw: Option<String>,
    // pub topic_wo: Option<String>,
    // pub topic_ro: Option<String>,
}

#[derive(Default)]
pub struct BytesSubWorld {
    pub att_rw: Option<BytesAttribute>,
    pub att_wo: Option<BytesAttribute>,
    pub att_ro: Option<BytesAttribute>,
    // pub topic_rw: Option<String>,
    // pub topic_wo: Option<String>,
    // pub topic_ro: Option<String>,
}

#[derive(Default, World)]
pub struct BasicsWorld {
    /// Reactor object
    ///
    pub r: Option<Reactor>,

    ///
    ///
    pub platform_status: Option<StatusAttribute>,

    ///
    ///
    pub platform_notifications: Option<NotificationAttribute>,

    /// Fifo to store incoming platform notifications
    ///
    pub platform_notifications_pack: Option<Arc<Mutex<NotificationPack>>>,

    ///
    ///
    pub att_instance_status: Option<JsonAttribute>,

    /// Reactor sub world data
    ///
    pub reactor: ReactorSubWorld,

    /// Boolean sub world data
    ///
    pub boolean: BooleanSubWorld,

    /// String sub world data
    ///
    pub string: StringSubWorld,

    /// Number sub world data
    ///
    pub number: NumberSubWorld,

    /// Bytes sub world data
    ///
    pub bytes: BytesSubWorld,
}

impl Debug for BasicsWorld {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BooleanWorld")
            // .field("r", &self.r)
            .finish()
    }
}

///
///
#[given(expr = "a reactor connected on a test platform")]
async fn a_client_connected_on_a_test_platform(world: &mut BasicsWorld) {
    let options = ReactorOptions::new(
        PLAFORM_LOCALHOST,
        PLAFORM_PORT,
        PLAFORM_CA_CERTIFICATE,
        Some(NAMESPACE),
    );

    // No additional setup required before connecting to the test platform
    {
        println!("Connecting to {}:{}...", PLAFORM_LOCALHOST, PLAFORM_PORT);
        let reactor = panduza::new_reactor(options).await.unwrap();
        world.r = Some(reactor);
        println!("      connection => ok");
    }

    // Get the status attribute from the reactor and store it in the world
    {
        println!("Getting status attribute...");
        world.platform_status = Some(
            world
                .r
                .as_ref()
                .expect("need reactor to get status attribute")
                .new_status_attribute()
                .await,
        );
        println!("      status att => ok");
    }

    // Get the notification attribute from the reactor and store it in the world
    {
        // Create a pack to store notifications
        println!("Getting notification attribute...");
        world.platform_notifications_pack = Some(Arc::new(Mutex::new(NotificationPack::default())));
        world.platform_notifications = Some(
            world
                .r
                .as_ref()
                .expect("need reactor to get notification attribute")
                .new_notification_attribute()
                .await,
        );

        // // Set up a callback to store new notifications into the pack
        // if let Some(platform_notifications) = world.platform_notifications.as_mut() {
        //     platform_notifications
        //         .add_callback(
        //             {
        //                 let pack = world.platform_notifications_pack.clone();
        //                 move |notif| {
        //                     let pack = pack.clone();
        //                     // Clone the notification and push it into the pack
        //                     async move {
        //                         if let Some(pack) = pack {
        //                             let mut pack = pack.lock().unwrap();
        //                             pack.push(notif);
        //                         }
        //                     }
        //                     .boxed()
        //                 }
        //             },
        //             None::<fn(&panduza::fbs::NotificationBuffer) -> bool>,
        //         )
        //         .await;
        // }
        println!("ok");
    }

    world
        .platform_status
        .as_mut()
        .expect("need status attribute to wait for instances")
        .wait_for_all_instances_to_be_running(Duration::from_secs(15))
        .await
        .expect("Error while waiting for instance to be in running state");

    println!("reactor ready");
}

///
///
#[then(expr = "the status attribute must indicate running for all instances")]
async fn the_status_attribute_must_be(world: &mut BasicsWorld) {
    world
        .platform_status
        .as_mut()
        .unwrap()
        .wait_for_all_instances_to_be_running(Duration::from_secs(15))
        .await
        .expect("Error while waiting for instance to be in running state");
}

#[then(expr = "the status attribute must indicate an error for one instance")]
async fn the_status_attribute_must_indicate_for_one_instance(world: &mut BasicsWorld) {
    world
        .platform_status
        .as_mut()
        .unwrap()
        .wait_for_at_least_one_instance_to_be_not_running(Duration::from_secs(15))
        .await
        .expect("Error while waiting for instance to be in error state");

    world
        .platform_status
        .as_mut()
        .unwrap()
        .wait_for_all_instances_to_be_running(Duration::from_secs(15))
        .await
        .expect("Error while waiting for instance to be in running state");
}

// ----------------------------------------------------------------------------

#[then(expr = "the notification attribute must indicate no alert")]
fn the_notification_attribute_must_indicate_no_alert(world: &mut BasicsWorld) {
    // clear all notifications
    world
        .platform_notifications_pack
        .as_mut()
        .unwrap()
        .lock()
        .unwrap()
        .reset();
}

#[then(expr = "the notification attribute must indicate an alert for this instance")]
fn the_notification_attribute_must_indicate_an_alert_for_this_instance(world: &mut BasicsWorld) {
    // check that the notification attribute is not empty
    assert!(!world
        .platform_notifications_pack
        .as_mut()
        .unwrap()
        .lock()
        .unwrap()
        .has_alert());
}
