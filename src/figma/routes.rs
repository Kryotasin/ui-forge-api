use actix_web::web;
use crate::figma::{get_file}; // Import the new components module

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_file::get_file)
       // Add the new endpoints for accessing structured data
       .service(get_file::list_components)
       .service(get_file::list_styles)
       .service(get_file::get_document);
}