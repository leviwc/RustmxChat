use super::m20240302_000001_create_user_table::User;
use sea_orm_migration::prelude::*;
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Chat::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Chat::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(UserInChat::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(UserInChat::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(UserInChat::ChatId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("chat_id_to_user_in_chat_fk")
                            .from(UserInChat::Table, UserInChat::ChatId)
                            .to(Chat::Table, Chat::Id),
                    )
                    .col(ColumnDef::new(UserInChat::UserId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("user_id_to_user_in_chat_fk")
                            .from(UserInChat::Table, UserInChat::UserId)
                            .to(User::Table, User::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserInChat::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Chat::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Chat {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum UserInChat {
    Table,
    Id,
    UserId,
    ChatId,
}
