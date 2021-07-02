use serde::Deserialize;
use std::error::Error;

pub async fn get_repositories() -> Result<Vec<String>, Box<dyn Error>> {
    /// ```json
    /// {
    ///   "repositories": [
    ///     <name>,
    ///     ...
    ///   ]
    /// }
    /// ```
    #[derive(Deserialize)]
    struct RegistryResponse {
        repositories: Vec<String>,
    }

    let response = reqwest::get("http://localhost:5000/v2/_catalog")
        .await?
        .json::<RegistryResponse>()
        .await?;
    Ok(response.repositories)
}
