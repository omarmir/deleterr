use super::models::{AuthError, AuthErrorType};
use actix_session::Session;

pub fn login(session: Session, message: String) -> Result<(), AuthError> {
    session
        .insert("message", message.clone())
        .map_err(|_err| AuthError::new(AuthErrorType::SessionStoreUnavailable))
}

pub fn current_session(session: Session) -> Result<String, AuthError> {
    let result = session.get::<String>("message");

    result
        .map_err(|_err| AuthError::new(AuthErrorType::SessionStoreUnavailable))?
        .ok_or(AuthError::new(AuthErrorType::Unauthorized))
}
