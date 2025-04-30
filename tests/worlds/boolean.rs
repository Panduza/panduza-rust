use cucumber::{given, then, when, World};
use panduza::{reactor::ReactorOptions, BooleanAttribute, JsonAttribute, Reactor};
use std::{fmt::Debug, str::FromStr};
use cucumber::Parameter;


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


#[derive(Default, World)]
pub struct BooleanWorld {

    pub r: Option<Reactor>,

    pub att_instance_status: Option<JsonAttribute>,

    pub att_rw : Option<BooleanAttribute>,
    pub att_wo : Option<BooleanAttribute>,

    pub topic_rw : Option<String>,
    pub topic_ro : Option<String>,
    pub topic_wo : Option<String>
}

impl Debug for BooleanWorld {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BooleanWorld")
        // .field("r", &self.r)
        .finish()
    }
}

///
/// 
#[given(expr = "a client connected to {string} on port {int}")]
async fn given_a_connected_client(world: &mut BooleanWorld, hostname: String, port: u16) {

    
    let options = ReactorOptions::new();
    let mut reactor = panduza::new_reactor(options).await.unwrap();


    world.r = Some(reactor);
}

///
/// 
#[given(expr = "the attribute wo {string}")]
async fn given_the_attribute_wo(world: &mut BooleanWorld, attribute_name: String) {
    world.topic_wo = Some(attribute_name.clone());

    let attribute: panduza::BooleanAttribute = world.r.as_ref().unwrap().find_attribute(attribute_name).expect_boolean()
    .await
    .unwrap();

    world.att_wo = Some(attribute);
}

///
/// 
#[given(expr = "the status attribute for the instance managing the wo attribute")]
async fn given_the_status_attribute(world: &mut BooleanWorld) {
    
    let instance_status_topic = world.att_wo.as_ref().unwrap().get_instance_status_topic();

    let attribute = world.r.as_ref().unwrap().build_instance_status_attribute(instance_status_topic).expect_json()
    .await.unwrap();

    world.att_instance_status = Some(attribute);
}

///
/// 
#[given(expr = "the attribute rw {string}")]
async fn given_the_attribute_rw(world: &mut BooleanWorld, attribute_name: String) {
    world.topic_rw = Some(attribute_name.clone());

    let attribute: panduza::BooleanAttribute = world.r.as_ref().unwrap().find_attribute(attribute_name).expect_boolean()
    .await
    .unwrap();

    world.att_rw = Some(attribute);
}

///
/// 
#[when(expr = "I set rw boolean to {boolean}")]
async fn i_set_rw_boolean(world: &mut BooleanWorld, value: Boolean) {
    world.att_rw.as_mut().unwrap().set(value.into_bool()).await.unwrap();
}

///
/// 
#[when(expr = "I set wo boolean to {boolean}")]
async fn i_set_wo_boolean(world: &mut BooleanWorld, value: Boolean) {
    world.att_wo.as_mut().unwrap().set(value.into_bool()).await.unwrap();
}

///
/// 
#[then(expr = "the rw boolean value is {boolean}")]
async fn the_rw_boolean_value_is(world: &mut BooleanWorld, expected_value: Boolean) {
    let read_value = world.att_rw.as_mut().unwrap().get().unwrap();
    assert_eq!(read_value, expected_value.into_bool(), "read '{:?}' != expected '{:?}'", read_value, expected_value.into_bool() );
}



#[then(expr = "the instance status attribute must be {string}")]
async fn the_instance_status_attribute_must_be(world: &mut BooleanWorld, s: String) {
    
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
