use crate::domain::user::UserId;
use crate::libs::usecase::UseCase;
use crate::usecase::get_user_usecase::{GetUserInput, GetUserOutput, GetUserUseCase};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use serde::Serialize;
use std::sync::Arc;

#[derive(Serialize)]
pub struct UserResponse {
    id: u64,
    name: String,
    is_admin: bool,
}

pub struct AppState {
    pub get_user_usecase: GetUserUseCase
}

// ハンドラ（非同期関数）
pub async fn get_user_handler(
    State(state): State<Arc<AppState>>,
    Path(user_id): Path<u64>
) -> Result<Json<UserResponse>, axum::http::StatusCode> {
    let id = UserId(user_id);
    let input = GetUserInput { user_id: id };

    match state.get_user_usecase.execute(input) {
        GetUserOutput::Success(user) => Ok(
            Json(UserResponse {
                id: user.id.0,
                is_admin: user.is_admin(),
                name: user.name,
            })
        ),
        GetUserOutput::NotFound(_) => Err(StatusCode::NOT_FOUND),
        GetUserOutput::Error(_) => Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}