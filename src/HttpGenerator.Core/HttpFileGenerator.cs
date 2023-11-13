using System.Text;
using System.Text.Json;
using NSwag;
using NSwag.CodeGeneration.CSharp;

namespace HttpGenerator.Core;

public static class HttpFileGenerator
{
    public static async Task<GeneratorResult> Generate(string openApiPath)
    {
        var document = await OpenApiDocumentFactory.CreateAsync(openApiPath);
        var generator = new CSharpClientGenerator(document, new CSharpClientGeneratorSettings());
        var baseUrl = document.Servers.First().Url;
        
        var files = new List<HttpFile>();
        foreach (var kv in document.Paths)
        {
            foreach (var operations in kv.Value)
            {
                var operation = operations.Value;
                var verb = operations.Key.CapitalizeFirstCharacter();
                var name = GenerateOperationName(kv.Key, verb, operation, document, generator);
                var filename = $"{name.CapitalizeFirstCharacter()}.http";
                
                var code = new StringBuilder();
                code.AppendLine($"### {verb.ToUpperInvariant()} {kv.Key} Request");
                code.AppendLine();
                code.AppendLine($"{verb.ToUpperInvariant()} {baseUrl}{kv.Key}");
                code.AppendLine("Content-Type: application/json");
                code.AppendLine();
                
                if (operation.RequestBody?.Content?.ContainsKey("application/json") == true)
                {
                    var requestBody = operation.RequestBody;
                    var requestBodySchema = requestBody.Content["application/json"].Schema.ActualSchema;
                    var requestBodyJson = requestBodySchema.ToSampleJson().ToString();

                    if (requestBodySchema.Example != null)
                    {
                        requestBodyJson = JsonSerializer.Serialize(requestBodySchema.Example);
                    }
                    
                    code.AppendLine(requestBodyJson);
                }
                
                files.Add(new HttpFile(filename, code.ToString()));
            }
        }

        return new GeneratorResult(files);
    }

    private static string GenerateOperationName(
        string path,
        string verb,
        OpenApiOperation operation,
        OpenApiDocument document,
        CSharpClientGenerator generator,
        bool capitalizeFirstCharacter = false)
    {
        var operationName = generator
            .BaseSettings
            .OperationNameGenerator
            .GetOperationName(document, path, verb, operation);

        if (capitalizeFirstCharacter)
            operationName = operationName.CapitalizeFirstCharacter();

        return operationName;
    }
}