// ==========================
// FILE: src/model.rs
// ==========================

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

/// ---------- Model ----------
/// Bu model bazadagi `users` jadvalidan foydalanuvchi ma'lumotlarini olish uchun ishlatiladi
/// `FromRow` — SQL qatoridan modelga o'zgartirish uchun kerak
/// `Serialize` va `Deserialize` — JSON uchun kerak (API javoblari va so'rovlar)
/// `Debug` — xatolarni tekshirish uchun
/// `Clone` — kerak, chunki AppState `Clone` bo'lishi kerak
#[derive(Serialize, Deserialize, FromRow, Debug, Clone)]
pub struct UserModel {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub occupation: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// ---------- DTOs ----------
/// Bu model API ga yuboriladigan so'rovlar uchun ishlatiladi
/// Foydalanuvchi ma'lumotlarini olish yoki yaratish uchun ishlatiladi
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserRequest {
    pub name: String,
    pub email: String,
    pub phone: String,
    pub occupation: String,
}

/// ---------- View Model ----------
/// Bu model API uchun javob sifatida ishlatiladi
/// Foydalanuvchi ma'lumotlarini tashqi ko'rinishda (JSON) taqdim etadi
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserView {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub occupation: String,
    pub created_at: DateTime<Utc>,
}

/// ---------- Conversion ----------
/// UserModel dan UserView ga o'zgartirish
/// Bu konversiya API javoblari uchun kerak
impl From<UserModel> for UserView {
    fn from(m: UserModel) -> Self {
        UserView {
            id: m.id,
            name: m.name,
            email: m.email,
            phone: m.phone,
            occupation: m.occupation,
            created_at: m.created_at,
        }
    }
}

/// ---------- Repository (CRUD) ----------
/// Bu model bazaga CRUD operatsiyalarini bajarish uchun ishlatiladi
/// `PgPool` — PostgreSQL ulanish pooli
/// `Result` — xatolarni qaytarish uchun ishlatiladi
impl UserModel {
    /// Create
    pub async fn create(pool: &PgPool, req: UserRequest) -> Result<UserView, sqlx::Error> {
        // Eslatma (optimallashtirish): email uchun unique index yarating (lower(email))
        // va phone uchun ham unique index qo'ying. DB darajasida constraint tezroq va ishonchliroq.
        let model = sqlx::query_as::<_, UserModel>(
            r#"
            INSERT INTO users (name, email, phone, occupation)
            VALUES ($1, $2, $3, $4)
            RETURNING id, name, email, phone, occupation, created_at, updated_at
            "#,
        )
        .bind(&req.name)
        .bind(&req.email)
        .bind(&req.phone)
        .bind(&req.occupation)
        .fetch_one(pool)
        .await?;

        Ok(model.into())
    }

    /// Read by id
    pub async fn find_by_id(pool: &PgPool, id: i64) -> Result<UserView, sqlx::Error> {
        let opt = sqlx::query_as::<_, UserModel>(
            r#"
            SELECT id, name, email, phone, occupation, created_at, updated_at
            FROM users
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;

        let model = opt.ok_or(sqlx::Error::RowNotFound)?;
        Ok(model.into())
    }

    /// Read all
    pub async fn find_all(pool: &PgPool) -> Result<Vec<UserView>, sqlx::Error> {
        let rows = sqlx::query_as::<_, UserModel>(
            r#"
            SELECT id, name, email, phone, occupation, created_at, updated_at
            FROM users
            ORDER BY id
            "#,
        )
        .fetch_all(pool)
        .await?;

        Ok(rows.into_iter().map(Into::into).collect())
    }

    /// Read all (paged) — ixtiyoriy optimizatsiya
    pub async fn find_all_paged(
        pool: &PgPool,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<UserView>, sqlx::Error> {
        let rows = sqlx::query_as::<_, UserModel>(
            r#"
            SELECT id, name, email, phone, occupation, created_at, updated_at
            FROM users
            ORDER BY id
            LIMIT $1 OFFSET $2
            "#,
        )
        .bind(limit.max(1))
        .bind(offset.max(0))
        .fetch_all(pool)
        .await?;
        Ok(rows.into_iter().map(Into::into).collect())
    }

    /// Update (PUT)
    pub async fn update(pool: &PgPool, id: i64, req: UserRequest) -> Result<UserView, sqlx::Error> {
        let model = sqlx::query_as::<_, UserModel>(
            r#"
            UPDATE users
            SET name = $1,
                email = $2,
                phone = $3,
                occupation = $4,
                updated_at = NOW()
            WHERE id = $5
            RETURNING id, name, email, phone, occupation, created_at, updated_at
            "#,
        )
        .bind(&req.name)
        .bind(&req.email)
        .bind(&req.phone)
        .bind(&req.occupation)
        .bind(id)
        .fetch_optional(pool)
        .await?
        .ok_or(sqlx::Error::RowNotFound)?;

        Ok(model.into())
    }

    /// Delete — qaytadi: ta'sirlangan qatorlar soni
    pub async fn delete(pool: &PgPool, id: i64) -> Result<u64, sqlx::Error> {
        let res = sqlx::query(
            r#"
            DELETE FROM users WHERE id = $1
            "#,
        )
        .bind(id)
        .execute(pool)
        .await?;

        Ok(res.rows_affected())
    }
}
