use rocket_okapi::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Responder, Debug, Deserialize, Serialize, JsonSchema)]
pub struct MessageResponse {
    /// This is a message from the server.
    pub message: String,
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct IndexResponse {
    pub message: String, 
    pub testing: String, 
    pub this_worked: bool
}
