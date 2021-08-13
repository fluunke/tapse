mod db;
mod errors;
mod handlers;
mod models;
mod websocket;
use broadcaster::BroadcastChannel;
use futures::Future;
use handlers::{
    file::{list_files, upload_file},
    handle_embedded_file,
    message::list_messages,
    room::{insert_room, list_rooms},
    session::{get_username, set_username},
};
use http_types::{cookies::SameSite, headers::HeaderValue, StatusCode};
use models::Session;
use rust_embed::RustEmbed;
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};
use std::{
    net::{IpAddr, SocketAddr},
    path::PathBuf,
    pin::Pin,
    sync::Arc,
};
use tide::{
    security::{CorsMiddleware, Origin},
    sessions::{self},
    Next, Request, Response,
};
use websocket::WSEvent;

use crate::handlers::file::{delete_file, view_file};

#[derive(RustEmbed)]
#[folder = "frontend/build"]
struct Frontend;

#[derive(Clone)]
pub struct State {
    pool: Arc<SqlitePool>,
    broadcaster: BroadcastChannel<WSEvent>,
    password: Option<String>,
}

use clap::Clap;

#[derive(Clap)]
struct Opts {
    #[clap(short, long, default_value = "8080")]
    port: u16,
    #[clap(short, long, default_value = "127.0.0.1")]
    interface: IpAddr,
    #[clap(short, long, default_value = "./tapse.db")]
    /// Path to the database file
    db: PathBuf,
    #[clap(long)]
    password: Option<String>,
}

#[async_std::main]
async fn main() -> Result<(), sqlx::Error> {
    tide::log::start();

    let opts: Opts = Opts::parse();

    // Channel shared between state to send and receive websocket messages.
    let broadcaster = BroadcastChannel::new();

    let cors = CorsMiddleware::new()
        .allow_methods("GET, POST, OPTIONS, DELETE".parse::<HeaderValue>().unwrap())
        .allow_origin(Origin::from("*"))
        .allow_credentials(true);

    let db_url = format!(
        "sqlite://{}?mode=rwc",
        opts.db.to_str().expect("Invalid database path!")
    );

    let db = SqlitePoolOptions::new()
        .max_connections(16)
        .connect(&db_url)
        .await?;

    sqlx::migrate!().run(&db).await?;

    let mut app = tide::with_state({
        State {
            pool: Arc::new(db),
            broadcaster,
            password: opts.password,
        }
    });

    app.with(
        sessions::SessionMiddleware::new(
            sessions::MemoryStore::new(),
            "sessions_dont_need_to_be_secure_".as_bytes(),
        )
        .with_cookie_name("TAPSE_SESSION")
        .with_same_site_policy(SameSite::Lax),
    );

    app.at("/").get(handle_embedded_file);
    app.at("/*path").get(handle_embedded_file);

    // API handlers
    let mut api = app.at("/api");

    // Persist username
    // Note: this is not a *secure* session, neither is it meant to be (yet)
    api.at("/session").get(get_username);
    api.at("/session").post(set_username);

    api.with(authenticate);

    api.at("/rooms").post(insert_room);
    api.at("/rooms").get(list_rooms);

    api.at("/chat").get(list_messages);

    api.at("/files").post(upload_file);
    api.at("/files").get(list_files);
    api.at("/files/:id/:name").get(view_file);
    api.at("/files/:id/:name").delete(delete_file);
    crate::websocket::make_ws(api);

    app.with(cors);

    let addr: SocketAddr = SocketAddr::new(opts.interface, opts.port);

    app.listen(addr).await?;
    Ok(())
}

fn authenticate<'a>(
    req: Request<State>,
    next: Next<'a, State>,
) -> Pin<Box<dyn Future<Output = tide::Result> + Send + 'a>> {
    Box::pin(async {
        let user: Option<Session> = req.session().get("user");
        match user {
            Some(_) => Ok(next.run(req).await),
            None => Ok(Response::new(StatusCode::Unauthorized)),
        }
    })
}
