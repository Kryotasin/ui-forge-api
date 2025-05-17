use actix_web::{post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

// Struct for handling POST request data
#[derive(Deserialize)]
pub struct EchoRequest {
    message: Option<String>,
}

// Struct for returning response
#[derive(Serialize)]
pub struct ApiResponse {
    message: String,
    status: String,
    data: Option<serde_json::Value>,
}

// POST handler that echoes back the request body
#[post("")]
pub async fn echo(request: web::Json<EchoRequest>) -> impl Responder {
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