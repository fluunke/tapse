mod db;
mod errors;
mod handlers;
mod websocket;
use async_session::MemoryStore;
use axum::{routing::get, Extension, Router};
use broadcaster::BroadcastChannel;
use handlers::{
    file::{list_files, upload_file},
    handle_embedded_file,
    message::list_messages,
    room::{insert_room, list_rooms},
    session::{get_username, set_username},
};
use rust_embed::RustEmbed;
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};
use std::{
    net::{IpAddr, SocketAddr},
    path::PathBuf,
};

use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use websocket::WSEvent;

use crate::handlers::file::{delete_file, view_file};

#[derive(RustEmbed)]
#[folder = "frontend/build"]
struct Frontend;

#[derive(Clone)]
pub struct Server {
    password: Option<String>,
}

use clap::Parser;
#[derive(Parser)]
#[clap(name = "Tapse")]
#[clap(author = "fluunke")]
#[clap(version = "0.0.2")]
#[clap(about = " Real-time chat and file sharing, inspired by PirateBox", long_about = None)]
struct Opts {
    #[clap(short, long, default_value = "8080")]
    port: u16,
    #[clap(short, long, default_value = "127.0.0.1")]
    interface: IpAddr,
    #[clap(long, default_value = "./tapse.db")]
    /// Path to the sqlite database file
    db: PathBuf,
    #[clap(long)]
    /// Optional access password
    password: Option<String>,
}

pub const SESSION_COOKIE_NAME: &str = "TAPSE_SESSION";
pub type Broadcaster = BroadcastChannel<WSEvent>;
pub type Database = Pool<Sqlite>;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let opts = Opts::parse();

    dotenv::dotenv().ok();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "tapse=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Channel shared between state to send and receive websocket messages.
    let broadcaster: Broadcaster = BroadcastChannel::new();

    let db_url = format!(
        "sqlite://{}?mode=rwc",
        opts.db.to_str().expect("Invalid database path!")
    );

    let db: Database = SqlitePoolOptions::new()
        .max_connections(16)
        .connect(&db_url)
        .await?;

    sqlx::migrate!().run(&db).await?;

    let server_state = Server {
        password: opts.password,
    };

    let store = MemoryStore::new();

    let app = Router::new()
        .route("/", get(handle_embedded_file))
        .route("/style.css", get(handle_embedded_file))
        .route("/logo.svg", get(handle_embedded_file))
        .route("/dist/*path", get(handle_embedded_file))
        .nest(
            "/api",
            Router::new()
                .route("/session", get(get_username).post(set_username))
                .route("/rooms", get(list_rooms).post(insert_room))
                .route("/chat", get(list_messages))
                .route("/files", get(list_files).post(upload_file))
                .route("/files/:id/:name", get(view_file).delete(delete_file))
                .route("/ws", get(websocket::ws_handler)),
        )
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                // .allow_origin(Origin::exact("*".parse().unwrap()))
                .allow_methods(Any)
                .allow_headers(Any)
                .allow_credentials(true),
        )
        .layer(TraceLayer::new_for_http())
        .layer(Extension(server_state))
        .layer(Extension(broadcaster))
        .layer(Extension(db))
        .layer(Extension(store));

    let addr: SocketAddr = SocketAddr::new(opts.interface, opts.port);
    tracing::info!("Listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
