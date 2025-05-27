use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use reqwest::Client;
use mongodb::Client as MongoClient;

// Struct for query parameters
#[derive(Deserialize)]
pub struct NodeParams {
    file_key: Option<String>,
    ids: Option<String>,
}

// Struct for returning response
#[derive(Serialize)]
pub struct ApiResponse {
    file_key: String,
    message: String,
    status: String,
    data: Option<serde_json::Value>,
}

// GET handler that makes an actual request to Figma API
#[get("/getNode")]
pub async fn get_node(
    req: HttpRequest,
    query: web::Query<NodeParams>,
    db: web::Data<crate::db::mongo::MongoDb>,
) -> impl Responder {
    // Check for X-Figma-Token header
    let token = match req.headers().get("X-Figma-Token") {
        Some(token_header) => match token_header.to_str() {
            Ok(token_str) => token_str.to_string(),
            Err(_) => return HttpResponse::BadRequest().json(ApiResponse {
                file_key: "".to_string(),
                message: "Invalid token format".to_string(),
                status: "error".to_string(),
                data: None,
            }),
        },
        None => return HttpResponse::Unauthorized().json(ApiResponse {  
            file_key: "".to_string(),
            message: "Missing X-Figma-Token header".to_string(),
            status: "error".to_string(),
            data: None,
        }),
    };

    // Get the file_key parameter
    let file_key = match &query.file_key {
        Some(id) => id.clone(),
        None => return HttpResponse::BadRequest().json(ApiResponse {
            file_key: "".to_string(),
            message: "Missing file_key parameter".to_string(),
            status: "error".to_string(),
            data: None,
        }),
    };

    // Get the ids parameter
    let ids = match &query.ids {
        Some(ids) => ids.clone(),
        None => return HttpResponse::BadRequest().json(ApiResponse {
            file_key: "".to_string(),
            message: "Missing ids parameter".to_string(),
            status: "error".to_string(),
            data: None,
        }),
    };

    // Create HTTP client
    let client = Client::new();
    
    // Build the Figma API URL for nodes
    let url = format!("https://api.figma.com/v1/files/{}/nodes", file_key);
    
    // Build query parameters using HashMap
    let mut params = HashMap::new();
    params.insert("ids", ids);

    // Make the API request to Figma
    let response = match client
        .get(url)
        .query(&params)
        .header("X-Figma-Token", &token)
        .send()
        .await {
            Ok(resp) => resp,
            Err(e) => return HttpResponse::InternalServerError().json(ApiResponse {
                file_key: "".to_string(),
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
            file_key: "".to_string(),
            message: format!("Figma API error: {}", error_text),
            status: "error".to_string(),
            data: None,
        });
    }

    // Parse JSON response directly
    match response.json::<serde_json::Value>().await {
        Ok(json_data) => {
            // Create the API response
            let api_response = ApiResponse {
                file_key: file_key.clone(),
                message: format!("Successfully retrieved Figma nodes for file: {}", file_key),
                status: "success".to_string(),
                data: Some(json_data.clone()),
            };

            // Store the response in MongoDB
            if let Err(e) = db.insert_into_collection("figma_nodes", &api_response).await {
                eprintln!("Failed to store response in MongoDB: {}", e);
            }

            HttpResponse::Ok().json(api_response)
        },
        Err(e) => {
            HttpResponse::InternalServerError().json(ApiResponse {
                file_key: "".to_string(),
                message: format!("Failed to parse Figma API response: {}", e),
                status: "error".to_string(),
                data: None,
            })
        }
    }
} 