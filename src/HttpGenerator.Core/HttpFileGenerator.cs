using System.Text;
using NSwag;
using NSwag.CodeGeneration.CSharp;

namespace HttpGenerator.Core;

public class HttpFileGenerator
{
    public static async Task<GeneratorResult> Generate(string openApiPath)
    {
        var document = await OpenApiDocumentFactory.CreateAsync(openApiPath);
        var generator = new CSharpClientGenerator(document, new CSharpClientGeneratorSettings());

        var files = new List<HttpFile>();
        foreach (var kv in document.Paths)
        {
            foreach (var operations in kv.Value)
            {
                var operation = operations.Value;
                var verb = operations.Key.CapitalizeFirstCharacter();
                var name = GenerateOperationName(kv.Key, verb, operation, document, generator);
                var filename = $"{name}.http";
                
                var code = new StringBuilder();
                // Build .http content
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