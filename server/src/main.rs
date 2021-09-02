use {
    axum::Server, presentrs::Presentrs, std::net::SocketAddr,
    tower_http::trace::TraceLayer, tracing::info,
};

#[tokio::main]
async fn main() -> Result<(), hyper::Error> {
    tracing_subscriber::fmt::init();

    let address = SocketAddr::from(([0, 0, 0, 0], 8182));

    let app = Presentrs::new("dist").layer(TraceLayer::new_for_http());

    info!("Listening on {}", address);
    Server::bind(&address).serve(app.into_make_service()).await
}
