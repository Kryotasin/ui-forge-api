use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateComponentRequest {
    pub component_type: String,  // "button", "card", etc.
    pub name: String,           // "CustomButton"
    pub config: HashMap<String, String>,  // style properties
    pub framework: String,      // "react"
    pub typescript: bool,
    pub package_name: String,   // "@mycompany/custom-button"
    pub version: String,        // "1.0.0"
}

#[derive(Debug, Serialize)]
pub struct CreateComponentResponse {
    pub success: bool,
    pub message: String,
    pub job_id: Option<String>,
}