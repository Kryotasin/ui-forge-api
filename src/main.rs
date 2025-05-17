use actix_web::{App, HttpServer, web};
use std::io;

// Import the Figma module
mod figma;

#[actix_web::main]
async fn main() -> io::Result<()> {
    println!("Starting server at http://127.0.0.1:8080");
    
    HttpServer::new(|| {
        App::new()
            // Configure API routes under /api prefix
            .service(
                web::scope("/api")
                    // Mount the Figma module routes under /api/figma
                    .service(web::scope("/figma").configure(figma::routes::config))
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}