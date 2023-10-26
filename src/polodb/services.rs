use polodb_core::{
    bson::{doc, Bson},
    Database, Error,
};

use crate::{common::{
    models::{APIResponse, DeleterrError, ServiceInfo},
    services::map_to_api_response,
}, deleterr::models::RequestStatus};

pub fn get_database() -> Result<Database, Error> {
    let db = Database::open_file("deleterr.db")?;
    Ok(db)
}

pub async fn save_service(
    db: &Database,
    service_info: ServiceInfo,
) -> Result<APIResponse<Bson>, DeleterrError> {
    let collection = db.collection::<ServiceInfo>("services");
    let insert_result = collection.insert_one(service_info)?;
    let api_response =
        map_to_api_response(insert_result.inserted_id, 200, "Failure".to_string()).await?;

    Ok(api_response)
}

pub async fn get_service(
    db: &Database,
    service_name: Option<String>,
) -> Result<APIResponse<Vec<ServiceInfo>>, DeleterrError> {
    let collection = db.collection::<ServiceInfo>("services");

    let service = match service_name {
        Some(service_name) => collection.find(doc! {
            "service": service_name,
        }),
        _ => collection.find(None),
    }?;

    let results = service.filter_map(|service_info| service_info.ok()).collect();

    let api_response = map_to_api_response(results, 200, "Failure".to_string()).await?;

    Ok(api_response)
}

pub async fn _get_requests(
    db: &Database,
    take: usize,
    skip: usize,
) -> Result<APIResponse<Vec<RequestStatus>>, DeleterrError> {
    let collection = db.collection::<RequestStatus>("requests");
    let requests = collection.find(None)?;
    let results = requests.skip(skip).take(take).filter_map(|request| request.ok()).collect();
    let api_response = map_to_api_response(results, 200, "Failure".to_string()).await?;

    Ok(api_response)
}
