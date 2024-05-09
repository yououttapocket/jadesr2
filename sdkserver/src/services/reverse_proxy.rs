use axum::{
    body::Body,
    extract::{Request, State},
    http::uri::{PathAndQuery, Uri},
    response::{IntoResponse, Response},
};
use hyper::StatusCode;
use hyper_util::client::legacy::connect::HttpConnector;

use crate::config::CONFIGURATION;

type Client = hyper_util::client::legacy::Client<HttpConnector, Body>;

pub async fn forward_to_dispatch(
    State(client): State<Client>,
    mut req: Request,
) -> Result<Response, StatusCode> {
    let path = req.uri().path();
    let path_query = req
        .uri()
        .path_and_query()
        .map_or(path, PathAndQuery::as_str);

    let uri = format!("{}{}", CONFIGURATION.dispatch_endpoint, path_query);

    *req.uri_mut() = Uri::try_from(uri).unwrap();

    Ok(client
        .request(req)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?
        .into_response())
}
