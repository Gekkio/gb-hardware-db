// SPDX-FileCopyrightText: 2017-2022 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use axum::{http::StatusCode, response::IntoResponse, routing::get_service, Router};
use std::net::SocketAddr;
use tokio::io;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let app =
        Router::new().fallback(get_service(ServeDir::new("build")).handle_error(handle_error));

    let port = 8080;
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    println!("Development server listening at port {port}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handle_error(_: io::Error) -> impl IntoResponse {
    (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
}
