use std::time::Duration;

use diesel_async::{
    AsyncPgConnection, RunQueryDsl,
    pooled_connection::{
        AsyncDieselConnectionManager,
        bb8::{Pool, PooledConnection},
    },
};

pub type Db = Pool<AsyncPgConnection>;
pub type DbConn<'a> = PooledConnection<'a, AsyncPgConnection>;

pub async fn init_pool(
    database_url: &str,
    max_pool: u32,
) -> Result<Db, diesel_async::pooled_connection::PoolError> {
    let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(database_url);

    Pool::builder()
        .max_size(max_pool)
        .connection_timeout(Duration::from_secs(2))
        .build(manager)
        .await
}

pub async fn ping(pool: &Db) -> Result<(), diesel::result::Error> {
    let mut conn = pool
        .get()
        .await
        .map_err(|_| diesel::result::Error::BrokenTransactionManager)?;

    diesel::sql_query("SELECT 1").execute(&mut conn).await?;

    Ok(())
}
