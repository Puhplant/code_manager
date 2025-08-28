use clorinde::{deadpool_postgres::{self, Config}, tokio_postgres::NoTls};
use crate::{middleware::authentication::auth_middleware, registration::container::Container, services::auth::jwt_auth::JwtAuth};
pub mod registration;
pub mod services;
pub mod models;
pub mod middleware;
pub mod controllers;

#[tokio::main]
async fn main() {
    //let pool = PgPool::connect("postgres://postgres:postgres@localhost:5432/code_manager").await.unwrap();

    let jwt_auth = JwtAuth::new(
        "secret".as_bytes()
    );

    let mut postgres_config = Config::new();
    postgres_config.dbname = Some("code_manager".to_string());
    postgres_config.host = Some("127.0.0.1".to_string());
    postgres_config.user = Some("postgres".to_string());
    postgres_config.password = Some("postgres".to_string());
    postgres_config.port = Some(5432);

    let pool = postgres_config
        .create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls)
        .expect("Could not connect to database on startup");

    let container = Container::new(pool, jwt_auth);
    let app = axum::Router::new()
        .route("/", axum::routing::get(|| async { "Hello, World!" }))
        .route("/api/auth/login", axum::routing::post(controllers::auth_controller::login))
        .route("/api/auth/register", axum::routing::post(controllers::auth_controller::register))
        .route("/api/auth/refresh", axum::routing::post(controllers::auth_controller::refresh_token))
        .route("/api/tickets", axum::routing::post(controllers::ticket_controller::create_ticket))
        .route("/api/tickets/{id}", axum::routing::get(controllers::ticket_controller::get_ticket_by_id))
        .route("/api/tickets/{id}", axum::routing::put(controllers::ticket_controller::edit_ticket))
        .route("/api/tickets/{id}/move", axum::routing::put(controllers::ticket_controller::move_ticket))
        .route("/api/boards", axum::routing::get(controllers::board_controller::get_boards_by_account_id))
        .route("/api/tickets/board", axum::routing::get(controllers::ticket_controller::get_tickets_by_board_id))
        .with_state(container.clone())
        .route_layer(axum::middleware::from_fn_with_state(container.clone(), auth_middleware));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
