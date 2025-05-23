use async_graphql::*;
use mongodb::bson::doc;
use crate::db::mongo::MongoDb;
use crate::graphql::types::{FigmaFile, FigmaFileDocument, FigmaData, Document, Styles};

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn figma_file_by_key(
        &self,
        ctx: &Context<'_>,
        file_key: String,
    ) -> Result<Option<FigmaFile>> {
        let db = ctx.data::<MongoDb>()?;
        
        let filter = doc! { "file_key": &file_key };
        
        // First get the raw document
        let document: Option<FigmaFileDocument> = db
            .get_document_from_collection("figma_file", filter)
            .await
            .map_err(|e| Error::new(format!("Database error: {}", e)))?;
        
        // Convert to our GraphQL type
        match document {
            Some(doc) => {
                // Parse the data field if it exists
                let figma_data = if let Some(data_value) = &doc.data {
                    // Extract specific fields from the JSON
                    let document = data_value.get("document").cloned();
                    let components = data_value.get("components").cloned();
                    let component_sets = data_value.get("componentSets").cloned();
                    
                    // Extract styles from document if it exists
                    let styles = document.as_ref().and_then(|d| {
                        d.get("styles").cloned()
                    }).and_then(|styles_json| {
                        serde_json::from_value::<Styles>(styles_json).ok()
                    });
                    
                    // Parse document
                    let parsed_document = document.and_then(|d| {
                        serde_json::from_value::<Document>(d).ok()
                    });
                    
                    Some(FigmaData {
                        document: parsed_document,
                        components,
                        component_sets,
                        styles,
                        name: data_value.get("name").and_then(|v| v.as_str()).map(String::from),
                        version: data_value.get("version").and_then(|v| v.as_str()).map(String::from),
                        role: data_value.get("role").and_then(|v| v.as_str()).map(String::from),
                        last_modified: data_value.get("lastModified").and_then(|v| v.as_str()).map(String::from),
                        thumbnail_url: data_value.get("thumbnailUrl").and_then(|v| v.as_str()).map(String::from),
                    })
                } else {
                    None
                };
                
                Ok(Some(FigmaFile {
                    file_key: doc.file_key,
                    message: doc.message,
                    status: doc.status,
                    data: figma_data,
                }))
            }
            None => Ok(None),
        }
    }
    
    // Additional query to get specific fields directly
    async fn figma_file_data(
        &self,
        ctx: &Context<'_>,
        file_key: String,
        include_document: Option<bool>,
        include_components: Option<bool>,
        include_component_sets: Option<bool>,
        include_styles: Option<bool>,
    ) -> Result<Option<serde_json::Value>> {
        let db = ctx.data::<MongoDb>()?;
        
        let filter = doc! { "file_key": &file_key };
        
        let document: Option<FigmaFileDocument> = db
            .get_document_from_collection("figma_file", filter)
            .await
            .map_err(|e| Error::new(format!("Database error: {}", e)))?;
        
        match document {
            Some(doc) => {
                if let Some(data) = &doc.data {
                    let mut result = serde_json::Map::new();
                    
                    if include_document.unwrap_or(true) {
                        if let Some(document) = data.get("document") {
                            result.insert("document".to_string(), document.clone());
                        }
                    }
                    
                    if include_components.unwrap_or(true) {
                        if let Some(components) = data.get("components") {
                            result.insert("components".to_string(), components.clone());
                        }
                    }
                    
                    if include_component_sets.unwrap_or(true) {
                        if let Some(component_sets) = data.get("componentSets") {
                            result.insert("componentSets".to_string(), component_sets.clone());
                        }
                    }
                    
                    if include_styles.unwrap_or(true) {
                        if let Some(document) = data.get("document") {
                            if let Some(styles) = document.get("styles") {
                                result.insert("styles".to_string(), styles.clone());
                            }
                        }
                    }
                    
                    Ok(Some(serde_json::Value::Object(result)))
                } else {
                    Ok(None)
                }
            }
            None => Ok(None),
        }
    }
}