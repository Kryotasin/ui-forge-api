use std::process::Command;
use std::path::Path;
use std::io;
use crate::generator::config::GeneratorConfig;

pub struct NpmBuilder {
    config: GeneratorConfig,
}

impl NpmBuilder {
    pub fn new() -> Self {
        NpmBuilder {
            config: GeneratorConfig::from_env(),
        }
    }
    
    pub fn check_npm(&self) -> Result<String, io::Error> {
        let output = Command::new(&self.config.npm_path)
            .arg("--version")
            .output()?;
        
        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
        } else {
            Err(io::Error::new(
                io::ErrorKind::NotFound,
                "npm not found"
            ))
        }
    }
    
    pub fn install_dependencies(&self, project_path: &Path) -> Result<String, io::Error> {
        println!("Running npm install in {:?} using {}", project_path, &self.config.npm_path);
        
        let output = Command::new(&self.config.npm_path)
            .arg("install")
            .current_dir(project_path)
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
    
    pub fn build_package(&self, project_path: &Path) -> Result<String, io::Error> {
        let output = Command::new(&self.config.npm_path)
            .arg("run")
            .arg("build")
            .current_dir(project_path)
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
    
    pub fn pack_package(&self, project_path: &Path) -> Result<String, io::Error> {
        let output = Command::new(&self.config.npm_path)
            .arg("pack")
            .current_dir(project_path)
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