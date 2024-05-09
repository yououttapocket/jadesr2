use anyhow::Result;
use axum::body::Body;
use axum::extract::Request;
use axum::routing::{get, post};
use axum::{Router, ServiceExt};
use common::jade_supremacy;
use hyper_util::{client::legacy::connect::HttpConnector, rt::TokioExecutor};
use services::{auth, errors};
use tokio::net::TcpListener;
use tower::Layer;
use tower_http::normalize_path::NormalizePathLayer;
use tracing::Level;

type Client = hyper_util::client::legacy::Client<HttpConnector, Body>;

mod config;
mod services;

use common::logging::init_tracing;

use config::{init_config, CONFIGURATION};
use services::reverse_proxy;

#[tokio::main]
async fn main() -> Result<()> {
    jade_supremacy();
    init_tracing();
    init_config();

    let span = tracing::span!(Level::DEBUG, "main");
    let _ = span.enter();

    // For dispatch reverse proxy
    let client: Client =
        hyper_util::client::legacy::Client::<(), ()>::builder(TokioExecutor::new())
            .build(HttpConnector::new());

    let app = Router::new()
        .route("/query_dispatch", get(reverse_proxy::forward_to_dispatch))
        .route(
            "/query_gateway/:region_name",
            get(reverse_proxy::forward_to_dispatch),
        )
        .route(auth::RISKY_API_CHECK_ENDPOINT, post(auth::risky_api_check))
        .route(
            auth::LOGIN_WITH_PASSWORD_ENDPOINT,
            post(auth::login_with_password),
        )
        .route(
            auth::LOGIN_WITH_SESSION_TOKEN_ENDPOINT,
            post(auth::login_with_session_token),
        )
        .route(
            auth::GRANTER_LOGIN_VERIFICATION_ENDPOINT,
            post(auth::granter_login_verification),
        )
        .fallback(errors::not_found)
        .with_state(client);

    let app = NormalizePathLayer::trim_trailing_slash().layer(app);

    let addr = format!("0.0.0.0:{}", CONFIGURATION.http_port);
    let server = TcpListener::bind(&addr).await?;

    tracing::info!("sdkserver is listening at {addr}");
    axum::serve(server, ServiceExt::<Request>::into_make_service(app)).await?;

    Ok(())
}
