use super::models::{
    api::{APIResponse, APIStatus, RequestType},
    deleterr_error::DeleterrError,
    services::ServiceInfo,
};
use actix_web::{HttpResponse, Responder};
use reqwest::{
    header::{HeaderMap, HeaderValue, ACCEPT},
    Error,
};
use serde::Serialize;
use std::time::Duration;

pub async fn make_api_call(
    client_request: reqwest::RequestBuilder,
) -> Result<reqwest::Response, DeleterrError> {
    let response = client_request.send().await?;

    let response_code = response.status().as_u16();

    match response_code {
        200 => Ok(response),
        // Overseerr delete code
        204 => Ok(response),
        404 => Err(DeleterrError::new(APIStatus::NotFound.to_string().as_str())),
        401 => Err(DeleterrError::new(APIStatus::WrongKey.to_string().as_str())),
        403 => Err(DeleterrError::new(APIStatus::WrongKey.to_string().as_str())),
        _ => Err(DeleterrError::new(APIStatus::Other.to_string().as_str())),
    }
}

pub fn process_request<T>(response: Result<T, DeleterrError>) -> impl Responder
where
    T: serde::Serialize,
{
    return match response {
        Ok(response_ok) => {
            let api_response = APIResponse {
                success: true,
                data: Some(response_ok),
                error_msg: None,
            };
            send_response(api_response)
            //HttpResponse::Ok().json(api_response)
        }
        Err(error) => {
            let err_response: APIResponse<()> = APIResponse {
                success: false,
                data: None,
                error_msg: Some(error.to_string()),
            };
            send_response(err_response)
            //HttpResponse::Ok().json(err_response)
        }
    };
}

pub fn send_response<T>(api_response: APIResponse<T>) -> HttpResponse
where
    T: serde::Serialize,
{
    return HttpResponse::Ok().json(api_response);
}

pub fn create_api_url(endpoint: &String, service_info: &ServiceInfo) -> String {
    let proto = if service_info.use_ssl {
        "https://"
    } else {
        "http://"
    };

    let mut builder = String::new();

    builder.push_str(proto);
    builder.push_str(service_info.host.as_str());

    if let Some(port) = &service_info.port {
        builder.push(':');
        builder.push_str(port.clone().to_string().as_str())
    }

    if !service_info.host.ends_with('/') {
        builder.push('/');
    }

    let result = builder;

    let url = format!("{result}{endpoint}");

    return url;
}

pub fn get_api_endpoint(
    url: String,
    query: Vec<(&str, &str)>,
    api_key: Option<String>,
    req_type: RequestType,
) -> Result<reqwest::RequestBuilder, Error> {
    let mut headers = HeaderMap::new();

    headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
    if let Some(set_api_key) = api_key {
        let str = set_api_key.as_str();
        headers.insert(
            "X-Api-Key",
            HeaderValue::from_str(str).unwrap_or(HeaderValue::from_static("")), // Handling the case where someone gives us a bad API Key value
        );
    };

    let req_client = match req_type {
        RequestType::Get => reqwest::Client::new()
            .get(url)
            .query(&query)
            .timeout(Duration::from_secs(15))
            .headers(headers),
        RequestType::Delete => reqwest::Client::new()
            .delete(url)
            .query(&query)
            .timeout(Duration::from_secs(15))
            .headers(headers),
    };

    Ok(req_client)
}

pub fn bodied_delete_api_endpoint<T>(
    url: String,
    query: Vec<(&str, &str)>,
    api_key: Option<String>,
    body: T,
) -> Result<reqwest::RequestBuilder, Error>
where
    T: Serialize,
{
    let mut headers = HeaderMap::new();

    headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
    if let Some(set_api_key) = api_key {
        let str = set_api_key.as_str();
        headers.insert(
            "X-Api-Key",
            HeaderValue::from_str(str).unwrap_or(HeaderValue::from_static("")), // Handling the case where someone gives us a bad API Key value
        );
    };

    let req_client = reqwest::Client::new()
        .delete(url)
        .query(&query)
        .timeout(Duration::from_secs(15))
        .headers(headers)
        .json(&body);

    Ok(req_client)
}
