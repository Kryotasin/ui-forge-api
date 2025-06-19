use tera::{Tera, Context};
use once_cell::sync::Lazy;
use std::sync::Mutex;
use crate::generator::models::CreateComponentRequest;
use std::collections::HashMap;

// Initialize Tera as a global static
pub static TEMPLATES: Lazy<Mutex<Tera>> = Lazy::new(|| {
    let mut tera = Tera::default();
    
    // React component template (keep existing)
    tera.add_raw_template("react_component", r##"
import React from 'react';
{%- if typescript %}

interface {{ name }}Props {
  children?: React.ReactNode;
  onClick?: () => void;
  className?: string;
}
{%- endif %}

const {{ name }}{% if typescript %}: React.FC<{{ name }}Props>{% endif %} = ({ children, onClick, className }) => {
  const styles{% if typescript %}: React.CSSProperties{% endif %} = {
{%- for key, value in config %}
    {{ key }}: '{{ value }}',
{%- endfor %}
    cursor: 'pointer',
  };

  return (
    <{{ component_type }}
      style={styles}
      onClick={onClick}
      className={className}
    >
      {children}
    </{{ component_type }}>
  );
};

export default {{ name }};
"##).expect("Failed to add react_component template");

    // Add package.json template
    tera.add_raw_template("package_json", r##"{
  "name": "{{ package_name }}",
  "version": "{{ version }}",
  "description": "{{ name }} component",
  "main": "dist/index.js",
  "module": "dist/index.esm.js",
  "types": "dist/index.d.ts",
  "files": [
    "dist"
  ],
  "scripts": {
    "build": "rollup -c",
    "prepublishOnly": "npm run build"
  },
  "peerDependencies": {
    "react": "^16.8.0 || ^17.0.0 || ^18.0.0",
    "react-dom": "^16.8.0 || ^17.0.0 || ^18.0.0"
  },
  "devDependencies": {
    "@types/react": "^18.0.0",
    "@rollup/plugin-typescript": "^11.1.0",
    "rollup": "^3.20.0",
    "typescript": "^5.0.0",
    "tslib": "^2.5.0"
  },
  "license": "MIT"
}
"##).expect("Failed to add package_json template");

    Mutex::new(tera)
});

// Add new struct to hold all generated files
pub struct GeneratedPackage {
    pub component_code: String,
    pub package_json: String,
    pub index_ts: String,
}

// Update function to generate all files
pub fn generate_package_files(request: &CreateComponentRequest) -> Result<GeneratedPackage, String> {
    let tera = TEMPLATES.lock().unwrap();
    let mut context = Context::new();
    
    // Common context
    context.insert("name", &request.name);
    context.insert("component_type", &request.component_type);
    context.insert("typescript", &request.typescript);
    context.insert("config", &request.config);
    context.insert("package_name", &request.package_name);
    context.insert("version", &request.version);
    
    // Generate component code
    let component_code = tera.render("react_component", &context)
        .map_err(|e| format!("Component template error: {}", e))?;
    
    // Generate package.json
    let package_json = tera.render("package_json", &context)
        .map_err(|e| format!("Package.json template error: {}", e))?;
    
    // Generate index.ts (simple export)
    let index_ts = format!("export {{ default as {} }} from './{}';", &request.name, &request.name);
    
    Ok(GeneratedPackage {
        component_code,
        package_json,
        index_ts,
    })
}