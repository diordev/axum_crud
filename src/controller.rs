// ==========================
// FILE: src/controller.rs
// ==========================

use crate::AppState;
use crate::errors::ApiError; // mavjud bo'lsa
use crate::model::{UserModel, UserRequest, UserView};
use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use serde::Deserialize; // AppState { pool: PgPool }

/// GET /users — barchasi (ixtiyoriy paginate)
#[derive(Debug, Deserialize)]
pub struct Pager {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

/// Barcha foydalanuvchilarni olish
pub async fn users(
    State(state): State<AppState>,
    Query(p): Query<Pager>,
) -> Result<Json<Vec<UserView>>, ApiError> {
    let limit = p.limit.unwrap_or(100);
    let offset = p.offset.unwrap_or(0);

    let list = if p.limit.is_some() || p.offset.is_some() {
        UserModel::find_all_paged(&state.pool, limit, offset).await?
    } else {
        UserModel::find_all(&state.pool).await?
    };
    // ⚠️ Debug: response body’ni ko‘rish (string sifatida)
    match serde_json::to_string(&list) {
        Ok(s) => tracing::debug!(%s, "users response"),
        Err(e) => tracing::debug!(?e, "serialize failed"),
    }

    Ok(Json(list))
}

/// GET /users/:id — bitta foydalanuvchi
pub async fn get_user_by_id(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<UserView>, ApiError> {
    let user = UserModel::find_by_id(&state.pool, id).await?;
    Ok(Json(user))
}

/// POST /users — yaratish
pub async fn create_user(
    State(state): State<AppState>,
    Json(body): Json<UserRequest>,
) -> Result<(StatusCode, Json<UserView>), ApiError> {
    // Oddiy validatsiya (tez, lekin asosiy constraintlarni DBga qo'ying)
    if body.name.trim().is_empty() {
        return Err(ApiError::BadRequest("name required".into()));
    }
    if body.email.trim().is_empty() {
        return Err(ApiError::BadRequest("email required".into()));
    }

    let user = UserModel::create(&state.pool, body).await?;
    Ok((StatusCode::CREATED, Json(user)))
}

/// PUT /users/:id — yangilash
pub async fn update_user(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(body): Json<UserRequest>,
) -> Result<Json<UserView>, ApiError> {
    let user = UserModel::update(&state.pool, id, body).await?;
    Ok(Json(user))
}

/// DELETE /users/:id
pub async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<StatusCode, ApiError> {
    let affected = UserModel::delete(&state.pool, id).await?;
    if affected == 0 {
        Err(ApiError::NotFound)
    } else {
        Ok(StatusCode::NO_CONTENT)
    }
}
