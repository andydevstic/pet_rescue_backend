use actix_web::{App, HttpRequest, HttpServer, Responder, web::{self, Data}};
use dotenv::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  load_env();

  HttpServer::new(move || {
    App::new()
      .app_data(connect_db())
      .route("/", web::get().to(greet))
  })
    .bind("127.0.0.1:3005")?
    .run()
    .await
}

fn connect_db() -> PostgresConnection {
  let config = Config::new();

  PostgresConnection {
    read_conn: generate_pool(&config.db_read_url[..]).unwrap(),
    write_conn: generate_pool(&config.db_write_url[..]).unwrap(),
  }
}

fn load_env() -> () {
  match dotenv() {
    Ok(_) => println!("Env loaded successfully!"),
    Err(_) => panic!("Error loading env!"),
  };
}