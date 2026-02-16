use serde::{Serialize, Deserialize};
use axum::{
    Json,
    response::{IntoResponse, Response},
    async_trait,
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
};
use crate::auth::tokens::validate_jwt;
use axum_extra::{
    headers::{Authorization, authorization::Bearer},
    extract::{CookieJar, TypedHeader}
};
use crate::context::environment::{Environment, Singleton};


#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Claims {
    pub username: String,
    pub exp: u64,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}

pub struct ErrorResponse(pub StatusCode, pub String);

#[derive(Serialize)]
struct ErrorMsg {
    error: String,
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response {
        let body = Json(ErrorMsg { error: self.1 });
        (self.0, body).into_response()
    }
}

type BearerHeader = TypedHeader::<Authorization<Bearer>>;

#[async_trait]
impl<B> FromRequestParts<B> for Claims
where B: Send + Sync {
    type Rejection = ErrorResponse;

    async fn from_request_parts(req: &mut Parts, _state: &B) -> Result<Self, Self::Rejection> {
        let secret = Environment::get_var("ORION_SECRET").expect("ORION_SECRET environment variable not set");
        if let Ok(TypedHeader(auth)) = BearerHeader::from_request_parts(req, _state).await {
            let token = auth.token();
            return validate_jwt(token, secret.as_bytes())
                .map_err(|_| ErrorResponse(StatusCode::UNAUTHORIZED, "Invalid token".to_string()));
        }
        let jar = CookieJar::from_headers(&req.headers);
        if let Some(token_cookie) = jar.get("authToken") {
            let token = token_cookie.value();
            return validate_jwt(token, secret.as_bytes())
                .map_err(|_| ErrorResponse(StatusCode::UNAUTHORIZED, "Invalid token".to_string()));
        }

        return Err(ErrorResponse(StatusCode::UNAUTHORIZED, "No auth token".to_string()));
    }
}