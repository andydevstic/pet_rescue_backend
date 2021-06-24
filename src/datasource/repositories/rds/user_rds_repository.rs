use diesel::{PgConnection, RunQueryDsl};

use crate::schema::users::dsl::*;
use crate::models::user::user_model::User;
use crate::models::user::create_user_dto::CreateUserDTO;
use diesel::prelude::*;

pub fn create_user(conn: &PgConnection, payload: CreateUserDTO) -> User {
  let created_user = diesel::insert_into(users)
    .values((
      full_name.eq(payload.full_name.unwrap()),
      email.eq(payload.email),
      role_id.eq(payload.role_id),
      address.eq(payload.address.unwrap()),
      phone_number.eq(payload.phone_number),
    ))
    .get_result(conn).unwrap();
}