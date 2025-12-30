use std::net::TcpListener;
use crate::usecase::get_user_usecase::GetUserUseCase;
use account_domain::user::user_repository::UserRepository;
use account_infra::user::in_memory_user_repository::InMemoryUserRepository;
use std::sync::Arc;
use axum::Router;
use axum::routing::get;
use crate::handlers::user_handler::get_user_handler;

mod handlers;
mod state;
mod usecase;

#[tokio::main]
async fn main() {
    let user_repository = Arc::new(InMemoryUserRepository) as Arc<dyn UserRepository>;

    let get_user_usecase = GetUserUseCase::new(user_repository);

    let state = Arc::new(state::AppState { get_user_usecase });

    let app = Router::new()
        .route("/users/{user_id}", get(get_user_handler))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
