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
    
    // Build the Figma API URL (using HashMap approach for cleaner parameter building)
    let url = format!("https://api.figma.com/v1/files/{}", file_key);
    
    // Build query parameters using HashMap
    let mut params = HashMap::new();
    if let Some(version) = &query.version {
        params.insert("version", version.clone());
    }
    if let Some(ids) = &query.ids {
        params.insert("ids", ids.clone());
    }
    if let Some(depth) = &query.depth {
        params.insert("depth", depth.clone());
    }
    if let Some(geometry) = &query.geometry {
        params.insert("geometry", geometry.clone());
    }
    if let Some(plugin_data) = &query.plugin_data {
        params.insert("plugin_data", plugin_data.clone());
    }
    if let Some(branch_data) = &query.branch_data {
        params.insert("branch_data", branch_data.clone());
    }

    // Make the API request to Figma
    let response = match client
        .get(url)
        .query(&params)  // Use .query() with HashMap
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

    // Check status code before consuming response body
    if !response.status().is_success() {
        let status = response.status();
        // Get error response body
        let error_text = match response.text().await {
            Ok(text) => text,
            Err(_) => "Could not read error response".to_string(),
        };
        
        return HttpResponse::build(status).json(ApiResponse {
            message: format!("Figma API error: {}", error_text),
            status: "error".to_string(),
            data: None,
        });
    }

    // Parse JSON response directly (more efficient - single step)
    match response.json::<serde_json::Value>().await {
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