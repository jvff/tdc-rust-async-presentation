use {
    axum::{http::StatusCode, service::get, Router, Server},
    std::{convert::Infallible, io, net::SocketAddr},
    tower_http::{services::ServeDir, trace::TraceLayer},
    tracing::info,
};

#[tokio::main]
async fn main() -> Result<(), hyper::Error> {
    tracing_subscriber::fmt::init();

    let address = SocketAddr::from(([0, 0, 0, 0], 8182));

    let app = Router::new()
        .nest(
            "/",
            get(ServeDir::new("../dist")).handle_error(
                |error: io::Error| -> Result<_, Infallible> {
                    Ok((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Failed to serve file: {}", error),
                    ))
                },
            ),
        )
        .layer(TraceLayer::new_for_http());

    info!("Listening on {}", address);
    Server::bind(&address).serve(app.into_make_service()).await
}
