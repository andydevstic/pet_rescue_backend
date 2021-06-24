use std::env;

pub struct Config {
  pub jwt_secret: String,
  pub db_read_url: String,
  pub db_write_url: String,
  pub db_migration_url: String,
}

impl Config {
  pub fn new() -> Config {
    let jwt_secret = match env::var("JWT_SECRET") {
      Ok(data) => data,
      Err(_) => panic!("JWT secret must be set!"),
    };

    let db_read_url = match env::var("POSTGRES_READ") {
      Ok(data) => data,
      Err(_) => panic!("Postgres read connection url must be set!"),
    };

    let db_write_url = match env::var("POSTGRES_WRITE") {
      Ok(data) => data,
      Err(_) => panic!("Postgres write connection url must be set!"),
    };

    let db_migration_url = db_write_url.clone();

    Config {
      jwt_secret: jwt_secret.clone(),
      db_read_url: db_read_url.clone(),
      db_write_url: db_write_url.clone(),
      db_migration_url: db_migration_url.clone(),
    }
  }
}
