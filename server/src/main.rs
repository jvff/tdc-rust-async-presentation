use {
    axum::{http::StatusCode, service::get, Router, Server},
    std::{convert::Infallible, io, net::SocketAddr},
    tower_http::services::ServeDir,
};

#[tokio::main]
async fn main() -> Result<(), hyper::Error> {
    let address = SocketAddr::from(([0, 0, 0, 0], 8182));

    let app = Router::new().nest(
        "/",
        get(ServeDir::new("../dist")).handle_error(
            |error: io::Error| -> Result<_, Infallible> {
                Ok((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to serve file: {}", error),
                ))
            },
        ),
    );

    Server::bind(&address).serve(app.into_make_service()).await
}
