﻿using System.Diagnostics;
using System.Diagnostics.CodeAnalysis;
using NSwag;
using NSwag.CodeGeneration.OperationNameGenerators;

namespace HttpGenerator.Core;

internal class OperationNameGenerator : IOperationNameGenerator
{
    private readonly IOperationNameGenerator defaultGenerator =
        new MultipleClientsFromOperationIdOperationNameGenerator();

    [ExcludeFromCodeCoverage]
    public bool SupportsMultipleClients => throw new NotImplementedException();

    [ExcludeFromCodeCoverage]
    public string GetClientName(OpenApiDocument document, string path, string httpMethod, OpenApiOperation operation)
    {
        return defaultGenerator.GetClientName(document, path, httpMethod, operation);
    }

    public string GetOperationName(
        OpenApiDocument document,
        string path,
        string httpMethod,
        OpenApiOperation operation)
    {
        try
        {
            return defaultGenerator
                .GetOperationName(document, path, httpMethod, operation)
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

    public bool CheckForDuplicateOperationIds(
        OpenApiDocument document)
    {
        List<string> operationNames = new();
        foreach (var kv in document.Paths)
        {
            foreach (var operations in kv.Value)
            {
                var operation = operations.Value;
                operationNames.Add(
                    GetOperationName(
                        document,
                        kv.Key,
                        operations.Key,
                        operation));
            }
        }

        return operationNames.Distinct().Count() != operationNames.Count;
    }
}