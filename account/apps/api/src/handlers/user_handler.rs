use crate::usecase::create_user_usecase::{CreateUserInput, CreateUserOutput, CreateUserUseCase};
use crate::usecase::get_user_usecase::{GetUserInput, GetUserOutput, GetUserUseCase};
use account_domain::user::user::User;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct UserResponse {
    id: String,
    name: String,
    is_admin: bool,
}
impl UserResponse {
    pub fn from(user: User) -> Self {
        Self {
            id: user.id.0.to_string(),
            name: user.name.clone(),
            is_admin: user.is_admin(),
        }
    }
}

// ハンドラ（非同期関数）
pub async fn get_user_handler(
    State(usecase): State<GetUserUseCase>,
    Path(user_id): Path<String>
) -> Result<Json<UserResponse>, StatusCode> {
    let input = GetUserInput { user_id: user_id.parse().unwrap() };

    match usecase.execute(input).await {
        GetUserOutput::Success(user) => Ok(
            Json(UserResponse {
                id: user.id.0.to_string(),
                is_admin: user.is_admin(),
                name: user.name,
            })
        ),
        GetUserOutput::NotFound => Err(StatusCode::NOT_FOUND),
        GetUserOutput::Error => Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

#[derive(Deserialize)]
pub struct CreateUserRequest {
    name: String
}
pub async fn create_user_handler(
    State(usecase): State<CreateUserUseCase>,
    Json(payload): Json<CreateUserRequest>
) -> Result<Json<UserResponse>, StatusCode> {
    let input = CreateUserInput { name: payload.name };

    match usecase.execute(input).await {
        CreateUserOutput::Success(user) => Ok(Json(UserResponse::from(user))),
        CreateUserOutput::Error => Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}