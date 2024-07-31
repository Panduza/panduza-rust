use rumqttc::ClientError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AttributeError {
    #[error("error with mqtt connection")]
    Message(#[from] ClientError),
    #[error("error on mutex lock")]
    InternalMutex(String),
    #[error("weak pointer cannot be upgraded")]
    InternalPointerUpgrade,
    #[error("set request sent but no response received")]
    EnsureTimeout,
    #[error("we do not know what happened")]
    Unkonwn,
}
