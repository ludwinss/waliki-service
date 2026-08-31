use anyhow::Context;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

pub fn init_pool(database_uri: &str) -> anyhow::Result<PgPool> {
    let manager = ConnectionManager::<PgConnection>::new(database_uri);
    Pool::builder()
        .build(manager)
        .with_context(|| "failed to build PostgreSQL connection pool")
}
