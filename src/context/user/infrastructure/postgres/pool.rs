use anyhow::Context;
use diesel::connection::SimpleConnection;
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, CustomizeConnection, Error as R2d2Error, Pool};

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

#[derive(Debug)]
struct SearchPathCustomizer;

impl CustomizeConnection<PgConnection, R2d2Error> for SearchPathCustomizer {
    fn on_acquire(&self, conn: &mut PgConnection) -> Result<(), R2d2Error> {
        conn.batch_execute(r#"SET search_path TO "user", public"#)
            .map_err(R2d2Error::QueryError)?;
        Ok(())
    }
}

pub fn init_pool(database_uri: &str) -> anyhow::Result<PgPool> {
    let manager = ConnectionManager::<PgConnection>::new(database_uri);
    Pool::builder()
        .connection_customizer(Box::new(SearchPathCustomizer))
        .build(manager)
        .with_context(|| "failed to build PostgreSQL connection pool")
}
