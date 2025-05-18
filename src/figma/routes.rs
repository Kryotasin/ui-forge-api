use actix_web::web;
use crate::figma::{get_file, components}; // Import the new components module

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_file::get_file)
       // Add the new endpoints for accessing structured data
       .service(components::list_components)
       .service(components::list_styles)
       .service(components::get_document);
}