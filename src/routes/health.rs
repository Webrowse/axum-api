use axum::{ Json, http::StatusCode };

pub struct HealthData {
    status: StatusCode,
}

pub async fn health() -> Json<HealthData> {
    let health_data = HealthData { status: StatusCode::OK};
    Json(health_data)
}