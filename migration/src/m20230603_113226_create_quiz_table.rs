use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
	async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		manager
			.create_table(
				Table::create()
					.table(Quiz::Table)
					.if_not_exists()
					.col(ColumnDef::new(Quiz::Id).uuid().not_null().primary_key())
					.col(ColumnDef::new(Quiz::OwnerId).big_unsigned().not_null())
					.col(ColumnDef::new(Quiz::Text).string().not_null())
					.to_owned(),
			)
			.await
	}

	async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		manager
			.drop_table(Table::drop().table(Quiz::Table).to_owned())
			.await
	}
}

#[derive(Iden)]
pub enum Quiz {
	Table,
	Id,
	OwnerId,
	Text,
}
