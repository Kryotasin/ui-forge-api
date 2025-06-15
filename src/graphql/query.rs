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
        
        // Get the raw document
        let document: Option<FigmaFileDocument> = db
            .get_document_from_collection("figma_file", filter.clone())
            .await
            .map_err(|e| Error::new(format!("Database error: {}", e)))?;
        
        // Convert to our GraphQL type
        match document {
            Some(doc) => {
                Ok(Some(FigmaFile {
                    file_key: doc.file_key,
                    message: doc.message,
                    status: doc.status,
                    data: doc.data,
                }))
            }
            None => Ok(None),
        }
    }
    
    async fn figma_file_data(
        &self,
        ctx: &Context<'_>,
        file_key: String,
        node_id: String,
    ) -> Result<Option<serde_json::Value>> {
        println!("File key: {:?}", file_key);
        println!("Node ID: {:?}", node_id);
        
        let db = ctx.data::<MongoDb>()?;
        
        // Option 1: Filter by both file_key and node_id
        let filter = doc! { 
            "file_key": &file_key,
            "node_id": &node_id 
        };
        
        let document: Option<FigmaFileDocument> = db
            .get_document_from_collection("figma_nodes", filter)
            .await
            .map_err(|e| Error::new(format!("Database error: {}", e)))?;
        
        Ok(document.and_then(|doc| doc.data))
    }
}