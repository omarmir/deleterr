use polodb_core::{bson::Bson, Database, Error};

use crate::common::{
    models::{APIResponse, DeleterrError, ServiceInfo},
    services::map_to_api_response,
};

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
