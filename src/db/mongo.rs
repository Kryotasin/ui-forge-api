use mongodb::{
    bson::doc, 
    options::{ClientOptions, ServerApi, ServerApiVersion}, 
    Client, Database
};
use std::env;

// Allow dead_code warnings since these will be used in the future
#[allow(dead_code)]
#[derive(Clone)]
pub struct MongoDb {
    pub client: Client,
    pub database: Database,
}

impl MongoDb {
    pub async fn init() -> Result<Self, mongodb::error::Error> {
        // Get MongoDB connection string - try complete URI first, then build from components
        let mongo_uri = match env::var("MONGODB_URI") {
            Ok(uri) => uri,
            Err(_) => {
                // Get username and password from environment variables
                let username = env::var("MONGODB_USERNAME")
                    .unwrap_or_else(|_| "kshitijsinha2023".to_string());
                let password = env::var("MONGODB_PASSWORD")
                    .unwrap_or_else(|_| "xpYjAqVbLe6KEXOJ".to_string()); //TODO: Remove this
                
                // Build the connection string with username and password
                format!(
                    "mongodb+srv://{}:{}@cluster0.jio8mfu.mongodb.net/?retryWrites=true&w=majority&appName=Cluster0",
                    username, password
                )
            }
        };
        
        // Get database name
        let db_name = env::var("MONGODB_DATABASE")
            .unwrap_or_else(|_| "ui_forge".to_string());

        println!("Connecting to MongoDB...");

        // Configure and create the client with Atlas settings
        let mut client_options = ClientOptions::parse(mongo_uri).await?;
        
        // Set the server_api field to use the Stable API v1
        let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
        client_options.server_api = Some(server_api);
        
        // Set pool size
        client_options.max_pool_size = Some(10);
        
        // Create the client
        let client = Client::with_options(client_options)?;
        
        // Get a handle to the database
        let database = client.database(&db_name);
        
        // Ping the database to verify the connection works
        client.database("admin")
            .run_command(doc! {"ping": 1})
            .await?;
        
        println!("Pinged your deployment. You successfully connected to MongoDB!");
        
        Ok(MongoDb { client, database })
    }

    pub async fn insert_into_collection<T>(
        &self,
        collection_name: &str,
        document: T,
    ) -> Result<(), mongodb::error::Error> 
    where
        T: serde::Serialize + Send + Sync,
    {
        let collection = self.database.collection(collection_name);
        let res = collection.insert_one(document).await?;
        println!("Inserted a document into {}", res.inserted_id);
        Ok(())
    }

    pub async fn get_document_from_collection<T>(
        &self,
        collection_name: &str,
        filter: mongodb::bson::Document,
    ) -> Result<Option<T>, mongodb::error::Error> 
    where 
        T: serde::de::DeserializeOwned + Send + Sync,
    {
        let collection = self.database.collection(collection_name);
        let document = collection.find_one(filter).await?;
        Ok(document)
    }
    
    
    
    // This method will be used in the future as your app grows
    #[allow(dead_code)]
    pub fn collection<T>(&self, name: &str) -> mongodb::Collection<T> 
    where 
        T: serde::de::DeserializeOwned + serde::Serialize + Send + Sync,
    {
        self.database.collection(name)
    }
}