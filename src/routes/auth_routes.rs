use axum::{
    routing::{get, post},
    Router,
};
use sqlx::PgPool;

use crate::controllers::AuthController;
use crate::utils::JwtConfig;

pub fn auth_routes(pool: PgPool, jwt_config: JwtConfig) -> Router {
    Router::new()
        // Admin authentication routes
        .route("/admin/register", post(AuthController::register_admin))
        .route("/admin/login", post(AuthController::login_admin))
        
        // Student authentication routes
        .route("/student/register", post(AuthController::register_student))
        .route("/student/login", post(AuthController::login_student))
        
        // Mentor authentication routes
        .route("/mentor/register", post(AuthController::register_mentor))
        .route("/mentor/login", post(AuthController::login_mentor))
        
        // OTP routes
        .route("/verify-otp", post(AuthController::verify_otp_login))
        .route("/resend-otp", post(AuthController::resend_otp))
        
        // Common authentication routes
        .route("/refresh", post(AuthController::refresh_token))
        .route("/logout", post(AuthController::logout))
        .route("/me", get(AuthController::get_current_user))
        .route("/verify", post(AuthController::verify_token_endpoint))
        
        .with_state((pool, jwt_config))
}
