use crate::chat::database::ChatDatabase;
use askama::Template;
use axum::extract::State;
use axum::{debug_handler, response::IntoResponse, Form};
use serde::Deserialize;
use std::sync::Arc;

use crate::{entities::message, AppState, HtmlTemplate};
#[derive(Template)]
#[template(path = "message-form.html")]
pub struct MessageForm {
    messages: Vec<message::Model>,
    chat_id: i32,
    user_id: i32,
}

#[derive(Deserialize)]
pub struct GetMessagesRequest {
    chat_id: i32,
    user_id: i32,
}

#[debug_handler]
pub async fn chat_page(
    State(state): State<Arc<AppState>>,
    Form(message): Form<GetMessagesRequest>,
) -> impl IntoResponse {
    let GetMessagesRequest { chat_id, user_id } = message;
    if let Ok((messages, _)) =
        ChatDatabase::get_chat_messages_by_id(&state.db, chat_id.into(), 1, 300).await
    {
        let template = MessageForm {
            messages,
            chat_id,
            user_id,
        };

        return HtmlTemplate(template);
    }
    HtmlTemplate(MessageForm {
        messages: vec![],
        chat_id,
        user_id,
    })
}
