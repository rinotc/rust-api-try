use crate::usecase::get_user_usecase::GetUserUseCase;
use axum::extract::FromRef;
use std::sync::Arc;
use crate::usecase::register_user_usecase::RegisterUserUseCase;

pub struct AppState {
    pub get_user_usecase: GetUserUseCase,
    pub create_user_usecase: RegisterUserUseCase
}

impl FromRef<Arc<AppState>> for GetUserUseCase {
    fn from_ref(state: &Arc<AppState>) -> Self {
        state.get_user_usecase.clone()
    }
}

impl FromRef<Arc<AppState>> for RegisterUserUseCase {
    fn from_ref(state: &Arc<AppState>) -> Self {
        state.create_user_usecase.clone()
    }
}