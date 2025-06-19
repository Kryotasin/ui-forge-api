use std::env;

pub struct GeneratorConfig {
    pub npm_path: String,
    pub use_docker: bool,
    pub docker_image: String,
    pub build_timeout_seconds: u64,
}

impl GeneratorConfig {
    pub fn from_env() -> Self {
        GeneratorConfig {
            npm_path: env::var("NPM_PATH").unwrap_or_else(|_| "npm".to_string()),
            use_docker: env::var("USE_DOCKER_BUILD")
                .unwrap_or_else(|_| "true".to_string())
                .parse()
                .unwrap_or(true),
            docker_image: env::var("DOCKER_BUILD_IMAGE")
                .unwrap_or_else(|_| "node-builder:latest".to_string()),
            build_timeout_seconds: 300,
        }
    }
}