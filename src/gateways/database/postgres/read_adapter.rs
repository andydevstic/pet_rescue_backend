use crate::shared::config::Config;
use crate::shared::traits::*;
use diesel::*;

pub struct PostgresReadAdapter {
  pub conn: PgConnection,
}

impl PostgresAdapter for PostgresReadAdapter {
  fn connect_db(&mut self) -> () {
    let db_url = Config::new().db_read_url;

    self.conn = PgConnection::establish(db_url).expect("Error connecting to postgres read");
  }
}