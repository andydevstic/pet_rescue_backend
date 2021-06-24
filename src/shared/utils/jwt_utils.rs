// use crate::shared::config::Config;
// use crate::shared::consts::JwtPayload;
// use std::ops::Add;
// use serde::Serialize;
// use jsonwebtoken::{encode, Algorithm, Header, EncodingKey, errors};

// #[derive(Serialize)]
// struct Claims {
//   user: User,
//   exp: usize,
// }

// pub struct JwtUtils {}

// impl JwtUtils {
//   fn header(&self) -> Header {
//     Header::new(Algorithm::HS512)
//   }

//   pub fn generate_jwt(&self, payload: JwtPayload) -> Result<String, errors::Error> {
//     match payload {
//       JwtPayload::UserAuth { user } => {
//         let claim = Claims {
//           user,
//           exp: Utc::now().add(Duration::hours(6)).timestamp_millis() as usize,
//         };

//         let secret_key = Config::new().jwt_secret;

//         encode(&self.header(), &claim, &EncodingKey::from_secret(secret_key.as_bytes()))
//       }
//     }
//   }
// }