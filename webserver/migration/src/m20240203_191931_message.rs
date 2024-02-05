use sea_orm_migration::prelude::*;

use crate::{
    m20240302_000001_create_user_table::User, m20240302_000002_create_user_to_user_chat_table::Chat,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Message::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Message::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Message::Text).string().not_null())
                    .col(ColumnDef::new(Message::UserId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("user_id_to_user_in_chat_fk")
                            .from(Message::Table, Message::UserId)
                            .to(User::Table, User::Id),
                    )
                    .col(ColumnDef::new(Message::ChatId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("chat_id_to_user_in_chat_fk")
                            .from(Message::Table, Message::ChatId)
                            .to(Chat::Table, Chat::Id),
                    )
                    .col(ColumnDef::new(Message::Timestamp).integer().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Message::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Message {
    Table,
    Id,
    ChatId,
    UserId,
    Text,
    Timestamp,
}
