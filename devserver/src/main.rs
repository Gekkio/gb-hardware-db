// SPDX-FileCopyrightText: 2017-2022 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// SPDX-License-Identifier: MIT

use axum::{routing::get_service, Router};
use std::net::SocketAddr;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let app = Router::new().fallback(get_service(ServeDir::new("build")));

    let port = 8080;
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    println!("Development server listening at port {port}");
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
