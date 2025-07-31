use crate::attribute::notification::NotificationAttribute;
use crate::attribute::status::StatusAttribute;
use crate::attribute::structure::StructureAttribute;
use crate::attribute_builder::AttributeBuilder;
use crate::AttributeMetadata;
use crate::AttributeMode;
use zenoh::Session;

/// Builder module for creating Reactor instances
pub mod builder;
pub use builder::ReactorBuilder;

/// The reactor is the main structure that will handle the connections and the events
///
/// All the attribute and objects will be powered by the reactor
///
#[derive(Clone, Debug)]
pub struct Reactor {
    /// The zenoh session
    pub session: Session,

    /// The structure attribute
    pub structure: StructureAttribute,
}

/// PartialEq implementation that checks if session IDs are equal
impl PartialEq for Reactor {
    fn eq(&self, other: &Self) -> bool {
        self.session.zid() == other.session.zid()
    }
}

impl Reactor {
    /// Clone the inner structure attribute instantiated in the reactor
    pub async fn get_structure_attribute(&self) -> StructureAttribute {
        self.structure.clone()
    }
    /// Returns a ReactorBuilder for creating Reactor instances
    ///
    /// # Returns
    /// A new ReactorBuilder instance for configuring reactor creation
    pub fn builder() -> ReactorBuilder {
        ReactorBuilder::new()
    }

    /// Creates a new Reactor instance
    ///
    /// # Arguments
    /// * `session` - The Zenoh session to use for communication
    ///
    /// # Returns
    /// A new Reactor instance with an initialized StructureAttribute
    pub async fn new(session: Session) -> Self {
        // Create metadata for the structure attribute
        let metadata = AttributeMetadata::from_topic(
            "pza/_/structure".to_string(),
            Some("structure".to_string()),
            AttributeMode::ReadOnly,
        );

        // Create the structure attribute wrapping StdObjAttribute<StructureBuffer>
        let structure = StructureAttribute::new(session.clone(), metadata).await;

        Self { session, structure }
    }

    /// Create a new status attribute on "pza/_/status"
    ///
    /// # Returns
    /// A new StatusAttribute instance for monitoring platform status
    pub async fn new_status_attribute(&self) -> StatusAttribute {
        let metadata = AttributeMetadata::from_topic(
            "pza/_/status".to_string(),
            Some("status".to_string()),
            AttributeMode::ReadOnly,
        );

        StatusAttribute::new(self.session.clone(), metadata).await
    }

    /// Create a new notification attribute on "pza/_/notifications"
    ///
    /// # Returns
    /// A new NotificationAttribute instance for receiving platform notifications
    pub async fn new_notification_attribute(&self) -> NotificationAttribute {
        let metadata = AttributeMetadata::from_topic(
            "pza/_/notifications".to_string(),
            Some("notification".to_string()),
            AttributeMode::ReadOnly,
        );

        NotificationAttribute::new(self.session.clone(), metadata).await
    }

    /// Create an attribute builder finding metadata in structure attribute
    ///
    /// # Arguments
    /// * `pattern` - Pattern to search for in the structure attribute
    ///
    /// # Returns
    /// An AttributeBuilder instance that can be used to create specific attribute types
    pub async fn find_attribute<A: Into<String>>(&self, pattern: A) -> AttributeBuilder {
        let metadata = self.structure.find_attribute(pattern).await;
        AttributeBuilder::new(self.clone(), metadata)
    }
}
