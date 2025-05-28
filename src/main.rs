use actix_web::{App, HttpServer, web, guard};
use std::io;
use actix_web::middleware::Logger;
use actix_cors::Cors;

mod auth;
mod db;
mod figma;
mod mongo;
mod graphql;

use auth::middleware::FigmaTokenMiddleware;
use graphql::handler::{graphql_handler, graphql_playground};

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
    
    // Build GraphQL schema
    let schema = graphql::build_schema();
    
    println!("Starting server at http://127.0.0.1:8080");
    println!("GraphQL Playground available at http://127.0.0.1:8080/api/graphql");
    
    HttpServer::new(move || {
        // Create CORS middleware
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(Logger::default())
            .wrap(cors)  // Add CORS middleware
            .app_data(web::Data::new(mongodb.clone()))
            .app_data(web::Data::new(schema.clone()))
            .service(
                web::scope("/api")
                    .service(
                        web::resource("/graphql")
                            .guard(guard::Post())
                            .to(graphql_handler)
                    )
                    .service(
                        web::resource("/graphql")
                            .guard(guard::Get())
                            .to(graphql_playground)
                    )
                    .service(web::scope("/figma").configure(figma::routes::config))
                    .service(web::scope("/mongo").configure(mongo::routes::config))
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}