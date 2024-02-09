# HTTP File Generator

Generate .http files from OpenAPI specifications

`.http` files were made popular by the Visual Studio Code extension [REST Client](https://marketplace.visualstudio.com/items?itemName=humao.rest-client), which then was adopted by JetBrains IDE's, and later on [Visual Studio 2022](https://marketplace.visualstudio.com/items?itemName=MadsKristensen.RestClient)

## Usage

```pwsh
USAGE:
    httpgenerator [URL or input file] [OPTIONS]

EXAMPLES:
    httpgenerator ./openapi.json
    httpgenerator ./openapi.json --output ./
    httpgenerator ./openapi.json --output-type onefile
    httpgenerator https://petstore.swagger.io/v2/swagger.json
    httpgenerator https://petstore3.swagger.io/api/v3/openapi.json --base-url https://petstore3.swagger.io
    httpgenerator ./openapi.json --authorization-header Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c
    httpgenerator ./openapi.json --azure-scope [Some Application ID URI]/.default

ARGUMENTS:
    [URL or input file]    URL or file path to OpenAPI Specification file

OPTIONS:
                                                                DEFAULT                                                                                                                                                                                       
    -h, --help                                                                       Prints help information                                                                                                                                                  
    -v, --version                                                                    Prints version information                                                                                                                                               
    -o, --output <OUTPUT>                                       ./                   Output directory                                                                                                                                                         
        --no-logging                                                                 Don't log errors or collect telemetry                                                                                                                                    
        --skip-validation                                                            Skip validation of OpenAPI Specification file                                                                                                                            
        --authorization-header <HEADER>                                              Authorization header to use for all requests                                                                                                                             
        --load-authorization-header-from-environment                                 Load the authorization header from an environment variable or define it in the .http file. You can use --authorization-header-variable-name to specify the environment   
                                                                                     variable name                                                                                                                                                            
        --authorization-header-variable-name <VARIABLE-NAME>    authorization        Name of the environment variable to load the authorization header from                                                                                                   
        --content-type <CONTENT-TYPE>                           application/json     Default Content-Type header to use for all requests                                                                                                                      
        --base-url <BASE-URL>                                                        Default Base URL to use for all requests. Use this if the OpenAPI spec doesn't explicitly specify a server URL                                                           
        --output-type <OUTPUT-TYPE>                             OneRequestPerFile    OneRequestPerFile generates one .http file per request. OneFile generates a single .http file for all requests                                                           
        --azure-scope <SCOPE>                                                        Azure Entra ID Scope to use for retrieving Access Token for Authorization header                                                                                         
        --azure-tenant-id <TENANT-ID>                                                Azure Entra ID Tenant ID to use for retrieving Access Token for Authorization header                                                                                     
```

Running the following:

```sh
httpgenerator https://petstore.swagger.io/v2/swagger.json
```

Outputs the following:

```sh
HTTP File Generator v0.1.1
Support key: mbmbqvd

OpenAPI statistics:
 - Path Items: 14
 - Operations: 20
 - Parameters: 14
 - Request Bodies: 9
 - Responses: 20
 - Links: 0
 - Callbacks: 0
 - Schemas: 67

Files: 20
Duration: 00:00:02.3089450
```

Which will produce the following files:

```sh
-rw-r--r-- 1 christian 197121  593 Dec 10 10:44 DeleteOrder.http        
-rw-r--r-- 1 christian 197121  231 Dec 10 10:44 DeletePet.http
-rw-r--r-- 1 christian 197121  358 Dec 10 10:44 DeleteUser.http
-rw-r--r-- 1 christian 197121  432 Dec 10 10:44 GetFindPetsByStatus.http
-rw-r--r-- 1 christian 197121  504 Dec 10 10:44 GetFindPetsByTags.http  
-rw-r--r-- 1 christian 197121  371 Dec 10 10:44 GetInventory.http       
-rw-r--r-- 1 christian 197121  247 Dec 10 10:44 GetLoginUser.http       
-rw-r--r-- 1 christian 197121  291 Dec 10 10:44 GetLogoutUser.http      
-rw-r--r-- 1 christian 197121  540 Dec 10 10:44 GetOrderById.http
-rw-r--r-- 1 christian 197121  275 Dec 10 10:44 GetPetById.http
-rw-r--r-- 1 christian 197121  245 Dec 10 10:44 GetUserByName.http
-rw-r--r-- 1 christian 197121  513 Dec 10 10:44 PostAddPet.http
-rw-r--r-- 1 christian 197121  521 Dec 10 10:44 PostCreateUser.http
-rw-r--r-- 1 christian 197121  610 Dec 10 10:44 PostCreateUsersWithListInput.http
-rw-r--r-- 1 christian 197121  464 Dec 10 10:44 PostPlaceOrder.http
-rw-r--r-- 1 christian 197121  299 Dec 10 10:44 PostUpdatePetWithForm.http
-rw-r--r-- 1 christian 197121  274 Dec 10 10:44 PostUploadFile.http
-rw-r--r-- 1 christian 197121  513 Dec 10 10:44 PutUpdatePet.http
-rw-r--r-- 1 christian 197121  541 Dec 10 10:44 PutUpdateUser.http
```

In this example, the contents of `PostAddPet.http` looks like this:

```sh
@contentType = application/json

#############################################
### Request: POST /pet
### Summary: Add a new pet to the store
### Description: Add a new pet to the store
#############################################

POST https://petstore3.swagger.io/api/v3/pet
Content-Type: {{contentType}}

{
  "id": 0,
  "name": "name",
  "category": {
    "id": 0,
    "name": "name"
  },
  "photoUrls": [
    ""
  ],
  "tags": [
    {
      "id": 0,
      "name": "name"
    }
  ],
  "status": "available"
}
```

and the contents of `GetPetById.http` looks like this:

```sh
@contentType = application/json

#######################################
### Request: GET /pet/{petId}
### Summary: Find pet by ID
### Description: Returns a single pet
#######################################

### Path Parameter: ID of pet to return
@petId = 0

GET https://petstore3.swagger.io/api/v3/pet/{{petId}}
Content-Type: {{contentType}}
```

Here's an advanced example of generating `.http` files for a REST API hosted on Microsoft Azure that uses the Microsoft Entra ID service as an STS. For this example, I use PowerShell and Azure CLI to retrieve an access token for the user I'm currently logged in with.

```powershell
az account get-access-token --scope [Some Application ID URI]/.default `
| ConvertFrom-Json `
| %{
    httpgenerator `
        https://api.example.com/swagger/v1/swagger.json `
        --authorization-header ("Bearer " + $_.accessToken) `
        --base-url https://api.example.com `
        --output ./HttpFiles 
}
```

You can also use the `--azure-scope` and `azure-tenant-id` arguments internally use `DefaultAzureCredentials` from the `Microsoft.Extensions.Azure` NuGet package to retrieve an access token for the specified `scope`.

```powershell
httpgenerator `
  https://api.example.com/swagger/v1/swagger.json `
  --azure-scope [Some Application ID URI]/.default `
  --base-url https://api.example.com `
  --output ./HttpFiles 
```

#

For tips and tricks on software development, check out [my blog](https://christianhelle.com)

If you find this useful and feel a bit generous then feel free to [buy me a coffee ☕](https://www.buymeacoffee.com/christianhelle)
