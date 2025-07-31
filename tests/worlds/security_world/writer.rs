use cucumber::{given, then, when};

use super::{Boolean, SecurityWorld};
use bytes::Bytes;
use tokio::time::Duration;
use zenoh::{open, Config};

///
///
#[given(expr = "the json structure attribute")]
async fn given_the_json_structure_attribute(world: &mut SecurityWorld) {
    let attribute_builder = world
        .r
        .as_ref()
        .unwrap()
        .find_attribute("pza/_/structure/cmd")
        .await;
    // let attribute: panduza::JsonAttribute = attribute_builder.expect_json().await.unwrap();
    // world.json.att_rw = Some(attribute);
}

///
///
#[when(expr = "I modify structure attribute")]
async fn i_modify_structure_attribute(world: &mut SecurityWorld) {
    let value = serde_json::to_string(&"test").unwrap();

    let conf = format!(
        r#"{{
            "mode": "client",
            "connect": {{
                "endpoints": ["quic/127.0.0.1:7447"]
            }},
            "transport": {{
                "link": {{
                    "tls": {{
                        "root_ca_certificate": "credentials/certificates/root_ca_certificate.pem",
                        "enable_mtls": true,
                        "connect_private_key": "credentials/keys/writer_private_key.pem",
                        "connect_certificate": "credentials/certificates/writer_certificate.pem"
                    }}
                }}, 
            }}
        }}"#,
    );

    // println!("Zenoh client config: {}", conf);
    let config = Config::from_json5(&conf).unwrap();
    let session = open(config).await.unwrap();

    let queryable = session
        .declare_queryable("pza/_/structure/att")
        .await
        .unwrap();
    tokio::task::spawn(async move {
        while let Ok(query) = queryable.recv_async().await {
            query
                .reply("pza/_/structure/att", value.clone())
                .await
                .unwrap();
        }
    });

    // let queryable = world
    //     .r
    //     .as_mut()
    //     .unwrap()
    //     .session
    //     .declare_queryable("pza/_/structure/att")
    //     .await
    //     .unwrap();
    // tokio::task::spawn(async move {
    //     while let Ok(query) = queryable.recv_async().await {
    //         query
    //             .reply("pza/_/structure/att", value.clone())
    //             .await
    //             .unwrap();
    //     }
    // });
}

///
///
#[then(expr = "the structure attribute is not modified")]
async fn the_structure_attribute_is_not_modified(world: &mut SecurityWorld) {
    // Récupérer la valeur actuelle de l'attribut structure
    let mut handler = world
        .r
        .as_mut()
        .unwrap()
        .session
        .get("pza/_/structure/att")
        .await
        .unwrap();
    let reply = handler.recv_async().await.unwrap();

    // println!("reply: {}", String::from_utf8_lossy(&reply.result().unwrap().payload().to_bytes()).to_string());

    let sample = reply.result().unwrap().payload().to_bytes();
    let received_value = String::from_utf8_lossy(&sample).to_string();
    println!("Received value: {}", received_value);

    assert_ne!(
        received_value,
        "test",
        "Structure attribute has been modified but it should not"
    );
}

///
///
#[given(expr = "the boolean attribute rw {string}")]
async fn given_the_attribute_rw(world: &mut SecurityWorld, attribute_name: String) {
    world.boolean.topic_rw = Some(attribute_name.clone());

    let attribute_builder = world
        .r
        .as_ref()
        .unwrap()
        .find_attribute(attribute_name)
        .await;
    let attribute: panduza::BooleanAttribute = attribute_builder.try_into_boolean().await.unwrap();

    world.boolean.att_rw = Some(attribute);
}

///
///
#[when(expr = "I set rw boolean to {boolean}")]
async fn i_set_rw_boolean(world: &mut SecurityWorld, value: Boolean) {
    world
        .boolean
        .att_rw
        .as_mut()
        .unwrap()
        .set(value.into_bool())
        .await
        .unwrap();
}

///
///
#[then(expr = "the rw boolean value is {boolean}")]
async fn the_rw_boolean_value_is(world: &mut SecurityWorld, expected_value: Boolean) {
    let read_value = world.boolean.att_rw.as_mut().unwrap().get().await.unwrap();
    assert_eq!(
        read_value.value().unwrap(),
        expected_value.into_bool(),
        "read '{:?}' != expected '{:?}'",
        read_value,
        expected_value.into_bool()
    );
}

///
///
#[given(expr = "the writer boolean attribute wo {string}")]
async fn given_the_writer_attribute_wo(world: &mut SecurityWorld, attribute_name: String) {
    world.boolean.topic_wo = Some(attribute_name.clone());

    let attribute_builder = world
        .r
        .as_ref()
        .unwrap()
        .find_attribute(attribute_name)
        .await;
    let attribute =
        tokio::time::timeout(Duration::from_secs(5), attribute_builder.try_into_boolean())
            .await
            .unwrap()
            .unwrap();

    world.boolean.att_wo = Some(attribute);
}

///
///
#[given(expr = "the writer boolean attribute ro {string}")]
async fn given_the_writer_attribute_ro(world: &mut SecurityWorld, attribute_name: String) {
    world.boolean.topic_ro = Some(attribute_name.clone());

    let attribute_builder = world
        .r
        .as_ref()
        .unwrap()
        .find_attribute(attribute_name)
        .await;
    let attribute: panduza::BooleanAttribute = attribute_builder.try_into_boolean().await.unwrap();

    world.boolean.att_ro = Some(attribute);
}

///
///
#[when(expr = "I set writer wo boolean to {boolean}")]
async fn i_set_writer_wo_boolean(world: &mut SecurityWorld, value: Boolean) {
    world
        .boolean
        .att_wo
        .as_mut()
        .unwrap()
        .set(value.into_bool())
        .await
        .unwrap();
}

///
///
#[then(expr = "the ro writer boolean value is {boolean}")]
async fn the_ro_writer_boolean_value_is(world: &mut SecurityWorld, expected_value: Boolean) {
    world
        .boolean
        .att_ro
        .as_mut()
        .unwrap()
        .wait_for_value(expected_value.into_bool(), Some(Duration::from_secs(5)))
        .await;
    let read_value = world.boolean.att_ro.as_mut().unwrap().get().await.unwrap();
    assert_eq!(
        read_value.value().unwrap(),
        expected_value.into_bool(),
        "read '{:?}' != expected '{:?}'",
        read_value,
        expected_value.into_bool()
    );
}
