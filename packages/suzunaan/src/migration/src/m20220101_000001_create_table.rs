use extension::postgres::Type;
use sea_orm::{EnumIter, Iterable};
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(Alias::new("permission"))
                    .values([
                        Alias::new("Public"),
                        Alias::new("Private"),
                        Alias::new("Restricted"),
                    ])
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Post::Table)
                    .if_not_exists()
                    .col(pk_auto(Post::Index))
                    .col(ColumnDef::new(Post::Id).text().not_null())
                    .col(ColumnDef::new(Post::Title).text())
                    .col(ColumnDef::new(Post::Text).text())
                    .col(ColumnDef::new(Post::Author).text())
                    .col(ColumnDef::new(Post::AuthorEmail).text())
                    .col(ColumnDef::new(Post::Secret).text())
                    .col(ColumnDef::new(Post::Password).text())
                    .col(
                        ColumnDef::new(Post::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Post::UpdatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Post::Permission)
                            .enumeration(Alias::new("permission"), Permission::iter())
                            .not_null()
                            .default("Public"),
                    )
                    .to_owned(),
            )
            .await?;
        let db = manager.get_connection();

        db.execute_unprepared(
            "CREATE OR REPLACE FUNCTION update_modified_column()
            RETURNS TRIGGER AS $set_post_updatedat$
            BEGIN
                NEW.updated_at = now();
                RETURN NEW;
            END;
            $set_post_updatedat$ language 'plpgsql';
            CREATE TRIGGER update_post_modtime
            BEFORE UPDATE ON post
            FOR EACH ROW EXECUTE FUNCTION update_modified_column();",
        )
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Post::Table).to_owned())
            .await?;
        manager
            .drop_type(Type::drop().name(Alias::new("permission")).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
enum Post {
    Table,
    Index,
    Id,
    Title,
    Text,
    Author,
    AuthorEmail,
    Secret,
    Password,
    Permission,
    CreatedAt,
    UpdatedAt,
}

#[derive(Iden, EnumIter)]

pub enum Permission {
    #[iden = "Public"]
    Public,
    #[iden = "Private"]
    Private,
    #[iden = "Restricted"]
    Restricted,
}
