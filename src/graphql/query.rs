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
    ) -> Result<Option<serde_json::Value>> {
        let db = ctx.data::<MongoDb>()?;
        
        let filter = doc! { "file_key": &file_key };
        
        let document: Option<FigmaFileDocument> = db
            .get_document_from_collection("figma_file", filter)
            .await
            .map_err(|e| Error::new(format!("Database error: {}", e)))?;
        
        Ok(document.and_then(|doc| doc.data))
    }
}