# RPaaS WASM Modules

This repo contains a couple of WASM Modules that demonstrate using WASM to implement a User RP.  There is also a dotnet test host for executing the modules outside of RPaaS.

# Modules

The modules expect input and provide output as a JSON object written to stdin and stdout respectively. 

## HelloRpaas

Implements the RPaaS User RP extension interface, all functions will parse the provided JSON input, call a host provided function to log a message and then return a response object.

## Storage 

Implements the RPaaS User RP extension interface, all functions except `ResourceCreationBegin` will parse the provided JSON input, call a host provided function to log a message and then return a response object. The `ResourceCreationBegin` function will create a storage account by parsing the id property in the request payload and using the resource group and resource name derived from the id for the storage account resource group and name in the subscription derived from the subscription_id property. The resource group is expected to exist, the location for the resource is the location provided in the request payload. If there is no location property in the request payload then the location is derived from the environemt variable `AZURE_LOCATION`. The environment variable `AZURE_MSI_TOKEN` is expected to be set to a valid MSI token.

## Testing with dotnet test host

1. Build the wasm modules = `cargo build --target wasm32-wasi`
1. cd to ./examples/testhost
1. dotnet build
1. Execute `cat payload| .\bin\Debug\net5.0\testhost.exe function wasmmodule`

e.g. to test creating a storage account
1. Get an MSI token from Azure by navigating to https://cloudshell.azure.com/ and running the command `curl 'http://169.254.169.254/metadata/identity/oauth2/token?api-version=2018-02-01&resource=https%3A%2F%2Fmanagement.azure.com%2F' -H Metadata:true -s`.
1. Set the environment variable `AZURE_MSI_TOKEN` to the MSI token returned by the commnad above.
1. Update the json in `ValidResourceCreationValidateRequest.json` so that the request id property is for a subscription and resource group that is accessible from the MSI token.
1. `cat .\ValidResourceCreationValidateRequest.json| .\bin\Debug\net5.0\testhost.exe ResourceCreationBegin ..\..\target\wasm32-wasi\debug\storage.wasm`