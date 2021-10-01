
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use r2d2;
use std::env;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

embed_migrations!();

pub fn init() -> Pool {
    let db_url = env::var("DATABASE_URL").expect("Database url not set");
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    let pool = Pool::new(manager).expect("Failed to create db pool");
    //Running DB migrations
    let conn = pool.get().expect("Failed to get db connection");
    embedded_migrations::run(&conn).unwrap();
    // Return the pool for use
    return pool;
}