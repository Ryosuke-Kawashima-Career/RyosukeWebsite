pub enum AppError {
    Validation(String),      // 422 {"error": }
    Conflict(String),        // 409,
    NotFound,                // 404,
    Unauthorized,            // 401
    Internal(anyhow::Error), // 500
}
