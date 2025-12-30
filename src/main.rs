use std::sync::Arc;
use axum::extract::{Path, State};
use axum::{Json, Router};
use axum::routing::get;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserId(u64);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UserRole {
    Admin,
    User,
}

#[derive(Debug, Clone)]
pub struct User {
    pub id: UserId,
    pub name: String,
    pub role: UserRole,
}

impl User {
    // static メソッド
    pub fn new(id: u64, name: &str, role: UserRole) -> Self {
        Self { id: UserId(id), name: name.to_string(), role }
    }

    // 通常のメソッド
    pub fn is_admin(&self) -> bool {
        matches!(self.role, UserRole::Admin)
    }
}

#[derive(Debug)]
pub enum DomainError {
    NotFound,
    InfrastructureError,
}

pub trait  UserRepository: Send + Sync {
    fn find_by_id(&self, id: UserId) -> Result<User, DomainError>;
}

pub struct InMemoryUserRepository;

impl UserRepository for InMemoryUserRepository {
    fn find_by_id(&self, id: UserId) -> Result<User, DomainError> {
        match id.0 {
            1 => Ok(User::new(id.0, "name", UserRole::Admin)),
            2 => Ok(User::new(id.0, "name", UserRole::User)),
            3 => Err(DomainError::NotFound),
            _ => Err(DomainError::InfrastructureError),
        }
    }
}

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
