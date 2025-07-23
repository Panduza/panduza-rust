use crate::attribute::bytes::BytesAttribute;
use crate::attribute::notification::NotificationAttribute;
use crate::attribute::number::NumberAttribute;
use crate::attribute::status::StatusAttribute;
use crate::attribute::AttributeError;
use crate::attribute_metadata::AttributeMetadata;
use crate::reactor::Reactor;
use crate::BooleanAttribute;
use crate::StringAttribute;

/// Macro to create a metadata not found error
macro_rules! meta_data_not_found {
    ($msg:expr) => {
        AttributeError::NotFound(format!("metadata - {}", $msg))
    };
}

#[derive(Clone)]
/// Metadata for an attribute
///
pub struct AttributeBuilder {
    ///
    ///
    reactor: Reactor,

    ///
    ///
    metadata: Option<AttributeMetadata>,
}

impl AttributeBuilder {
    ///
    ///
    pub fn new(reactor: Reactor, metadata: Option<AttributeMetadata>) -> Self {
        Self {
            reactor: reactor,
            metadata: metadata,
        }
    }

    // ------------------------------------------------------------------------

    /// BOOLEAN
    ///
    pub async fn try_into_boolean(self) -> Result<BooleanAttribute, AttributeError> {
        // Check if metadata is available
        let metadata = self
            .metadata
            .ok_or_else(|| meta_data_not_found!("boolean"))?;

        // Check if the attribute is a boolean type
        if metadata.r#type != "boolean" {
            return Err(AttributeError::InvalidType(
                "boolean".to_string(),
                metadata.r#type.clone(),
            ));
        }

        // Create and return the BooleanAttribute
        Ok(BooleanAttribute::new(self.reactor.session, metadata).await)
    }

    // ------------------------------------------------------------------------

    /// NUMBER
    ///
    pub async fn try_into_number(self) -> Result<NumberAttribute, AttributeError> {
        let metadata = self
            .metadata
            .ok_or_else(|| meta_data_not_found!("number"))?;

        if metadata.r#type != "number" {
            return Err(AttributeError::InvalidType(
                "number".to_string(),
                metadata.r#type.clone(),
            ));
        }

        Ok(NumberAttribute::new(self.reactor.session, metadata).await)
    }

    // ------------------------------------------------------------------------

    /// STRING
    ///
    pub async fn try_into_string(self) -> Result<StringAttribute, AttributeError> {
        let metadata = self
            .metadata
            .ok_or_else(|| meta_data_not_found!("string"))?;

        if metadata.r#type != "string" {
            return Err(AttributeError::InvalidType(
                "string".to_string(),
                metadata.r#type.clone(),
            ));
        }

        Ok(StringAttribute::new(self.reactor.session, metadata).await)
    }

    // ------------------------------------------------------------------------

    /// BYTES
    ///
    pub async fn try_into_bytes(self) -> Result<BytesAttribute, AttributeError> {
        let metadata = self
            .metadata
            .ok_or_else(|| meta_data_not_found!("bytes"))?;

        if metadata.r#type != "bytes" {
            return Err(AttributeError::InvalidType(
                "bytes".to_string(),
                metadata.r#type.clone(),
            ));
        }

        Ok(BytesAttribute::new(self.reactor.session, metadata).await)
    }

    // ------------------------------------------------------------------------

    /// STATUS
    ///
    pub async fn try_into_status(self) -> Result<StatusAttribute, AttributeError> {
        let metadata = self
            .metadata
            .ok_or_else(|| meta_data_not_found!("status"))?;

        if metadata.r#type != "status" {
            return Err(AttributeError::InvalidType(
                "status".to_string(),
                metadata.r#type.clone(),
            ));
        }

        Ok(StatusAttribute::new(self.reactor.session, metadata).await)
    }

    // ------------------------------------------------------------------------

    /// NOTIFICATION
    ///
    pub async fn try_into_notification(self) -> Result<NotificationAttribute, AttributeError> {
        let metadata = self
            .metadata
            .ok_or_else(|| meta_data_not_found!("notification"))?;

        if metadata.r#type != "notification" {
            return Err(AttributeError::InvalidType(
                "notification".to_string(),
                metadata.r#type.clone(),
            ));
        }

        Ok(NotificationAttribute::new(self.reactor.session, metadata).await)
    }

    // ------------------------------------------------------------------------
}
