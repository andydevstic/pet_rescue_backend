use diesel::{PgConnection, r2d2};
use diesel::r2d2::{Pool, ConnectionManager};

type PgPool = Pool<ConnectionManager<PgConnection>>;

pub struct PostgresConnection {
  pub read_conn: PgPool,
  pub write_conn: PgPool,
}

pub fn generate_pool(database_url: &str) -> Result<PgPool, String> {
  let connection_manager: ConnectionManager<PgConnection> = r2d2::ConnectionManager::new(database_url);

  match Pool::builder()
    .max_size(8)
    .build(connection_manager) {
    Ok(data) => Ok(data),
    Err(_) => Err(format!("Failed to connect to database url: {}", database_url)),
  }
}