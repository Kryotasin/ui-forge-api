use mongodb::{
    bson::doc, 
    options::{ClientOptions, ServerApi, ServerApiVersion}, 
    Client, Database, Collection
};
use std::env;
use serde::{de::DeserializeOwned, Serialize};
use actix_web::{get, web, HttpResponse, Responder};

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
                    .map_err(|_| mongodb::error::Error::from(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "MONGODB_USERNAME environment variable is required"
                    )))?;
                let password = env::var("MONGODB_PASSWORD")
                    .map_err(|_| mongodb::error::Error::from(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "MONGODB_PASSWORD environment variable is required"
                    )))?;
                
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

        println!("Connecting to MongoDB with URI: {}", mongo_uri.replace(&env::var("MONGODB_PASSWORD").unwrap_or_default(), "****"));

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
    
    // Helper to get a typed collection
    pub fn collection<T>(&self, name: &str) -> Collection<T> 
    where 
        T: DeserializeOwned + Serialize + Send + Sync,
    {
        self.database.collection(name)
    }
}

#[get("/ping")]
pub async fn ping(db: web::Data<crate::db::mongo::MongoDb>) -> impl Responder {
    match db.database.run_command(doc! {"ping": 1}).await {
        Ok(_) => HttpResponse::Ok().json("pong"),
        Err(e) => HttpResponse::InternalServerError().json(format!("Database error: {}", e))
    }
}

#[get("/get-file-by-key/{file_key}")]
pub async fn get_file_by_key(
    db: web::Data<crate::db::mongo::MongoDb>,
    file_key: web::Path<String>,
) -> impl Responder {
    let file_key_str = file_key.into_inner();
    let filter = doc! { "file_key": file_key_str };
    
    match db.get_document_from_collection::<serde_json::Value>("figma_file", filter).await {
        Ok(Some(document)) => HttpResponse::Ok().json(document),
        Ok(None) => HttpResponse::NotFound().json("File not found"),
        Err(e) => HttpResponse::InternalServerError().json(format!("Database error: {}", e))
    }
}

#[get("/get-node/{file_key}/{node_id}")]
pub async fn get_node_by_id(
    db: web::Data<crate::db::mongo::MongoDb>,
    path: web::Path<(String, String)>,
) -> impl Responder {
    let (file_key_str, node_id_str) = path.into_inner();
    let filter = doc! { "file_key": file_key_str, "node_id": node_id_str };
    println!("Filter: {:?}", filter);
    
    match db.get_document_from_collection::<serde_json::Value>("figma_nodes", filter).await {
        Ok(Some(document)) => HttpResponse::Ok().json(document),
        Ok(None) => HttpResponse::NotFound().json("Node not found"),
        Err(e) => HttpResponse::InternalServerError().json(format!("Database error: {}", e))
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(ping)
       .service(get_file_by_key)
       .service(get_node_by_id);
}