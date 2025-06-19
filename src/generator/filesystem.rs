use std::fs;
use std::path::{Path, PathBuf};
use std::io::Write;
use uuid::Uuid;
use crate::generator::builder::GeneratedPackage;
use crate::generator::models::CreateComponentRequest;

pub struct PackageBuilder {
    pub job_id: String,
    pub base_path: PathBuf,
}

impl PackageBuilder {
    pub fn write_rollup_config(&self) -> Result<(), std::io::Error> {
        let rollup_config = r#"import typescript from '@rollup/plugin-typescript';
    
    export default {
      input: 'src/index.ts',
      output: [
        {
          file: 'dist/index.js',
          format: 'cjs',
          exports: 'named'
        },
        {
          file: 'dist/index.esm.js',
          format: 'es'
        }
      ],
      external: ['react', 'react-dom'],
      plugins: [typescript()]
    };
    "#;
        
        let rollup_path = self.base_path.join("rollup.config.js");
        let mut file = fs::File::create(rollup_path)?;
        file.write_all(rollup_config.as_bytes())?;
        
        Ok(())
    }


    pub fn new() -> Result<Self, std::io::Error> {
        let job_id = Uuid::new_v4().to_string();
        let base_path = PathBuf::from("./tmp/builds").join(&job_id);
        
        // Create the base directory structure
        fs::create_dir_all(&base_path)?;
        fs::create_dir_all(base_path.join("src"))?;
        
        Ok(PackageBuilder {
            job_id,
            base_path,
        })
    }
    
    pub fn write_package(&self, request: &CreateComponentRequest, package: &GeneratedPackage) -> Result<(), std::io::Error> {
        // Write component file
        let component_filename = if request.typescript {
            format!("{}.tsx", request.name)
        } else {
            format!("{}.jsx", request.name)
        };
        
        let component_path = self.base_path.join("src").join(component_filename);
        let mut file = fs::File::create(component_path)?;
        file.write_all(package.component_code.as_bytes())?;
        
        // Write index.ts
        let index_path = self.base_path.join("src").join("index.ts");
        let mut file = fs::File::create(index_path)?;
        file.write_all(package.index_ts.as_bytes())?;
        
        // Write package.json
        let package_json_path = self.base_path.join("package.json");
        let mut file = fs::File::create(package_json_path)?;
        file.write_all(package.package_json.as_bytes())?;
        
        // Write tsconfig.json if TypeScript
        if request.typescript {
            let tsconfig = self.create_tsconfig();
            let tsconfig_path = self.base_path.join("tsconfig.json");
            let mut file = fs::File::create(tsconfig_path)?;
            file.write_all(tsconfig.as_bytes())?;
        }
        self.write_rollup_config()?;
        Ok(())
    }
    
    fn create_tsconfig(&self) -> String {
        r#"{
  "compilerOptions": {
    "target": "es5",
    "module": "esnext",
    "lib": ["dom", "esnext"],
    "declaration": true,
    "outDir": "./dist",
    "strict": true,
    "jsx": "react",
    "esModuleInterop": true,
    "skipLibCheck": true,
    "forceConsistentCasingInFileNames": true
  },
  "include": ["src"],
  "exclude": ["node_modules", "dist"]
}"#.to_string()
    }
    
    pub fn get_path(&self) -> &Path {
        &self.base_path
    }
}