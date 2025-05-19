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
                    .unwrap_or_else(|_| "kshitijsinha2023".to_string());
                let password = env::var("MONGODB_PASSWORD")
                    .unwrap_or_else(|_| "xpYjAqVbLe6KEXOJ".to_string());
                
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

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(ping);
}