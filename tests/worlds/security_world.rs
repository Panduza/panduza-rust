mod default;
mod logger;
mod writer;

use cucumber::Parameter;
use cucumber::{given, then, World};
use panduza::security::certificate::generate_cert_client_from_pem_with_san;
use panduza::security::certificate::CertParams;
use panduza::security::utils::{
    ensure_panduza_dirs, ensure_panduza_programdata_dirs, write_panduza_file,
};
use panduza::security::utils::{generate_and_store_client_credentials, PanduzaFileType};
use panduza::{
    reactor::ReactorOptions, AttributeBuilder, BooleanAttribute, BytesAttribute, Reactor,
    StringAttribute,
};
use panduza::{NotificationAttribute, StatusAttribute};
use std::time::Duration;
use std::{fmt::Debug, str::FromStr};

// --- TEST PARAMETERS ---
const PLAFORM_LOCALHOST: &str = "127.0.0.1";
const PLAFORM_PORT: u16 = 7447;
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

    // pub att_wo_counter: Option<SiAttribute>,
    pub att_wo_counter_reset: Option<BooleanAttribute>,

    pub topic_rw: Option<String>,
    pub topic_wo: Option<String>,
    pub topic_ro: Option<String>,

    pub toggle_start_time: Option<std::time::Instant>,
}

#[derive(Default)]
pub struct SiSubWorld {
    // pub att_rw: Option<SiAttribute>,
    // pub att_wo: Option<SiAttribute>,
    // pub att_ro: Option<SiAttribute>,
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
pub struct EnumSubWorld {
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
pub struct SecurityWorld {
    /// Reactor object
    ///
    pub r: Option<Reactor>,

    ///
    ///
    pub platform_status: Option<StatusAttribute>,

    ///
    ///
    pub platform_notifications: Option<NotificationAttribute>,

    ///
    ///
    // pub att_instance_status: Option<JsonAttribute>,

    /// Reactor sub world data
    ///
    pub reactor: ReactorSubWorld,

    /// Boolean sub world data
    ///
    pub boolean: BooleanSubWorld,

    /// String sub world data
    ///
    pub string: StringSubWorld,

    /// Si sub world data
    ///
    pub si: SiSubWorld,

    /// Enum sub world data
    ///
    pub r#enum: EnumSubWorld,

    /// Bytes sub world data
    ///
    pub bytes: BytesSubWorld,
}

impl Debug for SecurityWorld {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BooleanWorld")
            // .field("r", &self.r)
            .finish()
    }
}

///
///
#[given(expr = "a writer reactor connected on a test platform")]
async fn a_writer_connected_on_a_test_platform(world: &mut SecurityWorld) {
    let (root_ca_certificate, writer_certificate, writer_private_key) =
        generate_and_store_client_credentials("writer", vec!["127.0.0.1".into()], 730)
            .expect("failed to generate writer credentials");

    let options = ReactorOptions::new(
        PLAFORM_LOCALHOST,
        PLAFORM_PORT,
        root_ca_certificate.as_str(),
        writer_certificate.as_str(),
        writer_private_key.as_str(),
        Some(NAMESPACE),
    );

    // No additional setup required before connecting to the test platform
    println!("Connecting to {}:{}...", PLAFORM_LOCALHOST, PLAFORM_PORT);
    let reactor = panduza::new_reactor(options).await.unwrap();
    println!("ok");

    println!("Getting status attribute...");

    world.r = Some(reactor);

    world.platform_status = Some(world.r.as_ref().unwrap().new_status_attribute().await);

    println!("ok");

    // Get the notification attribute from the reactor and store it in the world
    println!("Getting notification attribute...");
    world.platform_notifications =
        Some(world.r.as_ref().unwrap().new_notification_attribute().await);
    println!("ok");

    world
        .platform_status
        .as_mut()
        .unwrap()
        .wait_for_all_instances_to_be_running(Duration::from_secs(15))
        .await
        .expect("Error while waiting for instance to be in running state");

    println!("reactor ready");
}

///
///
#[given(
    expr = "a default user connecting to the platform without getting notifications and status"
)]
async fn a_default_user_connecting_to_the_platform(world: &mut SecurityWorld) {
    let (root_ca_certificate, default_certificate, default_private_key) =
        generate_and_store_client_credentials("default", vec!["127.0.0.1".into()], 730)
            .expect("failed to generate default credentials");

    let options = ReactorOptions::new(
        PLAFORM_LOCALHOST,
        PLAFORM_PORT,
        root_ca_certificate.as_str(),
        default_certificate.as_str(),
        default_private_key.as_str(),
        Some(NAMESPACE),
    );

    // No additional setup required before connecting to the test platform
    println!("Connecting to {}:{}...", PLAFORM_LOCALHOST, PLAFORM_PORT);
    let reactor = panduza::new_reactor(options).await.unwrap();
    println!("ok");
}

///
///
#[given(expr = "a logger reactor connected on a test platform")]
async fn a_logger_connected_on_a_test_platform(world: &mut SecurityWorld) {
    let (root_ca_certificate, logger_certificate, logger_private_key) =
        generate_and_store_client_credentials("logger", vec!["127.0.0.1".into()], 730)
            .expect("failed to generate logger credentials");

    let options = ReactorOptions::new(
        PLAFORM_LOCALHOST,
        PLAFORM_PORT,
        root_ca_certificate.as_str(),
        logger_certificate.as_str(),
        logger_private_key.as_str(),
        Some(NAMESPACE),
    );

    // No additional setup required before connecting to the test platform
    println!("Connecting to {}:{}...", PLAFORM_LOCALHOST, PLAFORM_PORT);
    let reactor = panduza::new_reactor(options).await.unwrap();
    println!("ok");

    println!("Getting status attribute...");

    world.r = Some(reactor);

    world.platform_status = Some(world.r.as_ref().unwrap().new_status_attribute().await);

    println!("ok");

    // Get the notification attribute from the reactor and store it in the world
    println!("Getting notification attribute...");
    world.platform_notifications =
        Some(world.r.as_ref().unwrap().new_notification_attribute().await);
    println!("ok");

    world
        .platform_status
        .as_mut()
        .unwrap()
        .wait_for_all_instances_to_be_running(Duration::from_secs(15))
        .await
        .expect("Error while waiting for instance to be in running state");

    println!("reactor ready");
}
