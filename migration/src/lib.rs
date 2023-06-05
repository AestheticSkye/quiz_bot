pub use sea_orm_migration::prelude::*;

mod m20230603_113226_create_quiz_table;
mod m20230603_120131_create_question_table;
mod m20230603_122017_create_answer_table;
mod m20230603_161257_create_quiz_creation_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
	fn migrations() -> Vec<Box<dyn MigrationTrait>> {
		vec![
            Box::new(m20230603_113226_create_quiz_table::Migration),
            Box::new(m20230603_120131_create_question_table::Migration),
            Box::new(m20230603_122017_create_answer_table::Migration),
            Box::new(m20230603_161257_create_quiz_creation_table::Migration),
        ]
	}
}
