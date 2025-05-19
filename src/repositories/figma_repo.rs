use mongodb::{
    bson::doc,
    IndexModel,
    Collection
};
use async_trait::async_trait;
use crate::db::mongo::MongoDb;
use crate::models::figma::FigmaFile;
use std::sync::Arc;
use futures::TryStreamExt;

#[async_trait]
pub trait FigmaRepository: Send + Sync {
    async fn get_file(&self, file_key: &str) -> Result<Option<FigmaFile>, mongodb::error::Error>;
    async fn get_file_by_id(&self, file_id: &str) -> Result<Option<FigmaFile>, mongodb::error::Error>;
    async fn save_file(&self, file: &mut FigmaFile) -> Result<String, mongodb::error::Error>;
    async fn list_files(&self, limit: Option<i64>) -> Result<Vec<FigmaFile>, mongodb::error::Error>;
}

pub struct FigmaRepositoryImpl {
    db: Arc<MongoDb>,
    collection: Collection<FigmaFile>,
}

impl FigmaRepositoryImpl {
    pub async fn new(db: Arc<MongoDb>) -> Result<Self, mongodb::error::Error> {
        let collection_name = "figma_files";
        let collection = db.collection::<FigmaFile>(collection_name);
        
        // Create instance
        let repo = Self {
            db,
            collection,
        };
        
        // Create indexes
        repo.create_indexes().await?;
        
        Ok(repo)
    }
    
    // Create required indexes
    async fn create_indexes(&self) -> Result<(), mongodb::error::Error> {
        let file_id_model = IndexModel::builder()
            .keys(doc! { "file_id": 1 })
            .options(mongodb::options::IndexOptions::builder().unique(true).build())
            .build();

        let file_key_model = IndexModel::builder()
            .keys(doc! { "file_key": 1 })
            .options(mongodb::options::IndexOptions::builder().unique(true).build())
            .build();

        self.collection.create_index(file_id_model).await?;
        self.collection.create_index(file_key_model).await?;
        Ok(())
    }
}

#[async_trait]
impl FigmaRepository for FigmaRepositoryImpl {
    async fn get_file(&self, file_key: &str) -> Result<Option<FigmaFile>, mongodb::error::Error> {
        self.collection.find_one(doc! { "file_key": file_key }).await
    }
    
    async fn get_file_by_id(&self, file_id: &str) -> Result<Option<FigmaFile>, mongodb::error::Error> {
        self.collection.find_one(doc! { "_id": file_id }).await
    }

    async fn save_file(&self, file: &mut FigmaFile) -> Result<String, mongodb::error::Error> {
        let file_key = file.file_key.clone();
        let file_id = file.file_id.clone();
        
        let result = self.collection
            .replace_one(
                doc! { "file_key": &file_key },
                file,
            )
            .await?;

        if let Some(id) = result.upserted_id {
            Ok(id.as_object_id().unwrap().to_hex())
        } else {
            Ok(file_id.unwrap())
        }
    }

    async fn list_files(&self, limit: Option<i64>) -> Result<Vec<FigmaFile>, mongodb::error::Error> {
        let mut find_options = mongodb::options::FindOptions::default();
        if let Some(limit) = limit {
            find_options.limit = Some(limit);
        }

        let cursor = self.collection.find(doc! {}).await?;
        let files: Vec<FigmaFile> = cursor.try_collect().await?;
        Ok(files)
    }
}