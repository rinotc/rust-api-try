use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use crate::handlers::user::user_response::UserResponse;
use crate::usecase::get_user_usecase::{GetUserInput, GetUserOutput, GetUserUseCase};

pub async fn get_user_handler(
    State(usecase): State<GetUserUseCase>,
    Path(user_id): Path<String>
) -> Result<Json<UserResponse>, StatusCode> {
    let input = GetUserInput {
        user_id: user_id.parse().unwrap(),
    };

    match usecase.execute(input).await {
        GetUserOutput::Success(user) => Ok(Json(UserResponse::from(user))),
        GetUserOutput::NotFound => Err(StatusCode::NOT_FOUND),
        GetUserOutput::Error => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}