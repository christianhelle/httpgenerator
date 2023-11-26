az account get-access-token `
| ConvertFrom-Json `
| %{
    dotnet run --project ../src/HttpGenerator/HttpGenerator.csproj -- `
    https://petstore3.swagger.io/api/v3/openapi.json `
        --authorization-header ("Bearer " + $_.accessToken) `
        --base-url https://petstore3.swagger.io `
        --output ./HttpFiles
        
    dotnet run --project ../src/HttpGenerator/HttpGenerator.csproj -- `
    https://petstore3.swagger.io/api/v3/openapi.json `
        --authorization-header ("Bearer " + $_.accessToken) `
        --base-url https://petstore3.swagger.io `
        --output-type OneFile
}