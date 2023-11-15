[![Build](https://github.com/christianhelle/httpgenerator/actions/workflows/build.yml/badge.svg)](https://github.com/christianhelle/httpgenerator/actions/workflows/build.yml)

# HTTP File Generator

Generate .http files from OpenAPI specifications

`.http` files were made populate by the Visual Studio Code extension [REST Client](https://marketplace.visualstudio.com/items?itemName=humao.rest-client), which then was adopted by JetBrains IDE's, and later on [Visual Studio 2022](https://marketplace.visualstudio.com/items?itemName=MadsKristensen.RestClient)

## Installation

This is tool is distrubuted as a .NET Tool on NuGet.org

To install, simply use the following command

```bash
dotnet tool install --global httpgenerator
```

## Usage

```
USAGE:
    httpgenerator [URL or input file] [OPTIONS]

EXAMPLES:
    httpgenerator ./openapi.json
    httpgenerator ./openapi.json --output ./
    httpgenerator https://petstore.swagger.io/v2/swagger.json
    httpgenerator https://petstore3.swagger.io/api/v3/openapi.json --base-url https://petstore3.swagger.io
    httpgenerator ./openapi.json --authorization-header Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c

ARGUMENTS:
    [URL or input file]    URL or file path to OpenAPI Specification file

OPTIONS:
                                           DEFAULT                                                                                                                           
    -h, --help                                                 Prints help information                                                                                       
    -o, --output <OUTPUT>                  ./                  Output directory                                                                                              
        --no-logging                                           Don't log errors or collect telemetry                                                                         
        --skip-validation                                      Skip validation of OpenAPI Specification file                                                                 
        --authorization-header <HEADER>                        Authorization header to use for all requests                                                                  
        --content-type <CONTENT-TYPE>      application/json    Default Content-Type header to use for all requests                                                           
        --base-url <BASE-URL>                                  Default Base URL to use for all requests. Use this if the OpenAPI spec doesn't explicitly specify a server URL
```

Running the following:

```sh
httpgenerator https://petstore.swagger.io/v2/swagger.json
```

Outputs the following:

```
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
-rw-r--r--  1 christian  staff  299 Nov 13 22:40 AddPet.http
-rw-r--r--  1 christian  staff  276 Nov 13 22:40 CreateUser.http
-rw-r--r--  1 christian  staff  332 Nov 13 22:40 CreateUsersWithArrayInput.http
-rw-r--r--  1 christian  staff  330 Nov 13 22:40 CreateUsersWithListInput.http
-rw-r--r--  1 christian  staff  135 Nov 13 22:40 DeleteOrder.http
-rw-r--r--  1 christian  staff  115 Nov 13 22:40 DeletePet.http
-rw-r--r--  1 christian  staff  123 Nov 13 22:40 DeleteUser.http
-rw-r--r--  1 christian  staff  119 Nov 13 22:40 FindPetsByStatus.http
-rw-r--r--  1 christian  staff  115 Nov 13 22:40 FindPetsByTags.http
-rw-r--r--  1 christian  staff  117 Nov 13 22:40 GetInventory.http
-rw-r--r--  1 christian  staff  129 Nov 13 22:40 GetOrderById.http
-rw-r--r--  1 christian  staff  109 Nov 13 22:40 GetPetById.http
-rw-r--r--  1 christian  staff  117 Nov 13 22:40 GetUserByName.http
-rw-r--r--  1 christian  staff  107 Nov 13 22:40 LoginUser.http
-rw-r--r--  1 christian  staff  109 Nov 13 22:40 LogoutUser.http
-rw-r--r--  1 christian  staff  250 Nov 13 22:40 PlaceOrder.http
-rw-r--r--  1 christian  staff  297 Nov 13 22:40 UpdatePet.http
-rw-r--r--  1 christian  staff  111 Nov 13 22:40 UpdatePetWithForm.http
-rw-r--r--  1 christian  staff  296 Nov 13 22:40 UpdateUser.http
-rw-r--r--  1 christian  staff  135 Nov 13 22:40 UploadFile.http

```

In this example, the contents of `AddPet.http` looks like this:

```sh
### POST /pet Request

POST https://petstore.swagger.io/v2/pet
Content-Type: application/json

{
  "id": 0,
  "category": {
    "id": 0,
    "name": "name"
  },
  "name": "name",
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

Here's an advanced example of generating `.http` files for a REST API hosted on Microsoft Azure that uses the Microsoft Entra ID service as an STS. For this example, I use Azure CLI to retrieve an access token for the user I'm currently logged in with.

```powershell
az account get-access-token --scope [Some Application ID URI]/.default `
| ConvertFrom-Json `
| %{
    httpgenerator `
        https://api.example.com/swagger/v1/swagger.json `
        --authorization-header ("Bearer " + $_.accessToken) `
        --base-url https://api.example.com
        --output ./HttpFiles 
}
```
