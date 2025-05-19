use std::collections::HashMap;
use async_trait::async_trait;
use reqwest::Client;
use chrono::Utc;
use crate::models::figma::{
    FigmaFile, FigmaFileQuery, FigmaData, FigmaDocument, 
    FigmaComponent, FigmaComponentSet, FigmaStyle, FigmaNode
};
use crate::repositories::figma_repo::FigmaRepository;
use std::sync::Arc;

#[derive(Debug, thiserror::Error)]
pub enum FigmaServiceError {
    #[error("Figma API Error: {0}")]
    ApiError(String),
    
    #[error("Database Error: {0}")]
    DbError(#[from] mongodb::error::Error),
    
    #[error("Request Error: {0}")]
    RequestError(#[from] reqwest::Error),
    
    #[error("Missing required parameter: {0}")]
    MissingParam(String),
    
    #[error("JSON parsing error: {0}")]
    JsonError(#[from] serde_json::Error),
}

#[async_trait]
pub trait FigmaService: Send + Sync {
    async fn get_figma_file(&self, query: FigmaFileQuery, token: &str) -> Result<FigmaFile, FigmaServiceError>;
    async fn list_cached_files(&self, limit: Option<i64>) -> Result<Vec<FigmaFile>, FigmaServiceError>;
    async fn get_file_by_id(&self, file_id: &str) -> Result<Option<FigmaFile>, FigmaServiceError>;
}

pub struct FigmaServiceImpl {
    figma_repo: Arc<dyn FigmaRepository>,
    http_client: Client,
}

impl FigmaServiceImpl {
    pub fn new(figma_repo: Arc<dyn FigmaRepository>) -> Self {
        Self {
            figma_repo,
            http_client: Client::new(),
        }
    }
    
    // Extract structured data from Figma API response
    fn extract_figma_data(&self, raw_data: &serde_json::Value) -> Result<FigmaData, FigmaServiceError> {
        let mut figma_data = FigmaData::default();
        
        // Extract document structure
        if let Some(document) = raw_data.get("document") {
            if document.is_object() {
                figma_data.document = Some(serde_json::from_value(document.clone())?);
            }
        }
        
        // Extract components
        let mut components = Vec::new();
        if let Some(components_map) = raw_data.get("components").and_then(|v| v.as_object()) {
            for (key, component_value) in components_map {
                if let Some(mut component) = serde_json::from_value::<FigmaComponent>(component_value.clone()).ok() {
                    component.key = key.clone();
                    components.push(component);
                }
            }
        }
        figma_data.components = components;
        
        // Extract component sets
        let mut component_sets = Vec::new();
        if let Some(component_sets_map) = raw_data.get("componentSets").and_then(|v| v.as_object()) {
            for (key, set_value) in component_sets_map {
                if let Some(mut component_set) = serde_json::from_value::<FigmaComponentSet>(set_value.clone()).ok() {
                    component_set.key = key.clone();
                    component_sets.push(component_set);
                }
            }
        }
        figma_data.component_sets = component_sets;
        
        // Extract styles
        let mut styles = HashMap::new();
        if let Some(styles_map) = raw_data.get("styles").and_then(|v| v.as_object()) {
            for (key, style_value) in styles_map {
                if let Some(mut style) = serde_json::from_value::<FigmaStyle>(style_value.clone()).ok() {
                    style.key = key.clone();
                    styles.insert(key.clone(), style);
                }
            }
        }
        figma_data.styles = styles;
        
        Ok(figma_data)
    }
}

#[async_trait]
impl FigmaService for FigmaServiceImpl {
    async fn get_figma_file(&self, query: FigmaFileQuery, token: &str) -> Result<FigmaFile, FigmaServiceError> {
        // First check if we have it cached
        if let Ok(Some(file)) = self.figma_repo.get_file(&query.file_key).await {
            if query.version.is_none() || (query.version.is_some() && file.version == query.version) {
                // We have a valid cached version
                return Ok(file);
            }
        }
        
        // Not in cache or version mismatch, fetch from Figma API
        // Build the Figma API URL
        let url = format!("https://api.figma.com/v1/files/{}", &query.file_key);
        
        // Build query parameters
        let mut params = HashMap::new();
        if let Some(version) = &query.version {
            params.insert("version", version.clone());
        }
        if let Some(ids) = &query.ids {
            params.insert("ids", ids.clone());
        }
        if let Some(depth) = &query.depth {
            params.insert("depth", depth.clone());
        }
        if let Some(geometry) = &query.geometry {
            params.insert("geometry", geometry.clone());
        }
        if let Some(plugin_data) = &query.plugin_data {
            params.insert("plugin_data", plugin_data.clone());
        }
        if let Some(branch_data) = &query.branch_data {
            params.insert("branch_data", branch_data.clone());
        }

        // Make the API request to Figma
        let response = self.http_client
            .get(url)
            .query(&params)
            .header("X-Figma-Token", token)
            .send()
            .await?;

        // Check status code
        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await?;
            return Err(FigmaServiceError::ApiError(format!("{}: {}", status, error_text)));
        }

        // Parse JSON response
        let raw_data = response.json::<serde_json::Value>().await?;
        
        // Extract basic file info
        let name = raw_data.get("name").and_then(|v| v.as_str()).map(String::from);
        let last_modified = raw_data.get("lastModified").and_then(|v| v.as_str()).map(String::from);
        let version = raw_data.get("version").and_then(|v| v.as_str()).map(String::from);
        let thumbnail_url = raw_data.get("thumbnailUrl").and_then(|v| v.as_str()).map(String::from);
        
        // Extract structured data
        let figma_data = self.extract_figma_data(&raw_data)?;
        
        // Create file object
        let mut file = FigmaFile {
            file_id: None, // Will be generated during save
            file_key: query.file_key.clone(),
            name,
            last_modified,
            thumbnail_url,
            version,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            data: figma_data,
            raw_data: Some(raw_data),
        };
        
        // Save to cache and get assigned file_id
        match self.figma_repo.save_file(&mut file).await {
            Ok(file_id) => {
                println!("Cached Figma file with ID: {}", file_id);
                file.file_id = Some(file_id);
            },
            Err(e) => {
                println!("Warning: Failed to cache Figma file: {}", e);
                // Continue anyway, just couldn't cache
            }
        }
        
        Ok(file)
    }
    
    async fn get_file_by_id(&self, file_id: &str) -> Result<Option<FigmaFile>, FigmaServiceError> {
        let file = self.figma_repo.get_file_by_id(file_id).await?;
        Ok(file)
    }
    
    async fn list_cached_files(&self, limit: Option<i64>) -> Result<Vec<FigmaFile>, FigmaServiceError> {
        let files = self.figma_repo.list_files(limit).await?;
        Ok(files)
    }
}