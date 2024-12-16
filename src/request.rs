use serde::Deserialize;

#[derive(Deserialize)]
pub enum PackageRequest {
    Get {
        package_name: String,
        package_version: String,
    },
}
