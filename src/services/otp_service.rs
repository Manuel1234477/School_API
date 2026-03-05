use chrono::{Duration, Utc};
use rand::Rng;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::utils::AuthError;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct OtpRecord {
    pub id: Uuid,
    pub user_id: Uuid,
    pub otp_code: String,
    pub otp_type: String, // "login", "password_reset", "email_verification"
    pub is_used: bool,
    pub created_at: chrono::DateTime<Utc>,
    pub expires_at: chrono::DateTime<Utc>,
}

pub struct OtpService;

impl OtpService {
    /// Generate a random 6-digit OTP
    pub fn generate_otp() -> String {
        let mut rng = rand::thread_rng();
        let otp: u32 = rng.gen_range(100000..999999);
        otp.to_string()
    }

    /// Create and store OTP in database
    pub async fn create_otp(
        pool: &PgPool,
        user_id: Uuid,
        otp_type: &str,
        expiry_minutes: i64,
    ) -> Result<String, AuthError> {
        let otp_code = Self::generate_otp();
        let now = Utc::now();
        let expires_at = now + Duration::minutes(expiry_minutes);

        sqlx::query(
            "INSERT INTO otp_records (id, user_id, otp_code, otp_type, is_used, created_at, expires_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7)"
        )
        .bind(Uuid::new_v4())
        .bind(user_id)
        .bind(&otp_code)
        .bind(otp_type)
        .bind(false)
        .bind(now)
        .bind(expires_at)
        .execute(pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to create OTP: {}", e);
            AuthError::DatabaseError(e.to_string())
        })?;

        Ok(otp_code)
    }

    /// Verify OTP
    pub async fn verify_otp(
        pool: &PgPool,
        user_id: Uuid,
        otp_code: &str,
        otp_type: &str,
    ) -> Result<bool, AuthError> {
        let now = Utc::now();

        let record = sqlx::query_as::<_, OtpRecord>(
            "SELECT id, user_id, otp_code, otp_type, is_used, created_at, expires_at 
             FROM otp_records 
             WHERE user_id = $1 AND otp_code = $2 AND otp_type = $3 AND is_used = false
             ORDER BY created_at DESC
             LIMIT 1"
        )
        .bind(user_id)
        .bind(otp_code)
        .bind(otp_type)
        .fetch_optional(pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to fetch OTP: {}", e);
            AuthError::DatabaseError(e.to_string())
        })?
        .ok_or(AuthError::InvalidToken)?;

        // Check if OTP is expired
        if now > record.expires_at {
            return Err(AuthError::InvalidToken);
        }

        // Mark OTP as used
        sqlx::query("UPDATE otp_records SET is_used = true WHERE id = $1")
            .bind(record.id)
            .execute(pool)
            .await
            .map_err(|e| {
                tracing::error!("Failed to mark OTP as used: {}", e);
                AuthError::DatabaseError(e.to_string())
            })?;

        Ok(true)
    }

    /// Get latest OTP for user
    pub async fn get_latest_otp(
        pool: &PgPool,
        user_id: Uuid,
        otp_type: &str,
    ) -> Result<Option<OtpRecord>, AuthError> {
        sqlx::query_as::<_, OtpRecord>(
            "SELECT id, user_id, otp_code, otp_type, is_used, created_at, expires_at 
             FROM otp_records 
             WHERE user_id = $1 AND otp_type = $2 AND is_used = false
             ORDER BY created_at DESC
             LIMIT 1"
        )
        .bind(user_id)
        .bind(otp_type)
        .fetch_optional(pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to fetch OTP: {}", e);
            AuthError::DatabaseError(e.to_string())
        })
    }

    /// Clean up expired OTPs
    pub async fn cleanup_expired_otps(pool: &PgPool) -> Result<u64, AuthError> {
        let now = Utc::now();

        let result = sqlx::query("DELETE FROM otp_records WHERE expires_at < $1")
            .bind(now)
            .execute(pool)
            .await
            .map_err(|e| {
                tracing::error!("Failed to cleanup expired OTPs: {}", e);
                AuthError::DatabaseError(e.to_string())
            })?;

        Ok(result.rows_affected())
    }

    /// Resend OTP (invalidate old one and create new)
    pub async fn resend_otp(
        pool: &PgPool,
        user_id: Uuid,
        otp_type: &str,
    ) -> Result<String, AuthError> {
        // Invalidate old OTPs
        sqlx::query(
            "UPDATE otp_records SET is_used = true 
             WHERE user_id = $1 AND otp_type = $2 AND is_used = false"
        )
        .bind(user_id)
        .bind(otp_type)
        .execute(pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to invalidate old OTPs: {}", e);
            AuthError::DatabaseError(e.to_string())
        })?;

        // Create new OTP
        Self::create_otp(pool, user_id, otp_type, 10).await
    }
}
