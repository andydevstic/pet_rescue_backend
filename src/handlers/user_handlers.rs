use actix_web::{web, HttpResponse};
use crate::models::user::user_dtos::*;

fn create_user(create_user_dto: web::Json<CreateUserDTO>, pool: web::Data<Pool>) {}