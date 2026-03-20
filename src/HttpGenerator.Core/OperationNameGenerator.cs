using System.Diagnostics;
using System.Diagnostics.CodeAnalysis;
using Microsoft.OpenApi;

namespace HttpGenerator.Core;

internal interface IOperationNameGenerator
{
    string GetOperationName(
        OpenApiDocument document,
        string path,
        string httpMethod,
        OpenApiOperation operation);
}

internal class OperationNameGenerator : IOperationNameGenerator
{
    public string GetOperationName(
        OpenApiDocument document,
        string path,
        string httpMethod,
        OpenApiOperation operation)
    {
        try
        {
            // Try to use operationId first if available
            var operationName = string.IsNullOrWhiteSpace(operation.OperationId)
                ? $"{httpMethod}_{path}"
                : operation.OperationId!;
            
            return operationName
                .CapitalizeFirstCharacter()
                .ConvertKebabCaseToPascalCase()
                .ConvertRouteToCamelCase()
                .ConvertSpacesToPascalCase()
                .Prefix(
                    httpMethod
                        .ToLowerInvariant()
                        .CapitalizeFirstCharacter());
        }
        catch (Exception e)
        {
            Trace.TraceError(e.ToString());
            return httpMethod.CapitalizeFirstCharacter() + 
                   path.ConvertRouteToCamelCase()
                       .ConvertSpacesToPascalCase();
        }
    }
}
