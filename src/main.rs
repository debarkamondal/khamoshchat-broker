mod routes;
#[tokio::main]
async fn main() {
    let listener= tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
    .expect("failed to bind TCP listener");
    let app = routes::gen_routes();
    println!("Server started on http://localhost:3000");
    axum::serve(listener, app)
    .await
    .expect("Failed to initialize the server");
}
