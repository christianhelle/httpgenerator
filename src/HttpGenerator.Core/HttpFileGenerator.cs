using System.Text;
using NSwag;
using NSwag.CodeGeneration.CSharp;

namespace HttpGenerator.Core;

public static class HttpFileGenerator
{
    public static async Task<GeneratorResult> Generate(GeneratorSettings settings)
    {
        var document = await OpenApiDocumentFactory.CreateAsync(settings.OpenApiPath);
        var generator = new CSharpClientGenerator(document, new CSharpClientGeneratorSettings());
        generator.BaseSettings.OperationNameGenerator = new OperationNameGenerator(document);

        var baseUrl = settings.BaseUrl + document.Servers?.FirstOrDefault()?.Url;

        return settings.OutputType == OutputType.OneRequestPerFile
            ? GenerateMultipleFiles(settings, document, generator, baseUrl)
            : GenerateSingleFile(settings, document, baseUrl);
    }

    private static GeneratorResult GenerateSingleFile(
        GeneratorSettings settings,
        OpenApiDocument document,
        string baseUrl)
    {
        var code = new StringBuilder();
        foreach (var kv in document.Paths)
        {
            foreach (var operations in kv.Value)
            {
                code.AppendLine(
                    GenerateRequest(
                        settings,
                        baseUrl,
                        operations.Key.CapitalizeFirstCharacter(),
                        kv,
                        operations.Value));
            }
        }

        return new GeneratorResult(
            new[] { new HttpFile("Requests.http", code.ToString()) });
    }

    private static GeneratorResult GenerateMultipleFiles(
        GeneratorSettings settings,
        OpenApiDocument document,
        CSharpClientGenerator generator,
        string baseUrl)
    {
        var files = new List<HttpFile>();
        foreach (var kv in document.Paths)
        {
            foreach (var operations in kv.Value)
            {
                var operation = operations.Value;
                var verb = operations.Key.CapitalizeFirstCharacter();
                var name = generator
                    .BaseSettings
                    .OperationNameGenerator
                    .GetOperationName(document, kv.Key, verb, operation);
                var filename = $"{name.CapitalizeFirstCharacter()}.http";

                var code = GenerateRequest(settings, baseUrl, verb, kv, operation);
                files.Add(new HttpFile(filename, code));
            }
        }

        return new GeneratorResult(files);
    }

    private static string GenerateRequest(
        GeneratorSettings settings,
        string baseUrl,
        string verb,
        KeyValuePair<string, OpenApiPathItem> kv,
        OpenApiOperation operation)
    {
        var code = new StringBuilder();
        code.AppendLine($"### {verb.ToUpperInvariant()} {kv.Key} Request");
        code.AppendLine();
        code.AppendLine($"{verb.ToUpperInvariant()} {baseUrl}{kv.Key}");
        code.AppendLine("Content-Type: " + settings.ContentType);

        if (!string.IsNullOrWhiteSpace(settings.AuthorizationHeader))
        {
            code.AppendLine($"Authorization: {settings.AuthorizationHeader}");
        }

        var contentType = operation.RequestBody?.Content?.Keys
            ?.FirstOrDefault(c => c.Contains(settings.ContentType));

        code.AppendLine();
        if (operation.RequestBody?.Content is null || contentType is null)
            return code.ToString();

        var requestBody = operation.RequestBody;
        var requestBodySchema = requestBody.Content[contentType].Schema.ActualSchema;
        var requestBodyJson = requestBodySchema?.ToSampleJson()?.ToString() ?? string.Empty;

        code.AppendLine(requestBodyJson);
        return code.ToString();
    }
}