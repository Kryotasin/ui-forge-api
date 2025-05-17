use actix_web::{App, HttpServer, web};
use std::io;

mod auth;
mod figma;
mod mongo; 

use auth::middleware::FigmaTokenMiddleware;

#[actix_web::main]
async fn main() -> io::Result<()> {
    println!("Starting server at http://127.0.0.1:8080");
    
    HttpServer::new(|| {
        App::new()
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