use actix_web::{web, get, HttpResponse, Responder};
use crate::db::mongo::MongoDb;
use serde::Serialize;

#[derive(Serialize)]
struct ApiResponse {
    message: String,
    status: String,
    data: Option<serde_json::Value>,
}

#[get("/list-db")]
async fn list_databases(db: web::Data<MongoDb>) -> impl Responder {
    // Using the shared MongoDB client to list all databases
    match db.client.list_database_names().await {
        Ok(db_names) => {
            HttpResponse::Ok().json(ApiResponse {
                message: "Databases retrieved successfully".to_string(),
                status: "success".to_string(),
                data: Some(serde_json::json!({ "databases": db_names })),
            })
        },
        Err(e) => {
            HttpResponse::InternalServerError().json(ApiResponse {
                message: format!("Failed to list databases: {}", e),
                status: "error".to_string(),
                data: None,
            })
        }
    }
}

#[post("/add_figma_page")]
async fn add_figma_page(form: web::Form<User>) -> HttpResponse {
    let collection = db.client.database(env::var("MONGODB_DATABASE")).collection('figma_pages');
    let result = collection.insert_one(form.into_inner()).await;
    match result {
        Ok(_) => HttpResponse::Ok().body("Page added"),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(list_databases);
}