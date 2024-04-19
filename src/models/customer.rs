use mongodb::bson::oid::ObjectId;
use mongodb::bson::DateTime;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CustomerDocument {
    /// Document Id
    pub _id: ObjectId,
    /// customer name
    pub firstName: String,

    pub lastName: Option<String>,
    /// createdAt
    pub createdAt: DateTime,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
pub struct Customer {
    /// Document Id
    pub _id: String,
    /// customer name
    pub firstName: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lastName: Option<String>,
    /// createdAt
    pub createdAt: String,
}

#[derive(Debug, Serialize, Deserialize, JsonSchema, Clone)]
pub struct CustomerInput {
    /// customer name
    pub firstName: String,
    pub lastName: Option<String>,
}