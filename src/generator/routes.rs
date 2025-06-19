use actix_web::{post, get, web, HttpResponse, Responder};
use crate::generator::models::{CreateComponentRequest, CreateComponentResponse};
use crate::generator::builder::generate_package_files;
use crate::generator::filesystem::PackageBuilder;
use crate::generator::docker::DockerBuilder;


#[get("/build-docker-image")]
pub async fn build_docker_image() -> impl Responder {
    let docker_builder = DockerBuilder::new();
    
    // First check if Docker is available
    match docker_builder.check_docker() {
        Ok(version) => {
            println!("Docker found: {}", version);
        }
        Err(e) => {
            return HttpResponse::ServiceUnavailable().json(serde_json::json!({
                "success": false,
                "message": format!("Docker not available: {}", e)
            }));
        }
    }
    
    // Build the Docker image
    match docker_builder.build_image() {
        Ok(message) => {
            HttpResponse::Ok().json(serde_json::json!({
                "success": true,
                "message": message
            }))
        }
        Err(e) => {
            HttpResponse::InternalServerError().json(serde_json::json!({
                "success": false,
                "message": format!("Failed to build Docker image: {}", e)
            }))
        }
    }
}

#[post("/create-component")]
pub async fn create_component(
    req: web::Json<CreateComponentRequest>,
) -> impl Responder {
    // Generate all package files
    let package = match generate_package_files(&req) {
        Ok(pkg) => pkg,
        Err(e) => {
            return HttpResponse::BadRequest().json(CreateComponentResponse {
                success: false,
                message: e,
                job_id: None,
            })
        }
    };
    
    // Create package directory and write files
    let builder = match PackageBuilder::new() {
        Ok(b) => b,
        Err(e) => {
            return HttpResponse::InternalServerError().json(CreateComponentResponse {
                success: false,
                message: format!("Failed to create build directory: {}", e),
                job_id: None,
            })
        }
    };
    
    // Write all files to disk
    if let Err(e) = builder.write_package(&req, &package) {
        return HttpResponse::InternalServerError().json(CreateComponentResponse {
            success: false,
            message: format!("Failed to write files: {}", e),
            job_id: None,
        })
    }
    
    let mut response_message = format!("Package created at: {:?}", builder.get_path());
    
    // Use Docker to build the package
    let docker_builder = DockerBuilder::new();
    
    // Check Docker availability
    match docker_builder.check_docker() {
        Ok(_) => {
            response_message.push_str("\n\nStarting Docker build process...");
            
            // Run npm install
            response_message.push_str("\n1. Running npm install...");
            match docker_builder.run_npm_install(builder.get_path()) {
                Ok(_) => response_message.push_str(" ✓ Success"),
                Err(e) => {
                    response_message.push_str(&format!(" ✗ Failed: {}", e));
                    return HttpResponse::Ok().json(CreateComponentResponse {
                        success: true,
                        message: response_message,
                        job_id: Some(builder.job_id),
                    });
                }
            }
            
            // Run npm build
            response_message.push_str("\n2. Running npm build...");
            match docker_builder.run_npm_build(builder.get_path()) {
                Ok(_) => response_message.push_str(" ✓ Success"),
                Err(e) => {
                    response_message.push_str(&format!(" ✗ Failed: {}", e));
                    return HttpResponse::Ok().json(CreateComponentResponse {
                        success: true,
                        message: response_message,
                        job_id: Some(builder.job_id),
                    });
                }
            }
            
            // Run npm pack
            response_message.push_str("\n3. Creating npm package...");
            match docker_builder.run_npm_pack(builder.get_path()) {
                Ok(filename) => {
                    response_message.push_str(&format!(" ✓ Success: {}", filename));
                    response_message.push_str(&format!("\n\nPackage ready: {:?}/{}", builder.get_path(), filename));
                }
                Err(e) => {
                    response_message.push_str(&format!(" ✗ Failed: {}", e));
                }
            }
        }
        Err(e) => {
            response_message.push_str(&format!("\n\nDocker not available: {}", e));
            response_message.push_str("\nPlease ensure Docker is installed and running");
            response_message.push_str("\nThen call GET /api/generator/build-docker-image first");
        }
    }
    
    HttpResponse::Ok().json(CreateComponentResponse {
        success: true,
        message: response_message,
        job_id: Some(builder.job_id),
    })
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(create_component)
       .service(build_docker_image);
}