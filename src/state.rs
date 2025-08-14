// ==========================
// FILE: src/state.rs
// ==========================

use anyhow::{Context, Result}; //  Qulay xatolar zanjiri va Result turi
use dotenvy::dotenv; //  .env o'qish uchun
use sqlx::{PgPool, postgres::PgPoolOptions}; //   PostgreSQL pool builder va pool turi

/// App holati: bitta PgPool
/// `Clone` kerak, chunki Axum state'ni clone qilib worker/handlerlarga tarqatishi mumkin.
#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
}

impl AppState {
    /// Muhit o'zgaruvchilaridan DATABASE_URL ni olib, PgPool yaratadi.
    /// Bu funksiya *faqat* pool qaytarishga qaratilgan (soddalik uchun boshqaruvlarni minimal qildik).
    pub async fn from_env() -> Result<Self> {
        // .env fayl bo'lsa, yuklab oling (masalan, DATABASE_URL)
        dotenv().ok();

        // DATABASE_URL ni o'qish — topilmasa mazmunli xato bilan qaytamiz
        let db_url = std::env::var("DATABASE_URL").context("DATABASE_URL is missing")?;

        // PgPool yaratish — 10 ta ulanishgacha; connect() real ulanishni o'rnatadi
        let pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(&db_url)
            .await
            .context("failed to connect to Postgres")?;

        // Ixtiyoriy: ulanishni tekshirish uchun oddiy query (masalan, SELECT 1)
        // Agar bazaga ulanish muammo bo'lsa, bu yerda xato olamiz.
        sqlx::query_scalar::<_, i32>("SELECT 1")
            .fetch_one(&pool)
            .await
            .context("database sanity check failed")?;

        Ok(Self { pool })
    }
}
