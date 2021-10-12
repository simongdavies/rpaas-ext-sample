use std::io::{Read, stdin};
use serde::{Serialize, Deserialize};

fn main() {}    

#[derive(Serialize, Deserialize)]
struct Resource {
    id: String,
    name: String,
    #[serde(rename = "type")]
    resource_type: String,
    properties: ResourceProperties,
}

#[derive(Serialize, Deserialize)]
struct ResourceProperties {
    #[serde(rename = "propertyDeployment")]
    property_deployment: String,
    #[serde(rename = "propertyString")]
    property_string: String,
    #[serde(rename = "propertyInt")]
    property_int: u32,
}


#[derive(Serialize, Deserialize)]
struct Response{
    status: String,
    error: Option<Error>,
}
#[derive(Serialize, Deserialize)]
struct Error{
    code: String,
    message: String,
}

#[export_name = "validateresource"]
pub fn validate() {
    let mut data = String::new();
    let mut stdin = stdin();
    stdin.read_to_string(&mut data).unwrap();
    let _resource: Resource = serde_json::from_str(&data).unwrap();
    let response = Response{ 
        status: String::from("Succeeded"),
        error: None,
    };
    write_response(response);
    std::process::exit(0);
}

fn write_response(response: Response) {
    let response_json = serde_json::to_string(&response).unwrap();
    println!("{}", response_json);
}