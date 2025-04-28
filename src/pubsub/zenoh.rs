use super::{Error, IncomingUpdate, Options, PubSubEvent};
use bytes::Bytes;
use zenoh::{open, sample::Sample, Config, Session};

/// Set client Options
///
fn client_config(options: Options) -> Config {
    let mut config = Config::from_file("config_zenoh.json5").unwrap();

    config.insert_json5(
        "connect/endpoints",
        &format!("quic/{}/{}", options.ip, options.port),
    );
    config.insert_json5(
        "transport/link/tls/root_ca_certificate",
        &options.ca_certificate,
    );

    config
}

/// Start a Zenoh session
///
pub async fn new_connection(options: Options) -> Result<(Operator, Listener), Error> {
    let config = client_config(options);

    let session = open(config).await.unwrap();

    Ok((
        Operator::new(session.clone()),
        Listener::new(session.clone()),
    ))
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

pub struct Listener {
    session: Session,
    topics: Vec<String>,
    // notify
    // subscribers: Arc<Mutex<Vec<subscribers
}

impl Listener {
    ///
    ///
    pub fn new(session: Session, topic: String) -> Self {
        let mut topics = Vec::new();
        topics.push(topic);
        Self { session, topics }
    }

    ///
    ///
    pub fn process_incoming_publish(packet: Sample) -> PubSubEvent {
        PubSubEvent::IncomingUpdate(IncomingUpdate {
            topic: packet.key_expr().to_string(),
            payload: Bytes::copy_from_slice(packet.payload().to_bytes().as_ref()),
        })
    }

    ///
    ///
    pub async fn poll(&mut self) -> Result<PubSubEvent, Error> {
        let mut subscribers = Vec::new();
        for topic in &self.topics {
            let subscriber = self.session.declare_subscriber(topic).await.unwrap();
            subscribers.push(subscriber);
        }

        for sub in subscribers {
            if let Ok(sample) = sub.recv_async().await {
                return Ok(Self::process_incoming_publish(sample));
            }
        }
        Err(Error::ListenError {
            cause: "No messages received".to_string(),
        })

        // let subscriber = self.session.declare_subscriber("*/**/").await.unwrap();

        // match subscriber.recv_async().await {
        //     Ok(sample) => Ok(Self::process_incoming_publish(sample)),
        //     Err(e) => Err(Error::ListenError {
        //         cause: e.to_string(),
        //     }),
        // }

        // loop {
        //     match self.session {
        //         Ok(event) => {
        //             // println!("*** Notification = {:?}", event);
        //             match event {
        //                 rumqttc::Event::Incoming(incoming) => match incoming {
        //                     rumqttc::Packet::Publish(packet) => {
        //                         return Ok(Self::process_incoming_publish(packet));
        //                     }
        //                     _ => {}
        //                 },
        //                 _ => {}
        //             }
        //         }
        //         Err(e) => {
        //             return Err(Error::ListenError {
        //                 cause: e.to_string(),
        //             })
        //         }
        //     }
        // }
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[derive(Clone)]
///
///
pub struct Operator {
    session: Session,
}

impl Operator {
    pub fn new(session: Session) -> Self {
        Self { session: session }
    }

    ///
    ///
    pub fn declare_publisher(&self, topic: String, retain: bool) -> Result<Publisher, Error> {
        Ok(Publisher::new(self.session.clone(), topic, retain))
    }

    ///
    ///
    pub fn declare_subscriber(&self) -> Result<Subscriber, Error> {
        Ok(Subscriber::new(self.session.clone()))
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

#[derive(Clone, Debug)]
///
///
pub struct Publisher {
    session: Session,
    topic: String,
    retain: bool,
}

impl Publisher {
    pub fn new(session: Session, topic: String, retain: bool) -> Self {
        Self {
            session: session,
            topic: topic,
            retain: retain,
        }
    }

    /// Publish the payload
    ///
    pub async fn publish(&self, payload: Bytes) -> Result<(), Error> {
        let publisher = self
            .session
            .declare_publisher(self.topic.clone())
            .await
            .unwrap();
        publisher
            .put(payload)
            .await
            .map_err(|e| Error::PublishError {
                topic: self.topic.clone(),
                cause: e.to_string(),
            })
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------

///
///
pub struct Subscriber {
    session: Session,
}

impl Subscriber {
    pub fn new(session: Session) -> Self {
        Self { session: session }
    }

    ///
    ///
    pub async fn subscribe(&mut self, topic: String) -> Result<(), Error> {
        self.session
            .declare_subscriber(topic.clone())
            .await
            .map(|_| ())
            .map_err(|e| Error::SubscribeError {
                topic: topic,
                cause: e.to_string(),
            })
    }
}

// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
// ----------------------------------------------------------------------------
