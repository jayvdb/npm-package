//! Loads a package.json file and prints the dependencies and dev-dependencies with their descriptions.
use npm_package::{LocalPackageVersion, SyncNpmClient};

fn main() {
    let filename_arg = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Usage: cargo run --example read-package-json -- <filename>");
        std::process::exit(1);
    });
    let file = std::fs::File::open(filename_arg).unwrap();
    let package_json: LocalPackageVersion = serde_json::from_reader(file).unwrap();

    let client = SyncNpmClient::new();

    println!("Dependencies:");
    for (dep, _version) in &package_json.dependencies.unwrap_or_default() {
        match client.get(dep) {
            Ok(package) => println!("{}: {}", dep, package.description.unwrap_or_default()),
            Err(err) => println!("{}: not loaded: {}", dep, err),
        }
    }

    println!("\nDev-Dependencies:");
    for (dep, _version) in &package_json.dev_dependencies.unwrap_or_default() {
        match client.get(dep) {
            Ok(package) => println!("{}: {}", dep, package.description.unwrap_or_default()),
            Err(err) => println!("{}: not loaded: {}", dep, err),
        }
    }
}
