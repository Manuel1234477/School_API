use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::post,
    Json, Router,
};

use crate::services::{NewsletterService, SendNewsletterRequest, SubscribeRequest, UnsubscribeRequest};
use crate::state::AppState;
use crate::utils::AuthError;

pub fn newsletter_routes(state: AppState) -> Router {
    Router::new()
        .route("/newsletter/subscribe", post(subscribe))
        .route("/newsletter/unsubscribe", post(unsubscribe))
        .route("/newsletter/send", post(send_newsletter))
        .with_state(state)
}

#[utoipa::path(
    post,
    path = "/newsletter/subscribe",
    tag = "Newsletter",
    request_body = SubscribeRequest,
    responses(
        (status = 201, description = "Subscribed successfully", body = SubscriberResponse),
        (status = 500, description = "Internal server error")
    )
)]
async fn subscribe(
    State(state): State<AppState>,
    Json(req): Json<SubscribeRequest>,
) -> Result<impl IntoResponse, AuthError> {
    let svc = NewsletterService::new(state.pool.clone(), state.email_service.clone());
    let result = svc.subscribe(req).await?;
    Ok((StatusCode::CREATED, Json(result)))
}

#[utoipa::path(
    post,
    path = "/newsletter/unsubscribe",
    tag = "Newsletter",
    request_body = UnsubscribeRequest,
    responses(
        (status = 204, description = "Unsubscribed successfully"),
        (status = 404, description = "Email not found")
    )
)]
async fn unsubscribe(
    State(state): State<AppState>,
    Json(req): Json<UnsubscribeRequest>,
) -> Result<impl IntoResponse, AuthError> {
    let svc = NewsletterService::new(state.pool.clone(), state.email_service.clone());
    svc.unsubscribe(req).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(
    post,
    path = "/newsletter/send",
    tag = "Newsletter",
    request_body = SendNewsletterRequest,
    responses(
        (status = 200, description = "Newsletter sent", body = serde_json::Value),
        (status = 500, description = "Internal server error")
    ),
    security(("bearer_auth" = []))
)]
async fn send_newsletter(
    State(state): State<AppState>,
    Json(req): Json<SendNewsletterRequest>,
) -> Result<impl IntoResponse, AuthError> {
    let svc = NewsletterService::new(state.pool.clone(), state.email_service.clone());
    let sent = svc.send_newsletter(req).await?;
    Ok((StatusCode::OK, Json(serde_json::json!({ "sent": sent }))))
}
