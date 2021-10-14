use rpaas::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub fn main() {}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ResourceProperties {
    cluster: String,
    datacenter: String,
    nodes: u32,
    service_account_name: String,
    dummy_volume: DummyVolume,
}
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DummyVolume {}

#[export_name = "ResourceCreationValidate"]
pub fn create_validate() {
    trace_info("ResourceCreationValidate Called");
    match get_payload::<ResourceProperties>() {
        Ok(r) => {
            trace_info(&format!(
                "ResourceCreationValidate Request Parsed for {}",
                r.resource_id
            ));
            // Validate resource here
            exit_success_with_status();
        }
        Err(e) => exit_error(e, "Resource Create Validation Error", 200),
    }
}

#[export_name = "ResourceCreationBegin"]
pub fn create_begin() {
    trace_info("ResourceCreationBegin Called");
    match get_payload::<ResourceProperties>() {
        Ok(r) => {
            trace_info(&format!(
                "ResourceCreationBegin Request Parsed for {}",
                r.resource_id
            ));
            // Create resource here
            exit_success_with_resource(r.request);
        }
        Err(e) => exit_error(e, "Resource Creation Error", 500),
    }
}

#[export_name = "ResourceCreationCompleted"]
pub fn create_complete() {
    trace_info("ResourceCreationCompleted Called");
    match get_payload::<ResourceProperties>() {
        Ok(r) => {
            trace_info(&format!(
                "ResourceCreationCompleted Request Parsed for {}",
                r.resource_id
            ));
            // Create complete actions here
            exit_success_no_payload();
        }
        Err(e) => exit_error(e, "Resource Creation Complete Error", 500),
    }
}

#[export_name = "ResourceReadValidate"]
pub fn read_validate() {
    trace_info("ResourceReadValidate Called");
    match get_payload::<ResourceProperties>() {
        Ok(r) => {
            trace_info(&format!(
                "ResourceReadValidate Request Parsed for {}",
                r.resource_id
            ));
            // Validate read here
            exit_success_with_status();
        }
        Err(e) => exit_error(e, "Resource Read Validate Error", 200),
    }
}

#[export_name = "ResourceReadBegin"]
pub fn read_begin() {
    trace_info("ResourceReadBegin Called");
    match get_payload::<ResourceProperties>() {
        Ok(r) => {
            trace_info(&format!(
                "ResourceReadBegin Request Parsed for {}",
                r.resource_id
            ));
            // Read resource here
            exit_success_with_resource(r.request);
        }
        Err(e) => exit_error(e, "Resource Creation Error", 500),
    }
}

#[export_name = "ResourcePatchValidate"]
pub fn patch_validate() {
    trace_info("ResourcePatchValidate Called");
    match get_payload::<ResourceProperties>() {
        Ok(r) => {
            trace_info(&format!(
                "ResourcePatchValidate Request Parsed for {}",
                r.resource_id
            ));
            // Validate Patch here
            exit_success_with_resource(r.request);
        }
        Err(e) => exit_error(e, "Resource Patch Validate Error", 200),
    }
}

#[export_name = "ResourcePatchBegin"]
pub fn patch_begin() {
    trace_info("ResourcePatchBegin Called");
    match get_payload::<ResourceProperties>() {
        Ok(r) => {
            trace_info(&format!(
                "ResourcePatchBegin Request Parsed for {}",
                r.resource_id
            ));
            let mut headers = HashMap::new();
            headers.insert(
                "Content-Type".to_string(),
                vec!["application/merge-patch+json".to_string()],
            );
            // Patch here
            exit_success_with_resource_and_headers(r.request, headers);
        }
        Err(e) => exit_error(e, "Resource Patch Begin Error", 500),
    }
}

#[export_name = "ResourcePatchCompleted"]
pub fn patch_complete() {
    trace_info("ResourcePatchCompleted Called");
    match get_payload::<ResourceProperties>() {
        Ok(r) => {
            trace_info(&format!(
                "ResourcePatchCompleted Request Parsed for {}",
                r.resource_id
            ));
            // Patch complete actions here
            exit_success_no_payload();
        }
        Err(e) => exit_error(e, "Resource Patch Complete Error", 500),
    }
}

#[export_name = "ResourcePostAction"]
pub fn action() {
    trace_info("ResourcePostAction Called");
    match get_payload::<ResourceProperties>() {
        Ok(r) => {
            trace_info(&format!(
                "ResourcePostAction Request Parsed for {}",
                r.resource_id
            ));
            // Patch complete actions here
            exit_success_no_payload();
        }
        Err(e) => exit_error(e, "Resource Action Error", 500),
    }
}

#[export_name = "ResourceDeletionValidate"]
pub fn delete_validate() {
    trace_info("ResourceDeletionValidate Called");
    match get_payload::<ResourceProperties>() {
        Ok(r) => {
            trace_info(&format!(
                "ResourceDeletionValidate Request Parsed for {}",
                r.resource_id
            ));
            // Validate resource here
            exit_success_with_status();
        }
        Err(e) => exit_error(e, "Resource Delete Validation Error", 200),
    }
}

#[export_name = "ResourceDeletionBegin"]
pub fn delete_begin() {
    trace_info("ResourceDeletionBegin Called");
    match get_payload::<ResourceProperties>() {
        Ok(r) => {
            trace_info(&format!(
                "ResourceDeletionBegin Request Parsed for {}",
                r.resource_id
            ));
            // Create resource here
            exit_success_with_resource(r.request);
        }
        Err(e) => exit_error(e, "Resource Deletion Error", 500),
    }
}

#[export_name = "ResourceDeletionCompleted"]
pub fn delete_complete() {
    trace_info("ResourceDeletionCompleted Called");
    match get_payload::<ResourceProperties>() {
        Ok(r) => {
            trace_info(&format!(
                "ResourceDeletionCompleted Request Parsed for {}",
                r.resource_id
            ));
            // Create complete actions here
            exit_success_no_payload();
        }
        Err(e) => exit_error(e, "Resource Delete Complete Error", 500),
    }
}
