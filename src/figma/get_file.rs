use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use reqwest::Client;

// Struct for query parameters
#[derive(Deserialize)]
pub struct FileParams {
    file_key: Option<String>,
    version: Option<String>,
    ids: Option<String>,           
    depth: Option<String>,         
    geometry: Option<String>,      
    plugin_data: Option<String>,   
    branch_data: Option<String>,   
}

// Struct for returning response
#[derive(Serialize)]
pub struct ApiResponse {
    message: String,
    status: String,
    data: Option<serde_json::Value>,
}

// GET handler that makes an actual request to Figma API
#[get("/getFile")]
pub async fn get_file(
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

    // Create HTTP client
    let client = Client::new();
    
    // Build the Figma API URL with query parameters
    let mut url = format!("https://api.figma.com/v1/files/{}", file_key);
    
    // Add query parameters directly to URL if they exist
    if let Some(version) = &query.version {
        url.push_str(&format!("?version={}", version));
    }
    if let Some(ids) = &query.ids {
        url.push_str(&format!("{}ids={}", if url.contains('?') { "&" } else { "?" }, ids));
    }
    if let Some(depth) = &query.depth {
        url.push_str(&format!("{}depth={}", if url.contains('?') { "&" } else { "?" }, depth));
    }
    if let Some(geometry) = &query.geometry {
        url.push_str(&format!("{}geometry={}", if url.contains('?') { "&" } else { "?" }, geometry));
    }
    if let Some(plugin_data) = &query.plugin_data {
        url.push_str(&format!("{}plugin_data={}", if url.contains('?') { "&" } else { "?" }, plugin_data));
    }
    if let Some(branch_data) = &query.branch_data {
        url.push_str(&format!("{}branch_data={}", if url.contains('?') { "&" } else { "?" }, branch_data));
    }

    // Print the full URL and parameters
    println!("\n=== Figma API Request ===");
    println!("URL: {}", url);
    println!("Headers: X-Figma-Token: {}", token);
    println!("=======================\n");

    // Make the API request to Figma
    let response = match client
        .get(url)
        .header("X-Figma-Token", &token)
        .send()
        .await {
            Ok(resp) => resp,
            Err(e) => return HttpResponse::InternalServerError().json(ApiResponse {
                message: format!("Failed to call Figma API: {}", e),
                status: "error".to_string(),
                data: None,
            }),
        };

    // Store status code before consuming response
    let status = response.status();

    // Get raw response
    let raw_response = match response.text().await {
        Ok(text) => text,
        Err(e) => format!("Failed to read response: {}", e),
    };

    // Check status code
    if !status.is_success() {
        return HttpResponse::build(status).json(ApiResponse {
            message: format!("Figma API error: {}", raw_response),
            status: "error".to_string(),
            data: None,
        });
    }

    // Parse JSON response
    match serde_json::from_str::<serde_json::Value>(&raw_response) {
        Ok(json_data) => {
            HttpResponse::Ok().json(ApiResponse {
                message: format!("Successfully retrieved Figma file: {}", file_key),
                status: "success".to_string(),
                data: Some(json_data),
            })
        },
        Err(e) => {
            HttpResponse::InternalServerError().json(ApiResponse {
                message: format!("Failed to parse Figma API response: {}", e),
                status: "error".to_string(),
                data: None,
            })
        }
    }
}