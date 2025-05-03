mod reactor;

use cucumber::{given, then, when, World};
use panduza::{reactor::ReactorOptions, AttributeBuilder, BooleanAttribute, JsonAttribute, Reactor};
use std::{fmt::Debug, str::FromStr};
use cucumber::Parameter;

// --- TEST PARAMETERS ---
const PLAFORM_LOCALHOST: &str = "localhost";
const PLAFORM_PORT: u16 = 1883;
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
    fn into_bool(&self) -> bool  {
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
    pub topic_rw: Option<String>,
    pub topic_wo: Option<String>,
}

#[derive(Default, World)]
pub struct BasicsWorld {
    /// Reactor object
    /// 
    pub r: Option<Reactor>,

    pub att_instance_status: Option<JsonAttribute>,


    /// Reactor sub world data
    /// 
    pub reactor: ReactorSubWorld,

    /// Boolean sub world data
    /// 
    pub boolean: BooleanSubWorld,
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
    let options = ReactorOptions::new(PLAFORM_LOCALHOST, PLAFORM_PORT);
    let reactor = panduza::new_reactor(options).await.unwrap();

    world.r = Some(reactor);
}

///
/// 
#[given(expr = "the boolean attribute rw {string}")]
async fn given_the_attribute_rw(world: &mut BasicsWorld, attribute_name: String) {
    world.boolean.topic_rw = Some(attribute_name.clone());

    let attribute_builder = world.r.as_ref().unwrap().find_attribute(attribute_name).expect("Attribute not found");
    let attribute: panduza::BooleanAttribute = attribute_builder.expect_boolean().await.unwrap();

    world.boolean.att_rw = Some(attribute);
}

///
/// 
#[given(expr = "the boolean attribute wo {string}")]
async fn given_the_attribute_wo(world: &mut BasicsWorld, attribute_name: String) {
    world.boolean.topic_wo = Some(attribute_name.clone());

    let attribute_builder = world.r.as_ref().unwrap().find_attribute(attribute_name).expect("Attribute not found");
    let attribute: panduza::BooleanAttribute = attribute_builder.expect_boolean().await.unwrap();

    world.boolean.att_wo = Some(attribute);
}

///
/// 
#[given(expr = "the status attribute for the instance managing the wo attribute")]
async fn given_the_status_attribute(world: &mut BasicsWorld) {
    
    let instance_status_topic = world.boolean.att_wo.as_ref().unwrap().get_instance_status_topic();

    let attribute = world.r.as_ref().unwrap().build_instance_status_attribute(instance_status_topic).expect_json()
    .await.unwrap();

    world.att_instance_status = Some(attribute);
}

///
/// 
#[when(expr = "I set rw boolean to {boolean}")]
async fn i_set_rw_boolean(world: &mut BasicsWorld, value: Boolean) {
    world.boolean.att_rw.as_mut().unwrap().set(value.into_bool()).await.unwrap();
}

///
/// 
#[when(expr = "I set wo boolean to {boolean}")]
async fn i_set_wo_boolean(world: &mut BasicsWorld, value: Boolean) {
    world.boolean.att_wo.as_mut().unwrap().set(value.into_bool()).await.unwrap();
}

///
/// 
#[then(expr = "the rw boolean value is {boolean}")]
async fn the_rw_boolean_value_is(world: &mut BasicsWorld, expected_value: Boolean) {
    let read_value = world.boolean.att_rw.as_mut().unwrap().get().unwrap();
    assert_eq!(read_value, expected_value.into_bool(), "read '{:?}' != expected '{:?}'", read_value, expected_value.into_bool() );
}

///
///
#[then(expr = "the instance status attribute must be {string}")]
async fn the_instance_status_attribute_must_be(world: &mut BasicsWorld, s: String) {
    
    let status = world.att_instance_status.as_mut().unwrap();

    let data = status.get().unwrap();
    let state = data.get("state").and_then(|v| v.as_str());
    if state != Some(s.as_str())  {
        status.update_notifier().notified().await;

        let data = status.get().unwrap();
        let state = data.get("state").and_then(|v| v.as_str());
        assert_eq!(state, Some(s.as_str()), "Expected 'state' to be '{:?}', but got '{:?}'", s, state);
    }

}

