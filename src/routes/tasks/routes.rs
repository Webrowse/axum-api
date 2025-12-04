use axum::{Json, extract::{State, Path}};
use uuid::Uuid;
use crate::state::AppState;
use crate::routes::middleware_auth::JwtUser;
use crate::tasks::dto::{CreateTask, UpdateTask};
use crate::tasks::queries;

pub async fn create(
    State(state): State<AppState>,
    JwtUser(user_id): JwtUser,
    Json(body): Json<CreateTask>,
) -> Json<impl serde::Serialize> {
    let t = queries::create_task(&state.db, user_id, &body.title).await.unwrap();
    Json(t)
}

pub async fn list(
    State(state): State<AppState>,
    JwtUser(user_id): JwtUser,
) -> Json<impl serde::Serialize> {
    let tasks = queries::list_tasks(&state.db, user_id).await.unwrap();
    Json(tasks)
}

pub async fn update(
    State(state): State<AppState>,
    JwtUser(user_id): JwtUser,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateTask>,
) -> Json<impl serde::Serialize> {
    let t = queries::update_task(&state.db, user_id, id, body.title, body.done).await.unwrap();
    Json(t)
}

pub async fn delete(
    State(state): State<AppState>,
    JwtUser(user_id): JwtUser,
    Path(id): Path<Uuid>,
) -> Json<impl serde::Serialize> {
    queries::delete_task(&state.db, user_id, id).await.unwrap();
    Json({ "deleted"; true })
}
