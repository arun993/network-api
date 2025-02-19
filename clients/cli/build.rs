use prost_build::Config;
use std::error::Error;
use std::fs;
use std::process::Command;
use std::{env, path::Path};

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=proto/orchestrator.proto");
    println!("cargo:rerun-if-changed=build.rs");

    let mut config = Config::new();
    
    // Use protoc_arg for older prost-build versions
    config.protoc_arg("--experimental_allow_proto3_optional");

    println!("Current dir: {:?}", env::current_dir()?);

    let proto_path = Path::new("proto/orchestrator.proto");
    println!(
        "Looking for proto file at: {:?}",
        proto_path.canonicalize()?
    );

    if !proto_path.exists() {
        println!("Proto file not found at: {:?}", proto_path);
        return Err("Proto file not found".into());
    }

    let out_dir = "src/proto";
    config.out_dir(out_dir);

    // Verify protoc version (must be >= 3.12)
    let protoc_version = Command::new("protoc")
        .arg("--version")
        .output()
        .expect("Failed to check protoc version");
    
    if !protoc_version.status.success() {
        return Err("Failed to get protoc version".into());
    }
    
    let version_output = String::from_utf8_lossy(&protoc_version.stdout);
    println!("Protoc version: {}", version_output.trim());

    // Create output directory if needed
    if fs::metadata(out_dir).is_ok() {
        println!("Output directory {} exists.", out_dir);
    } else {
        fs::create_dir_all(out_dir)?;
        println!("Created output directory {}.", out_dir);
    }

    // Compile proto files
    match config.compile_protos(&["proto/orchestrator.proto"], &["proto"]) {
        Ok(_) => println!("Successfully compiled protobuf files."),
        Err(e) => {
            println!("Error compiling protobuf files: {}", e);
            return Err(Box::new(e));
        }
    }

    let generated_file_path = format!("{}/nexus_orchestrator.rs", out_dir);
    println!("Generated file saved to: {}", generated_file_path);

    Ok(())
}
