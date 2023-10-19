use super::models::{APIData, APIResponse, RequestResponse};
use actix_web::{HttpResponse, Responder};
use reqwest::Error;

pub async fn make_api_call(client_request: reqwest::RequestBuilder) -> Result<RequestResponse, Error> {
    let response = client_request.send().await?;

    let request_response: RequestResponse = RequestResponse {
        code: response.status().as_u16(),
        status: response.status().to_string(),
        response,
    };

    Ok(request_response)
}

pub fn process_request<T>(requests: Result<APIResponse<T>, Error>) -> impl Responder
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
    status: String,
) -> Result<APIResponse<T>, Error>
where
    for<'a> T: serde::Deserialize<'a>,
{
    if code != 200 {
        return Ok(APIResponse {
            success: false,
            data: APIData::Failure(status),
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