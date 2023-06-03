mod models;
pub mod schema;

use std::env;

use diesel::dsl::exists;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::select;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use tokio::sync::Mutex;
use tracing::info;

// use crate::database::models::NewCharacter;
// use crate::database::schema::characters::columns;
// use crate::database::schema::characters::dsl::characters;
// use crate::models::character::Character;
// use crate::Error;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub struct Database {
    connection: Mutex<PgConnection>,
}

impl Database {
    pub fn new() -> Self {
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let mut connection = PgConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

        if let Err(err) = connection.run_pending_migrations(MIGRATIONS) {
            panic!("Error migrating database: {}", err)
        }

        info!("Connected to database at {}", database_url);

        Database {
            connection: Mutex::new(connection),
        }
    }
}
