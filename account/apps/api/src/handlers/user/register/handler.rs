use crate::handlers::user::register::request::RegisterUserRequest;
use crate::handlers::user::register::response::RegisterUserResponse;
use crate::handlers::request_validator::validate_request;
use crate::usecase::register_user_usecase::{
    RegisterUserInput, RegisterUserOutput, RegisterUserUseCase,
};
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;

pub async fn register_user_handler(
    State(usecase): State<RegisterUserUseCase>,
    Json(payload): Json<RegisterUserRequest>,
) -> Result<Json<RegisterUserResponse>, StatusCode> {
    validate_request(payload, |validated| async move {
        let input = RegisterUserInput {
            name: validated.name,
            email: validated.email,
        };

        match usecase.execute(input).await {
            RegisterUserOutput::Success(user) => Ok(Json(RegisterUserResponse {})),
            RegisterUserOutput::Error => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    }).await
}
