use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use log::info;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

pub fn run_migrations(conn: &mut impl MigrationHarness<diesel::pg::Pg>) {
    conn.run_pending_migrations(MIGRATIONS).unwrap();

    info!("Migrations run successfully");
}
