use npm_package::AsyncNpmClient;
use tokio;

#[tokio::main]
async fn main() {
    let package_name = std::env::args().nth(1).unwrap_or("is-wsl".to_string());
    let package_name = &package_name.as_str();

    let client = AsyncNpmClient::new();
    let is_wsl_package = client.get(package_name).await.unwrap();

    println!(
        "Description of {} from the npm registry: {}",
        package_name,
        is_wsl_package.description.unwrap()
    );
}
