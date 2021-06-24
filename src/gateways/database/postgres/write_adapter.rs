use crate::shared::config::Config;
use crate::shared::traits::*;
use diesel::*;

pub struct PostgresWriteAdapter {
  pub conn: PgConnection,
}

impl PostgresAdapter for PostgresWriteAdapter {
  fn connect_db(&mut self) -> () {
    let db_url = Config::new().jwt_secret;

    self.conn = PgConnection::establish(db_url).expect("Error connecting to postgres write");
  }
}