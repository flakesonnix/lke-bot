use crate::templates;
use axum::{
    body::Body,
    http::Response,
    response::{IntoResponse, Redirect},
};
use tower_sessions::Session;

pub async fn index(session: Session) -> Response<Body> {
    let user: Option<crate::session::DiscordUser> = session.get("user").await.ok().flatten();

    if user.is_some() {
        return Redirect::to("/dashboard").into_response();
    }

    templates::index().into_response()
}
