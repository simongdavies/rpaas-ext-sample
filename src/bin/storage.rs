use azure_mgmt_storage::models::{Sku, SkuName, StorageAccountCreateParameters};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::time::Duration;

use azure_core::*;
use futures::executor::block_on;
use std::error::Error;
use std::sync::Arc;

use azure_identity::token_credentials::EnvironmentVariableCredential;
use azure_mgmt_storage::models::storage_account_properties;
use azure_mgmt_storage::operations::storage_accounts;
use azure_mgmt_storage::operations::storage_accounts::{create, get_properties};
use rpaas::*;
use thiserror::Error;

const LOCATION_ENV_VAR_NAME: &str = "AZURE_LOCATION";

#[derive(Error, Debug)]
pub enum RPaaSError {
    #[error("{0}")]
    AsyncCheck(String),
    #[error(
        "Failed to create storage account. Subscription {} resource group {} name {} : {}",
        subscription_id,
        resource_group_name,
        name,
        source
    )]
    CreateFailed {
        subscription_id: String,
        resource_group_name: String,
        name: String,
        source: create::Error,
    },
    #[error(
        "Failed to get storage account for async create. Subscription {} resource group {} name {} : {}",
        subscription_id,
        resource_group_name,
        name,
        source
    )]
    GetFailed {
        subscription_id: String,
        resource_group_name: String,
        name: String,
        source: get_properties::Error,
    },
    #[error(
        "Failed to create storage account. Subscription {} resource group {} name {} Timed out waiting for async operation to complete",
        subscription_id,
        resource_group_name,
        name
    )]
    AsyncCheckTimedOut {
        subscription_id: String,
        resource_group_name: String,
        name: String,
    },
    #[error("Resource Location is not set in payload or environment variable {}", LOCATION_ENV_VAR_NAME)]
    ResourceLocationNotSet,
}

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
            let resource_details = get_resource_details_from_resource_id(r.resource_id);
            match block_on(create_storage_account(
                resource_details.subscription_id,
                resource_details.resource_group_name,
                resource_details.name,
                r.request.location.to_owned(),
            )) {
                Ok(_) => exit_success_with_resource(r.request),
                Err(e) => exit_error(e, "Failed to create storage account", 500),
            }
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

async fn create_storage_account(
    subscription_id: String,
    resource_group_name: String,
    name: String,
    location: String,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    trace_info(&format!(
        "create_storage_account called subscription {} resource group {} name {}",
        subscription_id, resource_group_name, name
    ));
    let http_client: std::sync::Arc<Box<dyn azure_core::HttpClient>> =
        Arc::new(Box::new(WasiHttpClient {}));
    let token_credential = EnvironmentVariableCredential {};
    let config = &azure_mgmt_storage::config(http_client, Box::new(token_credential)).build();
    let mut resource_location = location;
    if resource_location.is_empty() {
        resource_location = match env::var(LOCATION_ENV_VAR_NAME) {
            Ok(loc) => loc,
            Err(_e) => return Err(Box::new(RPaaSError::ResourceLocationNotSet {})),
        };
    }

    let parameters = &StorageAccountCreateParameters {
        sku: Sku {
            name: SkuName::StandardGrs,
            tier: None,
        },
        location: resource_location,
        extended_location: None,
        identity: None,
        kind: azure_mgmt_storage::models::storage_account_create_parameters::Kind::StorageV2,
        properties: None,
        tags: None,
    };

    let response = storage_accounts::create(
        config,
        &resource_group_name,
        &name,
        parameters,
        &subscription_id,
    )
    .await
    .map_err(|err| RPaaSError::CreateFailed {
        subscription_id: subscription_id.to_owned(),
        resource_group_name: resource_group_name.to_owned(),
        name: name.to_owned(),
        source: err,
    })?;

    match response {
        storage_accounts::create::Response::Ok200(account) => {
            trace_info(&format!(
                "created_storage_account {}",
                account
                    .tracked_resource
                    .resource
                    .id
                    .ok_or("Expected resource Id")?
            ));
            Ok(())
        }
        storage_accounts::create::Response::Accepted202 => {
            get_storage_account(&subscription_id, &resource_group_name, &name).await?;
            Ok(())
        }
    }
}

async fn get_storage_account(
    subscription_id: &str,
    resource_group_name: &str,
    name: &str,
) -> Result<(), RPaaSError> {
    trace_info(&format!(
        "get_storage_account subscription {} resource group {} name {}",
        subscription_id, resource_group_name, name
    ));
    const ATTEMPT_LIMIT: i32 = 60;
    let http_client: std::sync::Arc<Box<dyn azure_core::HttpClient>> =
        Arc::new(Box::new(WasiHttpClient {}));
    let token_credential = EnvironmentVariableCredential {};
    let config = &azure_mgmt_storage::config(http_client, Box::new(token_credential)).build();
    let mut attempts = 0;
    let mut result = Ok(());

    loop {
        let response = storage_accounts::get_properties(
            config,
            resource_group_name,
            name,
            subscription_id,
            None,
        )
        .await
        .map_err(|err| RPaaSError::GetFailed {
            subscription_id: subscription_id.to_owned(),
            resource_group_name: resource_group_name.to_owned(),
            name: name.to_owned(),
            source: err,
        });
        match response {
            Ok(storage_account) => {
                if storage_account
                    .properties
                    .unwrap()
                    .provisioning_state
                    .unwrap()
                    == storage_account_properties::ProvisioningState::Succeeded
                {
                    trace_info(&format!(
                        "storage account created asynchronously subscription {} resource group {} name {}",
                        subscription_id, resource_group_name, name));
                    return Ok(());
                }
                if attempts < ATTEMPT_LIMIT {
                    attempts += 1;
                    std::thread::sleep(Duration::from_secs(2));
                } else {
                    result = Err(RPaaSError::AsyncCheckTimedOut {
                        subscription_id: subscription_id.to_owned(),
                        resource_group_name: resource_group_name.to_owned(),
                        name: name.to_owned(),
                    });
                    break;
                }
            }
            Err(err) => return Err(err),
        }
    }

    if let Err(err) = result {
        return Err(err);
    }
    Ok(())
}
