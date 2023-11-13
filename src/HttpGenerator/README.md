# HTTP File Generator
Generate .http files from OpenAPI specifications

`.http` files were made populate by the Visual Studio Code extension REST Client, which then was adopted by the JetBrains IDE's, and later on Visual Studio 2022

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
    httpgenerator https://petstore3.swagger.io/api/v3/openapi.yaml
    httpgenerator ./openapi.json --output ./

ARGUMENTS:
    [URL or input file]    URL or file path to OpenAPI Specification file

OPTIONS:
                             DEFAULT                                         
    -h, --help                          Prints help information              
    -o, --output <OUTPUT>    ./         Output directory                     
        --no-logging                    Don't log errors or collect telemetry
```

Running the following:

```sh
httpgenerator https://petstore.swagger.io/v2/swagger.json
```

Outputs the following:

```
HTTP File Generator v0.1.0
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