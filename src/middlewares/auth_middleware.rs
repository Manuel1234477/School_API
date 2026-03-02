use axum::{
    extract::Request,
    http::header,
    middleware::Next,
    response::Response,
};

use crate::utils::{AuthError, JwtConfig};

/// Middleware to extract and validate JWT token from Authorization header
#[allow(dead_code)]
pub async fn auth_middleware(
    mut request: Request,
    next: Next,
) -> Result<Response, AuthError> {
    let auth_header = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .ok_or(AuthError::Unauthorized)?;

    let token = crate::utils::extract_token_from_header(auth_header)
        .ok_or(AuthError::InvalidToken)?;

    // Get JWT config from request extensions (should be set in main.rs)
    let jwt_config = request
        .extensions()
        .get::<JwtConfig>()
        .cloned()
        .ok_or(AuthError::InternalServerError)?;

    // Verify token
    let claims = crate::utils::verify_token(&token, &jwt_config)
        .map_err(|_| AuthError::InvalidToken)?;

    // Ensure it's an access token
    if claims.token_type != "access" {
        return Err(AuthError::InvalidToken);
    }

    // Insert claims into request extensions for use in handlers
    request.extensions_mut().insert(claims);

    Ok(next.run(request).await)
}

/// Middleware to check if user has required role
#[allow(dead_code)]
pub fn role_middleware(required_roles: Vec<&'static str>) -> impl Fn(Request, Next) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Response, AuthError>> + Send>> + Clone {
    move |request: Request, next: Next| {
        let required_roles = required_roles.clone();
        Box::pin(async move {
            let claims = request
                .extensions()
                .get::<crate::models::Claims>()
                .cloned()
                .ok_or(AuthError::Unauthorized)?;

            if !required_roles.contains(&claims.role.as_str()) {
                return Err(AuthError::Forbidden);
            }

            Ok(next.run(request).await)
        })
    }
}
