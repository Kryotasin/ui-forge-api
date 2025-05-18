use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// Main FigmaFile model that represents a Figma design file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FigmaFile {
    // System-generated unique ID (separate from Figma's file_key)
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub file_id: Option<String>,
    
    // Figma's file key (from their API)
    pub file_key: String,
    pub name: Option<String>,
    pub last_modified: Option<String>,
    pub thumbnail_url: Option<String>,
    pub version: Option<String>,
    
    // Timestamps for our system
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime", default = "Utc::now")]
    pub created_at: DateTime<Utc>,
    
    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime", default = "Utc::now")]
    pub updated_at: DateTime<Utc>,
    
    // Structured data from Figma
    #[serde(default)]
    pub data: FigmaData,
    
    // Store the complete raw JSON data for flexibility
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_data: Option<serde_json::Value>,
}

/// Structured data extracted from Figma API
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FigmaData {
    // Document structure (pages, nodes, etc.)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document: Option<FigmaDocument>,
    
    // Components library
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub components: Vec<FigmaComponent>,
    
    // Component sets (groups of variants)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub component_sets: Vec<FigmaComponentSet>,
    
    // Styles (colors, text styles, effects, etc.)
    #[serde(default, skip_serializing_if = "HashMap::is_empty")]
    pub styles: HashMap<String, FigmaStyle>,
}

/// Document structure from Figma
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FigmaDocument {
    pub id: Option<String>,
    pub name: Option<String>,
    pub type_field: Option<String>,
    
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<FigmaNode>,
    
    // Other document properties
    #[serde(flatten)]
    pub properties: HashMap<String, serde_json::Value>,
}

/// Generic node in the Figma document
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FigmaNode {
    pub id: Option<String>,
    pub name: Option<String>,
    
    #[serde(rename = "type")]
    pub node_type: Option<String>,
    
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<FigmaNode>,
    
    // Other node properties
    #[serde(flatten)]
    pub properties: HashMap<String, serde_json::Value>,
}

/// Component in Figma's component library
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FigmaComponent {
    pub key: String,
    pub name: String,
    pub description: Option<String>,
    
    #[serde(rename = "componentSetId")]
    pub component_set_id: Option<String>,
    
    #[serde(rename = "nodeId")]
    pub node_id: String,
    
    // Other component properties
    #[serde(flatten)]
    pub properties: HashMap<String, serde_json::Value>,
}

/// Component set (group of variants)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FigmaComponentSet {
    pub key: String,
    pub name: String,
    pub description: Option<String>,
    
    #[serde(rename = "nodeId")]
    pub node_id: String,
    
    // Component IDs in this set
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub component_ids: Vec<String>,
    
    // Other component set properties
    #[serde(flatten)]
    pub properties: HashMap<String, serde_json::Value>,
}

/// Style (color, text, effect, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FigmaStyle {
    pub key: String,
    pub name: String,
    pub description: Option<String>,
    
    #[serde(rename = "styleType")]
    pub style_type: String,
    
    #[serde(rename = "nodeId")]
    pub node_id: Option<String>,
    
    // Other style properties
    #[serde(flatten)]
    pub properties: HashMap<String, serde_json::Value>,
}

impl FigmaFile {
    // Generate a unique file_id based on various inputs
    pub fn generate_file_id(&self) -> String {
        use sha2::{Sha256, Digest};
        
        // Current timestamp
        let now = Utc::now().timestamp_millis().to_string();
        
        // Combine file_key with timestamp and version for uniqueness
        let version = self.version.clone().unwrap_or_else(|| "unknown".to_string());
        let unique_string = format!("{}:{}:{}", self.file_key, now, version);
        
        // Create a hash of the combined string
        let mut hasher = Sha256::new();
        hasher.update(unique_string.as_bytes());
        let result = hasher.finalize();
        
        // Convert to hex string and truncate for reasonable length
        format!("fig_{}", hex::encode(&result[..8]))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FigmaFileQuery {
    pub file_key: String,
    pub version: Option<String>,
    pub ids: Option<String>,
    pub depth: Option<String>,
    pub geometry: Option<String>,
    pub plugin_data: Option<String>,
    pub branch_data: Option<String>,
}

// Response wrapper that matches your existing API pattern
#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub message: String,
    pub status: String,
    pub data: Option<T>,
}