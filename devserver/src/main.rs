// SPDX-FileCopyrightText: 2017-2023 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use axum::{
    http::{header, HeaderValue},
    routing::get_service,
    Router,
};
use std::net::SocketAddr;
use tower_http::{services::ServeDir, set_header::SetResponseHeaderLayer};

#[tokio::main]
async fn main() {
    let app = Router::new().fallback(get_service(ServeDir::new("build")).layer(
        SetResponseHeaderLayer::if_not_present(
            header::CACHE_CONTROL,
            HeaderValue::from_static("no-cache"),
        ),
    ));

    let port = 8080;
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    println!("Development server listening at port {port}");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
