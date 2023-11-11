use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Model {
    /// The model identifier, which can be referenced in the API endpoints.
    pub id: String,
    /// The Unix timestamp (in seconds) when the model was created.
    pub created: u32,
    /// The object type, which is always "model".
    pub object: String,
    /// The organization that owns the model.
    pub owned_by: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListModelResponse {
    // The object type, which is always "list".
    pub object: String,
    // Vector of the available models.
    pub data: Vec<Model>,
}
