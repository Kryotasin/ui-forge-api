use async_graphql::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct FigmaFile {
    pub file_key: String,
    pub message: String,
    pub status: String,
    pub data: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct FigmaData {
    pub document: Option<Document>,
    pub components: Option<serde_json::Value>,
    #[graphql(name = "componentSets")]
    pub component_sets: Option<serde_json::Value>,
    pub styles: Option<Styles>,
    pub name: Option<String>,
    pub version: Option<String>,
    pub role: Option<String>,
    #[graphql(name = "lastModified")]
    pub last_modified: Option<String>,
    #[graphql(name = "thumbnailUrl")]
    pub thumbnail_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct Document {
    pub id: String,
    pub name: String,
    #[graphql(name = "type")]
    pub node_type: String,
    pub children: Option<Vec<Document>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, SimpleObject)]
pub struct Styles {
    pub fills: Option<Vec<serde_json::Value>>,
    pub strokes: Option<Vec<serde_json::Value>>,
    pub effects: Option<Vec<serde_json::Value>>,
    pub grids: Option<Vec<serde_json::Value>>,
}

// For more granular control, you can create a custom type that maps the MongoDB document
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FigmaFileDocument {
    pub file_key: String,
    pub message: String,
    pub status: String,
    pub data: Option<serde_json::Value>,
}