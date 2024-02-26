use super::common::get_data;
use crate::common::models::{deleterr_error::DeleterrError, services::ServiceInfo};

const BUCKET_NAME: &str = "services";

pub fn get_service(service_name: String) -> Result<Option<ServiceInfo>, DeleterrError> {
    let services_data = get_data(BUCKET_NAME, service_name).unwrap_or(None);

    match services_data {
        Some(data) => {
            let service_info: ServiceInfo = ServiceInfo::from(data);
            Ok(Some(service_info))
        }
        None => Ok(None),
    }
}
