use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::io::{stdin, Read};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request<T> {
    pub resource_id: String,
    pub request: Resource<T>,
    pub headers: HashMap<String, Vec<String>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Resource<T> {
    id: String,
    name: String,
    #[serde(rename = "type")]
    resource_type: String,
    properties: T,
}

#[derive(Serialize, Deserialize)]
struct ResponsePayload {
    status: String,
    #[serde(rename = "error")]
    error: Option<RPaaSError>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Response<T: Serialize> {
    http_status_code: i32,
    payload: Option<T>,
    headers: HashMap<String, Vec<String>>,
}

#[derive(Serialize, Deserialize)]
struct RPaaSError {
    code: String,
    message: String,
}

pub fn stdin_as_request<T: DeserializeOwned>() -> Result<T, Box<dyn Error>> {
    let mut data = String::new();
    let mut stdin = stdin();
    stdin.read_to_string(&mut data)?;
    let request: T = serde_json::from_str::<T>(&data)?;
    Ok(request)
}

pub fn exit_success_with_headers(headers: HashMap<String, Vec<String>>) {
    let response = Response {
        http_status_code: 200,
        payload: Some(ResponsePayload {
            status: "Succeeded".to_owned(),
            error: None,
        }),
        headers,
    };
    let response_json = serde_json::to_string_pretty(&response).unwrap();
    println!("{}", response_json);
}

pub fn exit_success_with_status() {
    exit_success_with_headers(HashMap::new());
}

pub fn exit_success_with_resource_and_headers<T: Serialize>(
    resource: T,
    headers: HashMap<String, Vec<String>>,
) {
    let response = Response::<T> {
        http_status_code: 200,
        payload: Some(resource),
        headers,
    };
    let response_json = serde_json::to_string_pretty(&response).unwrap();
    println!("{}", response_json);
}

pub fn exit_success_with_resource<T: Serialize>(resource: T) {
    exit_success_with_resource_and_headers(resource, HashMap::new());
}

pub fn exit_success_no_payload() {
    let response = Response::<()> {
        http_status_code: 200,
        payload: None,
        headers: HashMap::new(),
    };
    let response_json = serde_json::to_string_pretty(&response).unwrap();
    println!("{}", response_json);
}

pub fn exit_error_with_headers(
    err: Box<dyn Error>,
    errorcode: &str,
    statuscode: i32,
    headers: HashMap<String, Vec<String>>,
) {
    let response = Response {
        http_status_code: statuscode,
        payload: Some(ResponsePayload {
            status: "Failed".to_owned(),
            error: Some(RPaaSError {
                code: errorcode.to_owned(),
                message: err.to_string(),
            }),
        }),
        headers,
    };
    let response_json = serde_json::to_string_pretty(&response).unwrap();
    println!("{}", response_json);
    trace_error(&format!(
        "Error Code:{} Message: {}  Status: {}",
        errorcode,
        err.to_string(),
        statuscode
    ));
}

pub fn exit_error(err: Box<dyn Error>, errorcode: &str, statuscode: i32) {
    exit_error_with_headers(err, errorcode, statuscode, HashMap::new());
}

pub fn get_payload<T: DeserializeOwned>() -> Result<Request<T>, Box<dyn Error>> {
    stdin_as_request::<Request<T>>()
}

pub fn trace_info(message: &str) {
    #[link(wasm_import_module = "rpaas_host")]
    extern "C" {
        fn TraceInfo(ptr: *const u8, len: usize);
    }
    let ptr = message.as_ptr();
    let len = message.len();
    unsafe { TraceInfo(ptr, len) };
}

pub fn trace_error(message: &str) {
    #[link(wasm_import_module = "rpaas_host")]
    extern "C" {
        fn TraceError(ptr: *const u8, len: usize);
    }
    let ptr = message.as_ptr();
    let len = message.len();
    unsafe { TraceError(ptr, len) };
}

pub fn trace_warning(message: &str) {
    #[link(wasm_import_module = "rpaas_host")]
    extern "C" {
        fn TraceWarning(ptr: *const u8, len: usize);
    }
    let ptr = message.as_ptr();
    let len = message.len();
    unsafe { TraceWarning(ptr, len) };
}
