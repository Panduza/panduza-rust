use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// States of the main Interface FSM
///
#[derive(Default, Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum InstanceState {
    Booting,
    Connecting,
    Initializating,
    Running,
    Warning,
    Error,
    Cleaning,
    Stopping,
    #[default]
    Undefined,
}

impl Display for InstanceState {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            InstanceState::Booting => write!(f, "Booting"),
            InstanceState::Connecting => write!(f, "Connecting"),
            InstanceState::Initializating => write!(f, "Initializating"),
            InstanceState::Running => write!(f, "Running"),
            InstanceState::Error => write!(f, "Error"),
            InstanceState::Warning => write!(f, "Warning"),
            InstanceState::Cleaning => write!(f, "Cleaning"),
            InstanceState::Stopping => write!(f, "Stopping"),
            InstanceState::Undefined => write!(f, "Undefined"),
        }
    }
}

impl From<InstanceState> for u16 {
    fn from(state: InstanceState) -> Self {
        match state {
            InstanceState::Undefined => 0,
            InstanceState::Booting => 1,
            InstanceState::Connecting => 2,
            InstanceState::Initializating => 3,
            InstanceState::Running => 4,
            InstanceState::Warning => 5,
            InstanceState::Error => 6,
            InstanceState::Cleaning => 7,
            InstanceState::Stopping => 8,
        }
    }
}
