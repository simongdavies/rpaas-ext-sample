use std::collections::HashMap;

use rpaas::*;
use serde::{Deserialize, Serialize};
fn main() {}
#[derive(Serialize, Deserialize)]
struct ResourceProperties {
    #[serde(rename = "propertyDeployment")]
    property_deployment: String,
    #[serde(rename = "propertyString")]
    property_string: String,
    #[serde(rename = "propertyInt")]
    property_int: u32,
}

#[export_name = "ResourceCreationValidate"]
pub fn create_validate() {
    match get_payload::<ResourceProperties>() {
        Ok(_) => {
            // Validate resource here
            exit_success_with_status();
        }
        Err(e) => exit_error(e, "Resource Create Validation Error", 200),
    }
}

#[export_name = "ResourceCreationBegin"]
pub fn create_begin() {
    match get_payload::<ResourceProperties>() {
        Ok(r) => {
            // Create resource here
            exit_success_with_resource(r.resource);
        }
        Err(e) => exit_error(e, "Resource Creation Error", 500),
    }
}

#[export_name = "ResourceCreationCompleted"]
pub fn create_complete() {
    match get_payload::<ResourceProperties>() {
        Ok(_) => {
            // Create complete actions here
            exit_success_no_payload();
        }
        Err(e) => exit_error(e, "Resource Creation Complete Error", 500),
    }
}

#[export_name = "ResourceReadValidate"]
pub fn read_validate() {
    match get_payload::<ResourceProperties>() {
        Ok(_) => {
            // Validate read here
            exit_success_with_status();
        }
        Err(e) => exit_error(e, "Resource Read Validate Error", 200),
    }
}

#[export_name = "ResourceReadBegin"]
pub fn read_begin() {
    match get_payload::<ResourceProperties>() {
        Ok(r) => {
            // Read resource here
            exit_success_with_resource(r.resource);
        }
        Err(e) => exit_error(e, "Resource Creation Error", 500),
    }
}

#[export_name = "ResourcePatchValidate"]
pub fn patch_validate() {
    match get_payload::<ResourceProperties>() {
        Ok(r) => {
            // Validate Patch here
            exit_success_with_resource(r.resource);
        }
        Err(e) => exit_error(e, "Resource Patch Validate Error", 200),
    }
}

#[export_name = "ResourcePatchBegin"]
pub fn patch_begin() {
    match get_payload::<ResourceProperties>() {
        Ok(r) => {
            let mut headers = HashMap::new();
            headers.insert(
                "Content-Type".to_string(),
                vec!["application/merge-patch+json".to_string()],
            );
            // Patch here
            exit_success_with_resource_and_headers(r.resource, headers);
        }
        Err(e) => exit_error(e, "Resource Patch Begin Error", 500),
    }
}

#[export_name = "ResourcePatchCompleted"]
pub fn patch_complete() {
    match get_payload::<ResourceProperties>() {
        Ok(_) => {
            // Patch complete actions here
            exit_success_no_payload();
        }
        Err(e) => exit_error(e, "Resource Patch Complete Error", 500),
    }
}

#[export_name = "ResourcePostAction"]
pub fn action() {
    match get_payload::<ResourceProperties>() {
        Ok(_) => {
            // Patch complete actions here
            exit_success_no_payload();
        }
        Err(e) => exit_error(e, "Resource Action Error", 500),
    }
}


#[export_name = "ResourceDeletionValidate"]
pub fn delete_validate() {
    match get_payload::<ResourceProperties>() {
        Ok(_) => {
            // Validate resource here
            exit_success_with_status();
        }
        Err(e) => exit_error(e, "Resource Delete Validation Error", 200),
    }
}

#[export_name = "ResourceDeletionBegin"]
pub fn delete_begin() {
    match get_payload::<ResourceProperties>() {
        Ok(r) => {
            // Create resource here
            exit_success_with_resource(r.resource);
        }
        Err(e) => exit_error(e, "Resource Deletion Error", 500),
    }
}

#[export_name = "ResourceDeletionCompleted"]
pub fn delete_complete() {
    match get_payload::<ResourceProperties>() {
        Ok(_) => {
            // Create complete actions here
            exit_success_no_payload();
        }
        Err(e) => exit_error(e, "Resource Delete Complete Error", 500),
    }
}