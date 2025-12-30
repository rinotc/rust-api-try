mod domain;
mod usecase;
mod libs;

use axum::extract::{Path, State};
use axum::routing::get;
use axum::{Json, Router};
use serde::Serialize;
use std::sync::Arc;
use crate::domain::domain_error::DomainError;
use crate::domain::user::UserId;
use crate::domain::user_repository::InMemoryUserRepository;
use crate::domain::user_repository::UserRepository;


#[derive(Serialize)]
struct UserResponse {
    id: u64,
    name: String,
    is_admin: bool,
}

// ハンドラ（非同期関数）
async fn get_user_handler(
    State(repository): State<Arc<dyn UserRepository>>,
    Path(user_id): Path<u64>
) -> Result<Json<UserResponse>, axum::http::StatusCode> {
    let id = UserId(user_id);

    repository.find_by_id(id)
        .map(|user| {
            Json(UserResponse {
                id: user.id.0,
                is_admin: user.is_admin(),
                name: user.name,
            })
        })
        .map_err(|e| match e {
            DomainError::NotFound => axum::http::StatusCode::NOT_FOUND,
            _ => axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        })
}

#[tokio::main]
async fn main() {
    let user_repository = Arc::new(InMemoryUserRepository) as Arc<dyn UserRepository>;

    // ルーターの設定
    let app = Router::new()
        .route("/users/{use_id}", get(get_user_handler))
        .with_state(user_repository);

    // サーバーの起動
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
