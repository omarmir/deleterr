use super::models::HashedUser;
use crate::{
    auth::models::User,
    common::models::{APIResponse, DeleterrError},
    store::store::{does_record_exist, get_persy},
};
use actix_session::Session;
use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    FromRequest, HttpResponse,
};
use actix_web_lab::middleware::Next;
use bcrypt::{hash, verify};
use persy::{Persy, PersyId};

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
    let persy = get_persy()?;

    let user = get_user_by_username(unhashed_user.username, &persy)?;

    match user {
        Some(user) => {
            let matches = verify(unhashed_user.password, user.hash.as_str()).map_err(|e| {
                DeleterrError::new(e.to_string().as_str())
                    .add_prefix("Unable to verify password hash.")
            })?;
            Ok(matches)
        }
        None => Ok(false),
    }
}

pub fn get_user_by_username(
    username: String,
    persy: &Persy,
) -> Result<Option<HashedUser>, DeleterrError> {
    let read_id = persy
        .get::<String, PersyId>("users_index", &username)?
        .next();

    if let Some(id) = read_id {
        let value = persy.read("users", &id)?;
        match value {
            Some(val) => Ok(Some(HashedUser::from(val))),
            None => Ok(None),
        }
    } else {
        Ok(None)
    }
}

pub fn hash_password(pass: String) -> Result<String, DeleterrError> {
    let hash = hash(pass, 10)
        .map_err(|e| {
            DeleterrError::new(e.to_string().as_str()).add_prefix("Unable to hash password.")
        })?
        .to_string();

    Ok(hash)
}

pub fn upsert_user(unhashed_user: User) -> Result<String, DeleterrError> {
    let hash = hash_password(unhashed_user.password)?;

    let user = HashedUser {
        username: unhashed_user.username,
        hash,
    };

    let persy = get_persy()?;
    //Start a transaction all the operations in persy are done inside a transaction.
    let persy_id = does_record_exist(&persy, &user.username, "users_index")?;

    let mut tx = persy.begin()?;
    let rec = &user.as_bytes();
    match persy_id {
        Some(id) => {
            tx.update("users", &id, &rec)?;
            let prepared = tx.prepare()?;
            prepared.commit()?;
            Ok(id.to_string())
        }
        None => {
            let new_id = tx.insert("users", &rec)?;

            tx.put::<String, PersyId>("users_index", user.username.to_string(), new_id)?;
            let prepared = tx.prepare()?;
            prepared.commit()?;

            Ok(new_id.to_string())
        }
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
