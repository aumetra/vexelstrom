use crate::config::Configuration;
use diesel::Connection;
use diesel_async::{
    AsyncPgConnection,
    async_connection_wrapper::AsyncConnectionWrapper,
    pooled_connection::{AsyncDieselConnectionManager, bb8},
};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};

pub mod changesets;
pub mod models;
pub mod schema;

pub type PgPool = bb8::Pool<AsyncPgConnection>;

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

pub async fn connect(config: &Configuration) -> eyre::Result<PgPool> {
    // run migrations
    tokio::task::spawn_blocking({
        let conn_str = config.database.url.clone();

        move || {
            let mut conn = AsyncConnectionWrapper::<AsyncPgConnection>::establish(&conn_str)?;
            conn.run_pending_migrations(MIGRATIONS)
                .map_err(eyre::Report::msg)?;

            eyre::Ok(())
        }
    })
    .await??;

    let manager = AsyncDieselConnectionManager::new(&config.database.url);
    let pool = bb8::Pool::builder()
        .max_size(config.database.num_conns.get() as u32)
        .build(manager)
        .await?;

    Ok(pool)
}
