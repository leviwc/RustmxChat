use crate::chat::database::ChatDatabase;
use askama::{Html, Template};
use axum::extract::ws::Message;
use axum::extract::Query;
use axum::extract::{ws::WebSocket, State, WebSocketUpgrade};
use axum::{debug_handler, response::IntoResponse, Form};
use futures::{sink::SinkExt, stream::StreamExt};
use redis::Commands;
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::info;

use crate::{entities::message, AppState, HtmlTemplate};

#[derive(Template)]
#[template(path = "message-form.html")]
pub struct MessageForm {
    messages: Vec<message::Model>,
    chat_id: i32,
    user_id: i32,
}

pub async fn get_chat_messages(
    db: &DatabaseConnection,
    chat_id: i32,
    user_id: i32,
) -> HtmlTemplate<MessageForm> {
    if let Ok((messages, _)) =
        ChatDatabase::get_chat_messages_by_id(db, chat_id.into(), 1, 300).await
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

#[derive(Template, Deserialize, Serialize)]
#[template(path = "message-list.html")]
pub struct MessageList {
    messages: Vec<message::Model>,
    user_id: i32,
}
#[derive(Deserialize)]
pub struct MessageRequest {
    message: String,
    user_id: i32,
    chat_id: i32,
}
#[debug_handler]
pub async fn add_message(
    State(state): State<Arc<AppState>>,
    Form(message): Form<MessageRequest>,
) -> impl IntoResponse {
    let res = ChatDatabase::add_message(
        &state.db,
        message.message.clone(),
        message.user_id,
        message.chat_id,
    )
    .await;
    if let Err(err) = res {
        info!("{}", err.to_string());
    }
    let mut redis_conn = state
        .redis
        .get_connection()
        .expect("Failed getting redis connection");
    let _ = redis_conn.publish::<String, String, String>("channel_1".to_string(), message.message);
    if let Ok((messages, _)) =
        ChatDatabase::get_chat_messages_by_id(&state.db, message.chat_id.into(), 1, 300).await
    {
        let template = MessageList {
            messages,
            user_id: message.user_id,
        };

        return HtmlTemplate(template);
    }
    HtmlTemplate(MessageList {
        messages: vec![],
        user_id: message.user_id,
    })
}

#[derive(Deserialize)]
pub struct GetMessagesRequest {
    chat_id: i32,
    user_id: i32,
}

#[debug_handler]
pub async fn get_messages(
    State(state): State<Arc<AppState>>,
    Form(message): Form<GetMessagesRequest>,
) -> impl IntoResponse {
    get_chat_messages(&state.clone().db, message.chat_id, message.user_id).await
}

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

#[derive(Deserialize)]
pub struct ReceiveMessagesRequest {
    chat_id: i32,
    user_id: i32,
}

#[debug_handler]
pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
    Query(message): Query<ReceiveMessagesRequest>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| websocket(socket, state, message.user_id, message.chat_id))
}

async fn websocket(stream: WebSocket, state: Arc<AppState>, user_id: i32, chat_id: i32) {
    // By splitting, we can send and receive at the same time.
    let (mut sender, mut receiver) = stream.split();

    let sender_state = state.clone();
    let mut sender_task = tokio::spawn(async move {
        let client = &sender_state.redis;
        let mut redis_conn = client
            .get_connection()
            .expect("Failed getting redis connection");
        let mut pubsub = redis_conn.as_pubsub();
        pubsub.subscribe(format!("chat:{chat_id}")).unwrap();

        loop {
            let msg = pubsub.get_message().unwrap();
            let payload: String = msg.get_payload().unwrap();
            let res: Result<message::Model, _> = serde_json::from_str(&payload);
            let msg = if let Ok(new_message) = res {
                let template = MessageList {
                    user_id,
                    messages: vec![new_message],
                };

                template
            } else {
                MessageList {
                    messages: vec![],
                    user_id,
                }
            };
            if sender
                .send(Message::Text(msg.render().unwrap()))
                .await
                .is_err()
            {
                break;
            }
        }
    });

    let receiver_state = state.clone();
    let mut receiver_task = tokio::spawn(async move {
        while let Some(Ok(Message::Text(text))) = receiver.next().await {
            let message: MessageRequest = serde_json::from_str(&text).unwrap();

            let res = ChatDatabase::add_message(
                &receiver_state.db,
                message.message.clone(),
                message.user_id,
                message.chat_id,
            )
            .await;
            if let Err(err) = res {
                info!("{}", err.to_string());
                break;
            }

            let mut redis_conn = state
                .redis
                .get_connection()
                .expect("Failed getting redis connection");

            if let Ok(new_message) = res {
                let _ = redis_conn.publish::<String, String, String>(
                    format!("chat:{}", message.chat_id),
                    serde_json::to_string(&new_message).unwrap(),
                );
            }
        }
    });

    // If any one of the tasks run to completion, we abort the other.
    tokio::select! {
        _ = (&mut sender_task) => sender_task.abort(),
        _ = (&mut receiver_task) => receiver_task.abort(),
    };
}
