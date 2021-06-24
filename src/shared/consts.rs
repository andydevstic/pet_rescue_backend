// use serde::Serialize;
// use crate::models::user::user_model::User;

// #[derive(Serialize, Debug)]
// pub enum Roles {
//   User(u8),
//   Moderator(u8),
//   Admin(u8),
// }

// impl Roles {
//   pub fn from_int(role_id: u8) -> Result<Roles, String> {
//     match role_id {
//       1 => Result::Ok(Roles::User(1)),
//       2 => Result::Ok(Roles::Moderator(2)),
//       3 => Result::Ok(Roles::Admin(3)),
//       _ => Err(format!("Role id {} not exist", role_id)),
//     }
//   }
// }

// pub enum JwtPayload {
//   UserAuth {
//     user: User,
//   }
// }
