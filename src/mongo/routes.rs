use actix_web::{web, get, HttpResponse};

#[get("/list-db")]
async fn list_databases() -> HttpResponse {
    HttpResponse::Ok().body("List of MongoDB databases")
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(list_databases);
}