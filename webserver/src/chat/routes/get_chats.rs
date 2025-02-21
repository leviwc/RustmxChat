use crate::chat::database::ChatDatabase;
use askama::Template;
use axum::extract::{State};
use axum::{debug_handler, response::IntoResponse, Form};



use serde::{Deserialize};
use std::sync::Arc;


use crate::{AppState, HtmlTemplate};

#[derive(Deserialize)]
pub struct GetChatsRequest {
    user_id: i64,
}
#[derive(Template)]
#[template(path = "chats.html")]
pub struct ChatsTemplate {
    chats: Vec<crate::entities::chat::Model>,
    user_id: i32,
}

#[debug_handler]
pub async fn get_chats(
    State(state): State<Arc<AppState>>,
    Form(request): Form<GetChatsRequest>,
) -> impl IntoResponse {
    let chats = ChatDatabase::get_chats_by_user_id(&state.db, request.user_id, 1, 100).await;
    if let Ok((chats, _)) = chats {
        return HtmlTemplate(ChatsTemplate {
            chats,
            user_id: request.user_id as i32,
        });
    }
    HtmlTemplate(ChatsTemplate {
        chats: vec![],
        user_id: request.user_id as i32,
    })
}
