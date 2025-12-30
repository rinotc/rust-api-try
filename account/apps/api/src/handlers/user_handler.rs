use crate::usecase::get_user_usecase::{GetUserInput, GetUserOutput, GetUserUseCase};
use account_libs::usecase::UseCase;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct UserResponse {
    id: String,
    name: String,
    is_admin: bool,
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
        GetUserOutput::NotFound(_) => Err(StatusCode::NOT_FOUND),
        GetUserOutput::Error(_) => Err(StatusCode::INTERNAL_SERVER_ERROR)
    }
}