mod auth;
mod context;

use axum::{
    routing::post, Json, Router, 
    http::{StatusCode, Request, Response}, 
    middleware::{from_fn, Next},
    body::Body
};
use axum_server::tls_rustls::RustlsConfig;
use tower_http::{services::ServeDir};
use std::net::SocketAddr;
use std::path::PathBuf;
use auth::{claims::{Claims, LoginRequest, LoginResponse, ErrorResponse}, tokens::{create_jwt, validate_credentials}};
use context::environment::{Environment, Singleton};


async fn handle_login(Json(payload): Json<LoginRequest>) -> Result<Json<LoginResponse>, ErrorResponse> {

    if validate_credentials(&payload.username, &payload.password) {
        let secret = Environment::get_var("ORION_SECRET").expect("ORION_SECRET environment variable not set");
        let token = create_jwt(Json(payload), secret.as_bytes()).map_err(|_| ErrorResponse(StatusCode::INTERNAL_SERVER_ERROR, "Failed to create JWT".to_string()))?;
        Ok(Json(LoginResponse { token }))
    } else {
        Err(ErrorResponse(StatusCode::UNAUTHORIZED, "Invalid username or password".to_string()))
    }
}

async fn auth_middleware(
    claims: Claims,
    mut request: Request<Body>,
    next: Next
) -> Result<Response<Body>, ErrorResponse> {
    request.extensions_mut().insert(claims);
    Ok(next.run(request).await)
}

#[tokio::main]
async fn main() {
    
    dotenvy::dotenv().ok();
    let routes: String = Environment::get_var("ROUTES").expect("ROUTES environment variable not set");

    let config: RustlsConfig = RustlsConfig::from_pem_file(
        PathBuf::from(Environment::get_var("TLS_CERT").expect("TLS_CERT environment variable not set")),
        PathBuf::from(Environment::get_var("TLS_KEY").expect("TLS_KEY environment variable not set")),
    )
    .await
    .expect("Failed to load TLS configuration");


    let static_service: ServeDir = ServeDir::new(routes.clone())
        .append_index_html_on_directories(true);

    let public_routes = Router::new()
        .route("/api/login", post(handle_login))
        .nest_service("/login", ServeDir::new(routes.clone() + "/login").append_index_html_on_directories(true))
        .nest_service("/public", ServeDir::new(routes.clone() + "/public"));
    
    let protected_routes = Router::new()
        .fallback_service(static_service)
        .layer(from_fn(auth_middleware));

    let app = public_routes.merge(protected_routes);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8443));

    println!("Listening on https://{}", addr);

    axum_server::bind_rustls(addr, config)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start server");
}