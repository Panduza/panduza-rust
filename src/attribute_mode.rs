
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AttributeMode {
    #[serde(rename = "RO")]
    ReadOnly,
    #[serde(rename = "WO")]
    WriteOnly,
    #[serde(rename = "RW")]
    ReadWrite,
}
