use crate::user::database::UserDatabase;
use askama::Template;
use axum::extract::State;
use axum::{debug_handler, response::IntoResponse, Form};
use bcrypt::hash;
use serde::Deserialize;
use std::sync::Arc;

use crate::AppState;

#[derive(Template)]
#[template(path = "success.html")]
pub struct SuccessMessage {}

#[derive(Deserialize)]
pub struct CreateUserRequest {
    username: String,
    password: String,
}

#[debug_handler]
pub async fn create_user(
    State(state): State<Arc<AppState>>,
    Form(message): Form<CreateUserRequest>,
) -> impl IntoResponse {
    let CreateUserRequest { username, password } = message;
    let password_hash = hash(password, bcrypt::DEFAULT_COST).unwrap();

    let _res = UserDatabase::create_user(&state.db, username, password_hash)
        .await
        .unwrap();
}
