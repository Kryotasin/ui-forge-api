use actix_web::{App, HttpServer, web};
use std::io;
use actix_web::middleware::Logger;

mod auth;
mod db;
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
            .wrap(Logger::default())
            .app_data(web::Data::new(mongodb.clone()))
            .service(
                web::scope("/api")
                    .service(web::scope("/figma").configure(figma::routes::config))
                    .service(web::scope("/mongo").configure(mongo::routes::config))
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}