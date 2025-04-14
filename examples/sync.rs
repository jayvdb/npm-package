use npm_package::SyncNpmClient;

fn main() {
    let package_name = std::env::args()
        .nth(1)
        .unwrap_or("is-interactive".to_string());
    let package_name = &package_name.as_str();

    let client = SyncNpmClient::new();
    let package = client.get(package_name).unwrap();
    let version_history = package.versions.keys().collect::<Vec<_>>();

    println!(
        "All {} releases on npm: {:?}",
        package_name, version_history
    );
}
