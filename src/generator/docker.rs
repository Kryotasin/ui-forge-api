use std::process::Command;
use std::path::{Path, PathBuf};
use std::io;
use crate::generator::config::GeneratorConfig;

pub struct DockerBuilder {
    config: GeneratorConfig,
}

impl DockerBuilder {
    pub fn new() -> Self {
        DockerBuilder {
            config: GeneratorConfig::from_env(),
        }
    }
    
    pub fn check_docker(&self) -> Result<String, io::Error> {
        let output = Command::new("docker")
            .arg("--version")
            .output()?;
        
        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
        } else {
            Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Docker not found"
            ))
        }
    }
    
    pub fn build_image(&self) -> Result<String, io::Error> {
        println!("Building Docker image...");
        
        // Get the current working directory
        let current_dir = std::env::current_dir()?;
        let dockerfile_path = current_dir.join("docker").join("node-builder.Dockerfile");
        
        // Check if Dockerfile exists
        if !dockerfile_path.exists() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                format!("Dockerfile not found at: {:?}", dockerfile_path)
            ));
        }
        
        let output = Command::new("docker")
            .args(&[
                "build",
                "-f", dockerfile_path.to_str().unwrap(),
                "-t", &self.config.docker_image,
                ".",
            ])
            .current_dir(&current_dir)  // Make sure we're in the project root
            .output()?;
        
        if output.status.success() {
            Ok("Docker image built successfully".to_string())
        } else {
            Err(io::Error::new(
                io::ErrorKind::Other,
                format!("Docker build failed: {}", String::from_utf8_lossy(&output.stderr))
            ))
        }
    }
    
    pub fn run_npm_install(&self, project_path: &Path) -> Result<String, io::Error> {
        let absolute_path = project_path.canonicalize()?;
        
        let output = Command::new("docker")
            .args(&[
                "run",
                "--rm",
                "-v", &format!("{}:/build", absolute_path.display()),
                &self.config.docker_image,
                "npm", "install"
            ])
            .output()?;
        
        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Err(io::Error::new(
                io::ErrorKind::Other,
                format!("npm install failed: {}", String::from_utf8_lossy(&output.stderr))
            ))
        }
    }
    
    pub fn run_npm_build(&self, project_path: &Path) -> Result<String, io::Error> {
        let absolute_path = project_path.canonicalize()?;
        
        let output = Command::new("docker")
            .args(&[
                "run",
                "--rm",
                "-v", &format!("{}:/build", absolute_path.display()),
                &self.config.docker_image,
                "npm", "run", "build"
            ])
            .output()?;
        
        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Err(io::Error::new(
                io::ErrorKind::Other,
                format!("npm build failed: {}", String::from_utf8_lossy(&output.stderr))
            ))
        }
    }
    
    pub fn run_npm_pack(&self, project_path: &Path) -> Result<String, io::Error> {
        let absolute_path = project_path.canonicalize()?;
        
        let output = Command::new("docker")
            .args(&[
                "run",
                "--rm",
                "-v", &format!("{}:/build", absolute_path.display()),
                &self.config.docker_image,
                "npm", "pack"
            ])
            .output()?;
        
        if output.status.success() {
            let filename = String::from_utf8_lossy(&output.stdout).trim().to_string();
            Ok(filename)
        } else {
            Err(io::Error::new(
                io::ErrorKind::Other,
                format!("npm pack failed: {}", String::from_utf8_lossy(&output.stderr))
            ))
        }
    }
}