-- Add migration script here
BEGIN;

-- Agar serverda mavjud bo'lsa, citext uchun:
-- Bu kengaytma case-insensitive matn ustunlarini qo'shadi.
-- Agar siz email yoki boshqa matn ustunlarini case-insensitive qilishni xohlasangiz, bu kengaytmani o'rnatishingiz kerak.
-- Agar siz case-sensitive matn ustunlarini ishlatmoqchi bo'lsangiz, bu kengaytma shart emas.
-- Misol uchun, email ustunini case-insensitive qilish uchun citext kengaytmasini o'rnatish kerak.
-- Bu kengaytma PostgreSQL 9.1 va undan yuqori versiyalarida mavjud.
CREATE EXTENSION IF NOT EXISTS citext;

-- Agar serverda mavjud bo'lsa, cron uchun:
-- Bu kengaytma rejalashtirilgan vazifalarni bajarish uchun ishlatiladi.
-- Agar siz ma'lum vaqt oralig'ida vazifalarni bajarishni xohlasangiz, bu kengaytmani o'rnatishingiz kerak.
-- Misol uchun, har kuni ma'lum vaqtda ma'lumotlarni yangilash yoki tozalash uchun cron kengaytmasini ishlatishingiz mumkin.
-- Bu kengaytma PostgreSQL 9.6 va undan yuqori versiyalarida mavjud.
-- Agar siz rejalashtirilgan vazifalarni bajarishni xohlamasangiz, bu kengaytma shart emas.
-- Bu kengaytma PostgreSQL serverida o'rnatilgan bo'lishi kerak.
CREATE EXTENSION IF NOT EXISTS pg_cron;

-- Agar serverda mavjud bo'lsa, pgcrypto uchun:
-- Bu kengaytma kriptografik funksiyalarni qo'shadi, masalan, UUID yaratish.
-- Bu kengaytma ko'pincha xavfsiz identifikatorlar yaratish uchun ishlatiladi.
-- Agar siz UUID yoki boshqa kriptografik funksiyalarni ishlatmoqchi bo'lsangiz, bu kengaytmani o'rnatishingiz kerak.
-- Agar siz faqat oddiy identifikatorlar (masalan, SERIAL) ishlatmoqchi bo'lsangiz, bu kengaytma shart emas.
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    email VARCHAR(100) UNIQUE NOT NULL,
    phone VARCHAR(15) UNIQUE NOT NULL,
    occupation VARCHAR(50),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);

-- updated_at ni avtomatik yangilash (ixtiyoriy, lekin foydali)
CREATE OR REPLACE FUNCTION set_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS trg_users_updated_at ON users;
CREATE TRIGGER trg_users_updated_at
BEFORE UPDATE ON users
FOR EACH ROW EXECUTE FUNCTION set_updated_at();
