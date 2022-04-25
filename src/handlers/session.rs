use async_session::{async_trait, MemoryStore, Session, SessionStore};
use axum::{
    extract::{FromRequest, RequestParts},
    http::StatusCode,
    response::{IntoResponse, Redirect, Response},
    Extension, Json,
};
use axum_extra::extract::{
    cookie::{Cookie, SameSite},
    CookieJar,
};
use nanoid::nanoid;
use serde::{Deserialize, Serialize};

use crate::{errors::TapseError, Server, SESSION_COOKIE_NAME};

#[derive(serde::Deserialize)]
pub struct SessionQuery {
    pub username: String,
    pub password: Option<String>,
}

pub async fn set_username(
    Json(query): Json<SessionQuery>,
    Extension(store): Extension<MemoryStore>,
    Extension(server): Extension<Server>,
    jar: CookieJar,
) -> Result<impl IntoResponse, TapseError> {
    if query.password != server.password {
        return Err(TapseError::WrongPassword);
    };

    // Create a new session filled with user data
    let mut session = Session::new();
    session
        .insert(
            "user",
            User {
                id: nanoid!(6),
                username: query.username,
            },
        )
        .unwrap();

    // Store session and get corresponding cookie
    let cookie = store.store_session(session).await.unwrap().unwrap();

    let c = Cookie::build(SESSION_COOKIE_NAME, cookie)
        .same_site(SameSite::Lax)
        .path("/")
        .finish();

    Ok((jar.add(c), Redirect::to("/")))
}

pub async fn get_username(user: User) -> impl IntoResponse {
    Json(user.username)
}

pub struct AuthError;

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        (StatusCode::UNAUTHORIZED, "Unauthorized").into_response()
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct User {
    pub id: String,
    pub username: String,
}

impl Default for User {
    fn default() -> Self {
        User {
            id: nanoid!(6),
            username: "Anonymous".to_string(),
        }
    }
}

#[async_trait]
impl<B> FromRequest<B> for User
where
    B: Send,
{
    type Rejection = AuthError;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let Extension(store) = Extension::<MemoryStore>::from_request(req)
            .await
            .expect("`MemoryStore` extension is missing");

        let cookies = CookieJar::from_request(req).await.unwrap();
        let session_cookie = cookies
            .get(SESSION_COOKIE_NAME)
            .ok_or(AuthError)?
            .value()
            .to_string();

        let session = store
            .load_session(session_cookie)
            .await
            .unwrap()
            .ok_or(AuthError)?;

        let user = session.get::<User>("user").ok_or(AuthError)?;

        Ok(user)
    }
}
