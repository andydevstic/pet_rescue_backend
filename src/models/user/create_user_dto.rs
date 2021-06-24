
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CreateUserDTO<'a> {
  pub username: &'a str,
  pub full_name: Option<&'a str>,
  pub email: &'a str,
  pub role_id: i32,
  pub address: Option<&'a str>,
  pub phone_number: &'a str
}