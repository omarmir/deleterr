use super::models::HashedUser;
use crate::{
    auth::models::User,
    common::models::{api::APIResponse, deleterr_error::DeleterrError},
    store::services::users::{get_user_by_username, update_user_in_store},
};
use actix_session::Session;
use actix_web::middleware::Next;
use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    FromRequest, HttpResponse,
};

pub fn login_user(session: Session, user: User) -> Result<String, DeleterrError> {
    let username = user.username.clone();
    let is_login_success = verify_user(user)?;
    if is_login_success {
        session
            .insert("message", username.clone())
            .map_err(|err| DeleterrError::new(err.to_string().as_str()))?;
        Ok(username)
    } else {
        Err(DeleterrError::new(
            "Login failed. Username or password incorrect.",
        ))
    }
}

pub fn logout_user(session: Session) -> Result<bool, DeleterrError> {
    session.purge();

    return Ok(true);
}

pub fn validate_session(session: Session, username: String) -> Result<String, DeleterrError> {
    let user_session: Option<String> = session
        .get("message")
        .map_err(|err| DeleterrError::new(err.to_string().as_str()))?;

    match user_session {
        Some(user) => {
            if user == username {
                Ok(user)
            } else {
                Err(DeleterrError::new(
                    "Unauthorized. No session matching provided credentials.",
                ))
            }
        }
        None => Err(DeleterrError::new(
            "Unauthorized. No session matching provided credentials.",
        )),
    }
}

pub fn verify_user(unhashed_user: User) -> Result<bool, DeleterrError> {
    let user = get_user_by_username(&unhashed_user.username)?;
    let matches = user.verify_hash(unhashed_user.password)?;

    Ok(matches)
}

pub fn update_password(session: Session, new_password: String) -> Result<(), DeleterrError> {
    let user_session: Option<String> = session
        .get("message")
        .map_err(|err| DeleterrError::new(err.to_string().as_str()))?;

    match user_session {
        Some(username) => {
            let hashed_user = HashedUser::from_user(User {
                username: username,
                password: new_password,
            })?;
            let updated_user = update_user_in_store(hashed_user);

            updated_user
        }
        None => Err(DeleterrError::new(
            "Unauthorized. No session, log in again.",
        )),
    }
}

pub async fn reject_anonymous_users(
    mut req: ServiceRequest,
    next: Next<impl MessageBody + 'static>,
) -> Result<ServiceResponse<impl MessageBody>, actix_web::Error> {
    let (http_request, payload) = req.parts_mut();
    let session = Session::from_request(http_request, payload).await?;

    let session_result = session
        .get::<String>("message")
        .expect("Session store not available!");

    match session_result {
        Some(_) => next
            .call(req)
            .await
            .map(ServiceResponse::map_into_left_body),
        None => {
            let api_response: APIResponse<()> = APIResponse {
                success: false,
                data: None,
                error_msg: Some(String::from("Unathorized. Log in first.")),
            };

            let unauthorized_response = HttpResponse::Unauthorized().json(api_response);

            Ok(ServiceResponse::new(
                http_request.clone(),
                unauthorized_response.map_into_right_body(),
            ))
        }
    }
}
