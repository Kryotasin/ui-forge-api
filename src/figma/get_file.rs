use actix_web::{get, web, HttpResponse, Responder};
use crate::models::figma::ApiResponse;
use crate::services::figma_service::FigmaService;

// GET handler to list components from a file
#[get("/file/{file_id}/components")]
pub async fn list_components(
    path: web::Path<String>,
    figma_service: web::Data<dyn FigmaService>,
) -> impl Responder {
    let file_id = path.into_inner();
    
    // Get the file from the database
    match figma_service.get_file_by_id(&file_id).await {
        Ok(Some(file)) => {
            // Extract components
            let components = file.data.components;
            HttpResponse::Ok().json(ApiResponse {
                message: format!("Retrieved {} components from file {}", components.len(), file_id),
                status: "success".to_string(),
                data: Some(components),
            })
        },
        Ok(None) => {
            HttpResponse::NotFound().json(ApiResponse {
                message: format!("File with ID {} not found", file_id),
                status: "error".to_string(),
                data: None::<()>,
            })
        },
        Err(e) => {
            HttpResponse::InternalServerError().json(ApiResponse {
                message: format!("Failed to retrieve file: {}", e),
                status: "error".to_string(),
                data: None::<()>,
            })
        }
    }
}

// GET handler to list styles from a file
#[get("/file/{file_id}/styles")]
pub async fn list_styles(
    path: web::Path<String>,
    figma_service: web::Data<dyn FigmaService>,
) -> impl Responder {
    let file_id = path.into_inner();
    
    // Get the file from the database
    match figma_service.get_file_by_id(&file_id).await {
        Ok(Some(file)) => {
            // Extract styles (convert HashMap to Vec for easier JSON serialization)
            let styles = file.data.styles.into_values().collect::<Vec<_>>();
            HttpResponse::Ok().json(ApiResponse {
                message: format!("Retrieved {} styles from file {}", styles.len(), file_id),
                status: "success".to_string(),
                data: Some(styles),
            })
        },
        Ok(None) => {
            HttpResponse::NotFound().json(ApiResponse {
                message: format!("File with ID {} not found", file_id),
                status: "error".to_string(),
                data: None::<()>,
            })
        },
        Err(e) => {
            HttpResponse::InternalServerError().json(ApiResponse {
                message: format!("Failed to retrieve file: {}", e),
                status: "error".to_string(),
                data: None::<()>,
            })
        }
    }
}

// GET handler to get document structure
#[get("/file/{file_id}/document")]
pub async fn get_document(
    path: web::Path<String>,
    figma_service: web::Data<dyn FigmaService>,
) -> impl Responder {
    let file_id = path.into_inner();
    
    // Get the file from the database
    match figma_service.get_file_by_id(&file_id).await {
        Ok(Some(file)) => {
            // Extract document
            match file.data.document {
                Some(document) => {
                    HttpResponse::Ok().json(ApiResponse {
                        message: format!("Retrieved document from file {}", file_id),
                        status: "success".to_string(),
                        data: Some(document),
                    })
                },
                None => {
                    HttpResponse::NotFound().json(ApiResponse {
                        message: format!("No document found in file {}", file_id),
                        status: "error".to_string(),
                        data: None::<()>,
                    })
                }
            }
        },
        Ok(None) => {
            HttpResponse::NotFound().json(ApiResponse {
                message: format!("File with ID {} not found", file_id),
                status: "error".to_string(),
                data: None::<()>,
            })
        },
        Err(e) => {
            HttpResponse::InternalServerError().json(ApiResponse {
                message: format!("Failed to retrieve file: {}", e),
                status: "error".to_string(),
                data: None::<()>,
            })
        }
    }
}