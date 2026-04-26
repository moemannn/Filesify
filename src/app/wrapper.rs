use std::sync::OnceLock;
use crate::app::AppState;

static APP_STATE: OnceLock<std::sync::Mutex<AppState>> = OnceLock::new();

pub fn app_state() -> &'static std::sync::Mutex<AppState> {
    APP_STATE.get_or_init(|| {
        std::sync::Mutex::new(AppState::new())
    })
}