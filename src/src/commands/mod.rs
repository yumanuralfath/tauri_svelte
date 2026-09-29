mod app;
mod auth;

use tauri::{ipc::Invoke, Runtime};

pub fn handler<R: Runtime>() -> impl Fn(Invoke<R>) -> bool + Send + Sync + 'static {
    tauri::generate_handler![app::get_app_info, auth::authenticate]
}
