use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};
use std::sync::Arc;
use uuid::Uuid;

use crate::services::{EmailService, EmailTemplate};
use crate::utils::AuthError;

// ─── Request / Response types ───

#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct SubscribeRequest {
    pub email: String,
    pub name: Option<String>,
}

#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct UnsubscribeRequest {
    pub email: String,
}

#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct SendNewsletterRequest {
    pub subject: String,
    pub content: String,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct SubscriberResponse {
    pub id: Uuid,
    pub email: String,
    pub name: Option<String>,
    pub is_active: bool,
}

// ─── Internal DB row ───

#[derive(Debug, FromRow)]
struct Subscriber {
    id: Uuid,
    email: String,
    name: Option<String>,
    is_active: bool,
}

// ─── Service ───

pub struct NewsletterService {
    pool: PgPool,
    email_service: Option<Arc<EmailService>>,
}

impl NewsletterService {
    pub fn new(pool: PgPool, email_service: Option<Arc<EmailService>>) -> Self {
        Self { pool, email_service }
    }

    pub async fn subscribe(&self, req: SubscribeRequest) -> Result<SubscriberResponse, AuthError> {
        let row = sqlx::query_as::<_, Subscriber>(
            r#"
            INSERT INTO newsletter_subscribers (email, name)
            VALUES ($1, $2)
            ON CONFLICT (email) DO UPDATE
                SET is_active = TRUE, unsubscribed_at = NULL, name = COALESCE($2, newsletter_subscribers.name)
            RETURNING id, email, name, is_active
            "#,
        )
        .bind(&req.email)
        .bind(&req.name)
        .fetch_one(&self.pool)
        .await
        .map_err(|_| AuthError::InternalServerError)?;

        Ok(SubscriberResponse {
            id: row.id,
            email: row.email,
            name: row.name,
            is_active: row.is_active,
        })
    }

    pub async fn unsubscribe(&self, req: UnsubscribeRequest) -> Result<(), AuthError> {
        let result = sqlx::query(
            "UPDATE newsletter_subscribers SET is_active = FALSE, unsubscribed_at = NOW() WHERE email = $1",
        )
        .bind(&req.email)
        .execute(&self.pool)
        .await
        .map_err(|_| AuthError::InternalServerError)?;

        if result.rows_affected() == 0 {
            return Err(AuthError::UserNotFound);
        }

        Ok(())
    }

    pub async fn send_newsletter(&self, req: SendNewsletterRequest) -> Result<u64, AuthError> {
        let svc = self
            .email_service
            .as_ref()
            .ok_or(AuthError::InternalServerError)?;

        let subscribers = sqlx::query_as::<_, Subscriber>(
            "SELECT id, email, name, is_active FROM newsletter_subscribers WHERE is_active = TRUE",
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|_| AuthError::InternalServerError)?;

        let base_url = std::env::var("BASE_URL")
            .unwrap_or_else(|_| "http://localhost:3000".to_string());

        let mut sent = 0u64;
        for sub in &subscribers {
            let display_name = sub.name.as_deref().unwrap_or("Subscriber");
            let unsubscribe_url = format!("{}/newsletter/unsubscribe?email={}", base_url, sub.email);
            let html = EmailTemplate::newsletter_email(
                display_name,
                &req.subject,
                &req.content,
                &unsubscribe_url,
            );
            let text = format!(
                "Hello {},\n\n{}\n\nUnsubscribe: {}\n\nBuidlFlow Team",
                display_name, req.content, unsubscribe_url
            );

            match svc.send_email(&sub.email, &req.subject, &html, &text).await {
                Ok(_) => sent += 1,
                Err(e) => tracing::error!("Failed to send newsletter to {}: {:?}", sub.email, e),
            }
        }

        Ok(sent)
    }
}
