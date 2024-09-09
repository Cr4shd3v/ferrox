//! Contains the implementation of the database pool. [deadpool] is used.
//!
//! Use [DatabaseFairing] as fairing for rocket.

use std::env;
use std::sync::OnceLock;

use deadpool::managed::Object;
use diesel_async::pooled_connection::deadpool::Pool;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::AsyncPgConnection;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::{Build, Rocket};

/// Fairing initializing the [DbPool].
pub struct DatabaseFairing;

#[async_trait]
impl Fairing for DatabaseFairing {
    fn info(&self) -> Info {
        Info {
            name: "database-init",
            kind: Kind::Ignite,
        }
    }

    async fn on_ignite(&self, rocket: Rocket<Build>) -> rocket::fairing::Result {
        DB_POOL.get_or_init(init_db);

        Ok(rocket)
    }
}

static DB_POOL: OnceLock<PgPool> = OnceLock::new();

type PgPool = Pool<AsyncPgConnection>;
/// Type describing a connection from the [DbPool].
pub type PooledConnection = Object<AsyncDieselConnectionManager<AsyncPgConnection>>;

fn init_db() -> PgPool {
    let uri = env::var("DATABASE_URL").expect("No DATABASE_URL found");

    let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(uri);
    Pool::builder(manager).build().expect("Failed to create deadpool")
}

/// Holds functions to retrieve connections from the pool.
pub struct DbPool;

impl DbPool {
    /// Retrieves a [PooledConnection] from the pool.
    ///
    /// # Safety
    /// This requires [Self::get_or_init_conn] to be called first.
    ///
    /// This usually happens through the [DatabaseFairing].
    pub async fn get_conn() -> Result<Object<AsyncDieselConnectionManager<AsyncPgConnection>>, deadpool::managed::PoolError<diesel_async::pooled_connection::PoolError>> {
        DB_POOL.get().unwrap().get().await
    }

    /// Gets a connection and initialize the pool if not initialized.
    pub async fn get_or_init_conn() -> Result<Object<AsyncDieselConnectionManager<AsyncPgConnection>>, deadpool::managed::PoolError<diesel_async::pooled_connection::PoolError>> {
        DB_POOL.get_or_init(init_db).get().await
    }
}

#[cfg(test)]
mod tests {
    use diesel::sql_types::Text;
    use diesel::IntoSql;
    use diesel_async::RunQueryDsl;

    use crate::prelude::*;

    #[async_test]
    async fn test_db() {
        EnvLoader::load_test();
        let mut conn = DbPool::get_or_init_conn().await.unwrap();
        let result = diesel::select("healthy".into_sql::<Text>()).load::<String>(&mut conn).await.unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0], "healthy");
    }
}