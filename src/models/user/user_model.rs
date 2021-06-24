use serde::*;
use diesel::{Queryable, Identifiable};

use crate::schema::*;

#[derive(Queryable, Identifiable, Serialize, Debug, PartialEq)]
pub struct User {
  pub id: i32,
  pub full_name: String,
  pub email: String,
  pub role_id: i32,
  pub address: String,
  pub phone_number: String,
}