// m20220101_000002_create_chef_table.rs

use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m_20220101_000002_create_contact_table" // Make sure this matches with the file name
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    // Define how to apply this migration: Create the Contact table.
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Contact::Table)
                    .col(
                        ColumnDef::new(Contact::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Contact::GivenName).string().not_null())
                    .col(ColumnDef::new(Contact::FamilyName).string().not_null())
                    .col(ColumnDef::new(Contact::Phone).string().not_null())
                    .col(ColumnDef::new(Contact::Email).string().not_null())
                    .to_owned(),
            )
            .await
    }

    // Define how to rollback this migration: Drop the Chef table.
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Contact::Table).to_owned())
            .await
    }
}

// For ease of access
#[derive(Iden)]
pub enum Contact {
    Table,
    Id,
    GivenName,
    FamilyName,
    Phone,
    Email,
}
