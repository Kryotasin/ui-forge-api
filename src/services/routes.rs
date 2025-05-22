use actix_web::{web, HttpResponse, Scope};
use crate::models::figma::FigmaFileQuery;
use crate::services::figma::{FigmaService, FigmaServiceImpl};
use std::sync::Arc;

// Example handler function
async fn hello() -> HttpResponse {
    HttpResponse::Ok().json("Hello from services API!")
}

// Example handler with path parameter
async fn get_service(id: web::Path<String>) -> HttpResponse {
    HttpResponse::Ok().json(format!("Service ID: {}", id))
}

// Handler for getting a Figma file
async fn get_figma_file(
    query: web::Query<FigmaFileQuery>,
    req: actix_web::HttpRequest,
    figma_service: web::Data<Arc<FigmaServiceImpl>>,
) -> HttpResponse {
    // Get the token from the X-Figma-Token header
    let token = match req.headers().get("X-Figma-Token") {
        Some(token) => match token.to_str() {
            Ok(token) => token,
            Err(_) => return HttpResponse::BadRequest().json("Invalid token format")
        },
        None => return HttpResponse::Unauthorized().json("Missing Figma token")
    };

    match figma_service.get_figma_file(query.into_inner(), token).await {
        Ok(file) => HttpResponse::Ok().json(file),
        Err(e) => HttpResponse::BadRequest().json(format!("Error: {}", e))
    }
}

// Handler for listing cached files
async fn list_cached_files(
    limit: web::Query<Option<i64>>,
    figma_service: web::Data<Arc<FigmaServiceImpl>>,
) -> HttpResponse {
    match figma_service.list_cached_files(limit.into_inner()).await {
        Ok(files) => HttpResponse::Ok().json(files),
        Err(e) => HttpResponse::BadRequest().json(format!("Error: {}", e))
    }
}

// Handler for getting file by ID
async fn get_file_by_id(
    file_id: web::Path<String>,
    figma_service: web::Data<Arc<FigmaServiceImpl>>,
) -> HttpResponse {
    match figma_service.get_file_by_id(&file_id).await {
        Ok(Some(file)) => HttpResponse::Ok().json(file),
        Ok(None) => HttpResponse::NotFound().json("File not found"),
        Err(e) => HttpResponse::BadRequest().json(format!("Error: {}", e))
    }
}

// Configure all routes for the services scope
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/figma")
            .route("/file", web::get().to(get_figma_file))
            .route("/files", web::get().to(list_cached_files))
            .route("/file/{file_id}", web::get().to(get_file_by_id))
    );
} 