use axum::extract::Path;
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

#[derive(Serialize)]
struct UserResponse {
    id: u64,
    name: String,
    is_admin: bool,
}

// ハンドラ（非同期関数）
async fn get_user_handler(Path(user_id): Path<u64>) -> Json<UserResponse> {
    // 本来はここでリポジトリを呼び出す
    let user = User::new(user_id, "Rustacean", UserRole::User);

    Json(UserResponse {
        id: user.id.0,
        is_admin: user.is_admin(),
        name: user.name,
    })
}

#[tokio::main]
async fn main() {
    // ルーターの設定
    let app = Router::new()
        .route("/users/{use_id}", get(get_user_handler));

    // サーバーの起動
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
