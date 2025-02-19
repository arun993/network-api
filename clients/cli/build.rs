use prost_build::Config;
use std::error::Error;
use std::fs;
use std::process::Command;
use std::{env, path::Path};

fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-changed=proto/orchestrator.proto");
    println!("cargo:rerun-if-changed=build.rs");

    let mut config = Config::new();
    
    // CORRECTED: Use prost-build's dedicated method for proto3 optional
    config.enable_proto3_optional(true);

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

    // Check protoc installation
    let output = Command::new("which")
        .arg("protoc")
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        println!("protoc is installed and accessible.");
    } else {
        println!("Error: protoc is not installed or not in PATH.");
        return Err("protoc not found".into());
    }

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

    if fs::metadata(&generated_file_path).is_ok() {
        println!("Generated file exists.");
    } else {
        println!("Error: Generated file does not exist.");
    }

    Ok(())
}
