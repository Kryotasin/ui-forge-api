use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

// Struct for handling POST request data
#[derive(Deserialize)]
pub struct EchoRequest {
    message: Option<String>,
}

// Struct for query parameters
#[derive(Deserialize)]
pub struct FileParams {
    file_key: Option<String>,
    version: Option<String>,
}

// Struct for returning response
#[derive(Serialize)]
pub struct ApiResponse {
    message: String,
    status: String,
    data: Option<serde_json::Value>,
}

// GET handler that accepts parameters and checks for X-Figma-Token header
#[get("/getFile")]
async fn get_file(
    req: HttpRequest,
    query: web::Query<FileParams>,
) -> impl Responder {
    // Check for X-Figma-Token header
    let token = match req.headers().get("X-Figma-Token") {
        Some(token_header) => match token_header.to_str() {
            Ok(token_str) => token_str.to_string(),
            Err(_) => return HttpResponse::BadRequest().json(ApiResponse {
                message: "Invalid token format".to_string(),
                status: "error".to_string(),
                data: None,
            }),
        },
        None => return HttpResponse::Unauthorized().json(ApiResponse {
            message: "Missing X-Figma-Token header".to_string(),
            status: "error".to_string(),
            data: None,
        }),
    };

    // Get the file_key parameter
    let file_key = match &query.file_key {
        Some(id) => id.clone(),
        None => return HttpResponse::BadRequest().json(ApiResponse {
            message: "Missing file_key parameter".to_string(),
            status: "error".to_string(),
            data: None,
        }),
    };

    // Get optional version parameter
    let version_info = match &query.version {
        Some(ver) => format!(" (version: {})", ver),
        None => "".to_string(),
    };

    // Return success response
    HttpResponse::Ok().json(ApiResponse {
        message: format!("Successfully retrieved file: {}{}", file_key, version_info),
        status: "success".to_string(),
        data: Some(serde_json::json!({
            "file_key": file_key,
            "token_provided": true,
            "token_length": token.len(),
            "version": query.version
        })),
    })
}

// POST handler that echoes back the request body
#[post("")]
async fn echo(request: web::Json<EchoRequest>) -> impl Responder {
    let message = match &request.message {
        Some(msg) => msg.clone(),
        None => "No message provided".to_string(),
    };
    
    HttpResponse::Ok().json(ApiResponse {
        message,
        status: "success".to_string(),
        data: None,
    })
}

// Configuration function that registers all routes
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_file)
       .service(echo);
}