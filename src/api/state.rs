use std::sync::Arc;
use axum::extract::FromRef;
use crate::usecase::get_user_usecase::GetUserUseCase;

pub struct AppState {
    pub get_user_usecase: GetUserUseCase
}

impl FromRef<Arc<AppState>> for GetUserUseCase {
    fn from_ref(state: &Arc<AppState>) -> Self {
        state.get_user_usecase.clone()
    }
}