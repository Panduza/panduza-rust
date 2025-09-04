use cucumber::{given, then, when};
use zenoh::{session, Config, Session};

use super::{Boolean, SecurityWorld};
use zenoh::open;
use tokio::time::{sleep, Duration};
use panduza::security::utils::ensure_panduza_dirs;
use panduza::security::utils::ensure_panduza_programdata_dirs;

///
///
#[when(expr = "I try to set rw boolean to {boolean}")]
async fn i_try_to_set_rw_boolean(world: &mut SecurityWorld, value: Boolean) {
    let session = world.r.as_mut().unwrap().session.clone();
    let val = value.into_bool();
    let a = session.put("pza/tester/boolean/rw", vec![val as u8]).await.unwrap();
}

// ///
// ///
// #[when(expr = "I try to set rw boolean to {boolean}")]
// async fn i_try_to_set_rw_boolean(world: &mut SecurityWorld, value: Boolean) {
//     world
//         .boolean
//         .att_rw
//         .as_mut()
//         .unwrap()
//         .set(value.into_bool())
//         .await
//         .unwrap();
// }


///
///
#[when(expr = "I toglle rw boolean")]
async fn i_toglle_rw_boolean(world: &mut SecurityWorld) {

    let (root_key_dir, root_cert_dir) = ensure_panduza_programdata_dirs();
    let root_cert_dir_display = root_cert_dir.display().to_string().replace("\\", "/");
    let root_ca_certificate = format!("{}/root_ca_certificate.pem", root_cert_dir_display);

    let (user_key_dir, user_cert_dir) = ensure_panduza_dirs();
    
    let cert_filename = "writer_certificate.pem";
    let key_filename = "writer_private_key.pem";

    let client_certificate = user_cert_dir
        .join(&cert_filename)
        .display()
        .to_string()
        .replace("\\", "/");
    let client_key = user_key_dir
        .join(&key_filename)
        .display()
        .to_string()
        .replace("\\", "/");

    let conf = format!(
        r#"{{
            "mode": "client",
            "connect": {{
                "endpoints": ["quic/127.0.0.1:7447"]
            }},
            "transport": {{
                "link": {{
                    "tls": {{
                        "root_ca_certificate": "{root_ca_certificate}",
                        "enable_mtls": true,
                        "connect_private_key": "{client_key}",
                        "connect_certificate": "{client_certificate}"
                    }}
                }}, 
            }}
        }}"#,
    );

    // println!("Zenoh client config: {}", conf);
    let config = Config::from_json5(&conf).unwrap();
    let session = open(config).await.unwrap();


    let session_clone = session.clone();
    tokio::spawn(async move {
        for i in 0..10 {
            sleep(Duration::from_millis(100)).await;
            let val = (i % 2) as u8;
            session_clone.put("pza/tester/boolean/rw", vec![val]).await.unwrap();
        }
    });
}


///
///
#[then(expr = "I receive ten messages")]
async fn i_receive_messages(world: &mut SecurityWorld) {
    let subscriber = world.r.as_mut().unwrap().session.declare_subscriber("pza/tester/boolean/rw").await.unwrap();
    let mut count = 0;
    while let Ok(sample) = subscriber.recv_async().await {
        count += 1;
        if count >= 10 {
            break;
        }
    }
}