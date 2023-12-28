use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::PgConnection;
use dotenv::dotenv;
use std::sync::Arc;
pub trait DBConnectionManager: Send + Sync {
    fn get_connection(
        &self,
    ) -> Result<PooledConnection<ConnectionManager<PgConnection>>, r2d2::Error>;
}

pub struct DBConnectionManagerImpl {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl DBConnectionManager for DBConnectionManagerImpl {
    fn get_connection(
        &self,
    ) -> Result<PooledConnection<ConnectionManager<PgConnection>>, r2d2::Error> {
        self.pool.get()
    }
}

pub fn get_connection_manager() -> Arc<dyn DBConnectionManager> {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder().build(manager).unwrap();
    Arc::new(DBConnectionManagerImpl { pool })
}
