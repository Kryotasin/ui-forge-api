use actix_web::{App, HttpServer, web};
use std::io;

mod auth;
mod db;    // New module for database connections
mod figma;
mod mongo; 

use auth::middleware::FigmaTokenMiddleware;

#[actix_web::main]
async fn main() -> io::Result<()> {
    // Initialize MongoDB connection
    let mongodb = match db::mongo::MongoDb::init().await {
        Ok(db) => {
            println!("MongoDB initialized successfully");
            db
        },
        Err(e) => {
            eprintln!("Failed to connect to MongoDB: {}", e);
            std::process::exit(1);
        }
    };
    
    println!("Starting server at http://127.0.0.1:8080");
    
    HttpServer::new(move || {
        App::new()
            // Add MongoDB client to application state
            .app_data(web::Data::new(mongodb.clone()))
            // Configure API routes under /api prefix
            .service(
                web::scope("/api")
                    // Mount the Figma module routes under /api/figma with auth middleware
                    .service(
                        web::scope("/figma")
                            .wrap(FigmaTokenMiddleware::new())
                            .configure(figma::routes::config)
                    )
                    // Mount the MongoDB module routes under /api/mongo
                    .service(web::scope("/mongo").configure(mongo::routes::config))
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}