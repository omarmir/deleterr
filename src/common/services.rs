use super::models::{APIData, APIResponse, DeleterrError, RequestResponse, ServiceInfo};
use actix_web::{HttpResponse, Responder};
use reqwest::{
    header::{HeaderMap, HeaderValue, ACCEPT},
    Error,
};
use std::time::Duration;

pub async fn make_api_call(
    client_request: reqwest::RequestBuilder,
) -> Result<RequestResponse, Error> {
    let response = client_request.send().await?;

    let request_response: RequestResponse = RequestResponse {
        code: response.status().as_u16(),
        status: response.status().to_string(),
        response,
    };

    Ok(request_response)
}

pub fn process_request<T>(requests: Result<APIResponse<T>, DeleterrError>) -> impl Responder
where
    T: serde::Serialize,
{
    return match requests {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(error) => {
            let err_response = APIResponse {
                success: false,
                data: APIData::<String>::Failure(error.to_string()),
                code: 500,
            };
            return HttpResponse::Ok().json(err_response);
        }
    };
}

pub async fn map_to_api_response<T>(
    service_reponse: T,
    code: u16,
    failure_status: String,
) -> Result<APIResponse<T>, DeleterrError>
where
    for<'a> T: serde::Deserialize<'a>,
{
    if code != 200 {
        return Ok(APIResponse {
            success: false,
            data: APIData::Failure(failure_status),
            code,
        });
    };

    let api_response = APIResponse {
        success: true,
        data: APIData::Success(service_reponse),
        code,
    };

    Ok(api_response)
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
        builder.push_str(port.clone().as_str())
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

    let req_client = reqwest::Client::new()
        .get(url)
        .query(&query)
        .timeout(Duration::from_secs(15))
        .headers(headers);

    Ok(req_client)
}
