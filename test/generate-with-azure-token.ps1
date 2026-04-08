az account get-access-token `
| ConvertFrom-Json `
| ForEach-Object {
  cargo run -- `
    https://petstore3.swagger.io/api/v3/openapi.json `
    --authorization-header ("Bearer " + $_.accessToken) `
    --base-url https://petstore3.swagger.io `
    --output ./HttpFiles

  cargo run -- `
    https://petstore3.swagger.io/api/v3/openapi.json `
    --authorization-header ("Bearer " + $_.accessToken) `
    --base-url https://petstore3.swagger.io `
    --output-type OneFile
}
