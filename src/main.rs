mod domain;
mod usecase;
mod libs;
mod api;
mod infra;

use crate::api::state::AppState;
use crate::api::user_handler::get_user_handler;
use crate::domain::user_repository::UserRepository;
use crate::usecase::get_user_usecase::GetUserUseCase;
use axum::routing::get;
use axum::Router;
use std::sync::Arc;
use crate::infra::user_repository::InMemoryUserRepository;

#[tokio::main]
async fn main() {
    // 1. インフラ層の生成
    let user_repository = Arc::new(InMemoryUserRepository) as Arc<dyn UserRepository>;

    // 2. ユースケース層の生成
    let get_user_usecase = GetUserUseCase::new(user_repository);

    // 2. アプリケーション状態
    let state = Arc::new(AppState {
        get_user_usecase
    });

    // ルーターの設定
    let app = Router::new()
        .route("/users/{use_id}", get(get_user_handler))
        .with_state(state);

    // サーバーの起動
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
