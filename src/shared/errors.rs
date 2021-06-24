pub struct AppError {
  code: u8,
  message: &'static str,
}

pub enum AppErrors {
  DatabaseError(AppError),
  InternalServerError(AppError),
  NotFoundError(AppError),
  UnAuthorizedError(AppError),
  ForbiddenError(AppError),
}