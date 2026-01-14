use crate::handlers::user::get::handler::get_user_handler;
use crate::handlers::user::register::handler::register_user_handler;
use crate::usecase::get_user_usecase::GetUserUseCase;
use crate::usecase::register_user_usecase::RegisterUserUseCase;
use account_domain::user::repository::UserRepository;
use account_infra::user::postgres_repository::PostgresUserRepository;
use axum::routing::{get, post};
use axum::Router;
use sea_orm::Database;
use std::sync::Arc;

mod handlers;
mod state;
mod usecase;

#[tokio::main]
async fn main() {
    // ログの初期化
    tracing_subscriber::fmt().with_env_filter("info").init();

    tracing::info!("Starting application...");

    let database_url = "postgres://account_user:account_password@localhost:5432/account";
    let db = Database::connect(database_url)
        .await
        .expect("Failed to connect to database");
    let user_repository = Arc::new(PostgresUserRepository::new(db)) as Arc<dyn UserRepository>;

    let get_user_usecase = GetUserUseCase::new(Arc::clone(&user_repository));
    let register_user_usecase = RegisterUserUseCase::new(Arc::clone(&user_repository));

    let state = Arc::new(state::AppState {
        get_user_usecase,
        create_user_usecase: register_user_usecase,
    });

    let app = Router::new()
        .route("/users", post(register_user_handler))
        .route("/users/{user_id}", get(get_user_handler))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
