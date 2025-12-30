use crate::usecase::get_user_usecase::GetUserUseCase;
use axum::extract::FromRef;
use std::sync::Arc;

pub struct AppState {
    pub get_user_usecase: GetUserUseCase
}

impl FromRef<Arc<AppState>> for GetUserUseCase {
    fn from_ref(state: &Arc<AppState>) -> Self {
        state.get_user_usecase.clone()
    }
}