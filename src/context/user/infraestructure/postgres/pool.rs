use diesel::{
    PgConnection,
    r2d2::{ConnectionManager, Pool},
};

pub type PgPool = Pool<ConnectionManager<PgConnection>>;

pub fn init_pool(database_uri: &str) -> PgPool {
    let manager = ConnectionManager::<PgConnection>::new(database_uri);
    Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}
