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
httpgenerator https://petstore3.swagger.io/api/v3/openapi.json
```

Outputs the following:

```
HTTP File Generator v0.1.0
Support key: mbmbqvd

OpenAPI statistics:
 - Path Items: 13
 - Operations: 19
 - Parameters: 17
 - Request Bodies: 9
 - Responses: 19
 - Links: 0
 - Callbacks: 0
 - Schemas: 73

Files: 19
Duration: 00:00:02.3426902
```

Which will produce the following files:

```sh
-rw-r--r--  1 christian  staff  276 Nov 13 22:31 AddPet.http
-rw-r--r--  1 christian  staff  253 Nov 13 22:31 CreateUser.http
-rw-r--r--  1 christian  staff  307 Nov 13 22:31 CreateUsersWithListInput.http
-rw-r--r--  1 christian  staff  112 Nov 13 22:31 DeleteOrder.http
-rw-r--r--  1 christian  staff   92 Nov 13 22:31 DeletePet.http
-rw-r--r--  1 christian  staff  100 Nov 13 22:31 DeleteUser.http
-rw-r--r--  1 christian  staff   96 Nov 13 22:31 FindPetsByStatus.http
-rw-r--r--  1 christian  staff   92 Nov 13 22:31 FindPetsByTags.http
-rw-r--r--  1 christian  staff   94 Nov 13 22:31 GetInventory.http
-rw-r--r--  1 christian  staff  106 Nov 13 22:31 GetOrderById.http
-rw-r--r--  1 christian  staff   86 Nov 13 22:31 GetPetById.http
-rw-r--r--  1 christian  staff   94 Nov 13 22:31 GetUserByName.http
-rw-r--r--  1 christian  staff   84 Nov 13 22:31 LoginUser.http
-rw-r--r--  1 christian  staff   86 Nov 13 22:31 LogoutUser.http
-rw-r--r--  1 christian  staff  227 Nov 13 22:31 PlaceOrder.http
-rw-r--r--  1 christian  staff  274 Nov 13 22:31 UpdatePet.http
-rw-r--r--  1 christian  staff   88 Nov 13 22:31 UpdatePetWithForm.http
-rw-r--r--  1 christian  staff  273 Nov 13 22:31 UpdateUser.http
-rw-r--r--  1 christian  staff  112 Nov 13 22:31 UploadFile.http
```

In this example, the contents of `AddPet.http` looks like this:

```sh
### POST /pet Request

POST /api/v3/pet
Content-Type: application/json

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