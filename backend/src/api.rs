use crate::registry;
use rocket::get;
use rocket::serde::json::Json;

#[get("/get_repositories")]
pub async fn get_repositories() -> Result<Json<Vec<String>>, String> {
    registry::get_repositories()
        .await
        .map_err(|e| format!("error :(\n{}", e))
        .map(Json)
}
