use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(User::IdentityId)
                            .uuid()
                            .not_null()
                            .unique_key(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(OsuTournament::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(OsuTournament::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(OsuTournament::Slug)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(OsuTournament::Name).string().not_null())
                    .col(
                        ColumnDef::new(OsuTournament::StartDate)
                            .timestamp()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(OsuTournament::EndDate)
                            .timestamp()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(OsuTournament::RegistrationStartDate)
                            .timestamp()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(OsuTournament::RegistrationEndDate)
                            .timestamp()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(OsuMappool::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(OsuMappool::Id)
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
                    .table(OsuTeam::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(OsuTeam::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(OsuTeam::Name)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_osu_team_captain")
                            .from_tbl(OsuTeam::Table)
                            .to_tbl(User::Table)
                            .from_col(OsuTeam::Id)
                            .to_col(User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_osu_team_members")
                            .from_tbl(OsuTeam::Table)
                            .to_tbl(User::Table)
                            .from_col(OsuTeam::Id)
                            .to_col(User::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(OsuTournament::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(OsuTeam::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(OsuMappool::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    IdentityId,
}

#[derive(DeriveIden)]
enum OsuTournament {
    Table,
    Id,
    Slug,
    Name,
    StartDate,
    EndDate,
    RegistrationStartDate,
    RegistrationEndDate,
}

#[derive(DeriveIden)]
enum OsuTeam {
    Table,
    Id,
    Name,
    Captain,
    Members,
}

#[derive(DeriveIden)]
enum OsuMappool {
    Table,
    Id,
}
