use crate::common::models::{ServiceInfo, Services};
use dotenv::dotenv;

fn build_service_info() -> ServiceInfo {
    dotenv().ok();
    let os_host = std::env::var("TT_HOST").expect("tt_host must be set.");
    let os_api_key = std::env::var("TT_API_KEY").expect("tt_api_key must be set.");
    let os_port = std::env::var("TT_PORT").expect("tt_port must be set.");
    let os_use_ssl = std::env::var("TT_USE_SSL").expect("tt_use_ssl must be set.");

    let parsed_port = os_port.parse::<String>().ok();

    return ServiceInfo {
        host: os_host,
        api_key: os_api_key,
        port: parsed_port,
        use_ssl: if os_use_ssl == "true" { true } else { false },
        service: Services::Overseerr,
    };
}

pub async fn delete_movie(
    rating_key: &str,
    user_id: &str,
) -> Result<TautulliResponse, DeleterrError> {
    let endpoint = format!("api/v2");
    let service_info = build_service_info();

    let api_url = create_api_url(&endpoint, &service_info);
    let query = vec![
        ("cmd", "get_history"),
        ("apikey", service_info.api_key.as_str()),
        ("rating_key", rating_key),
        ("user_id", user_id),
    ];

    let client_req = get_api_endpoint(api_url, query, None)?;
    let request_response = make_api_call(client_req).await?;
    let resp = request_response.response.json::<TautulliResponse>().await?;
    Ok(resp)
}
