// ==========================
// FILE: src/routes.rs
// ==========================

use crate::AppState;
use crate::controller::{create_user, delete_user, get_user_by_id, update_user, users};
use axum::{Router, routing::get}; // Axum router va marshrutlar uchun

/// Users router — keyinchalik main.rs ichida `.nest("/api", user_routes())` qiling
pub fn user_routes() -> Router<AppState> {
    Router::new()
        .route("/users", get(users).post(create_user))
        .route(
            "/users/{id}",
            get(get_user_by_id).put(update_user).delete(delete_user),
        )
}
