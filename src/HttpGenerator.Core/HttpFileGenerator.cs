using System.Text;
using NJsonSchema;
using NSwag;
using NSwag.CodeGeneration;
using NSwag.CodeGeneration.CSharp;
using NSwag.CodeGeneration.OperationNameGenerators;

namespace HttpGenerator.Core;

public static class HttpFileGenerator
{
    public static async Task<GeneratorResult> Generate(GeneratorSettings settings)
    {
        var document = await OpenApiDocumentFactory.CreateAsync(settings.OpenApiPath);
        var generator = new CSharpClientGenerator(document, new CSharpClientGeneratorSettings());
        generator.BaseSettings.OperationNameGenerator = new OperationNameGenerator();

        var baseUrl = settings.BaseUrl + document.Servers?.FirstOrDefault()?.Url;
        if (!Uri.IsWellFormedUriString(baseUrl, UriKind.Absolute) &&
            settings.OpenApiPath.StartsWith("http", StringComparison.OrdinalIgnoreCase))
        {
            baseUrl = new Uri(settings.OpenApiPath)
                          .GetLeftPart(UriPartial.Authority) +
                      baseUrl;
        }

        return settings.OutputType == OutputType.OneRequestPerFile
            ? GenerateMultipleFiles(settings, document, baseUrl, generator.BaseSettings.OperationNameGenerator)
            : GenerateSingleFile(settings, document, generator.BaseSettings.OperationNameGenerator, baseUrl);
    }

    private static GeneratorResult GenerateSingleFile(
        GeneratorSettings settings,
        OpenApiDocument document,
        IOperationNameGenerator operationNameGenerator,
        string baseUrl)
    {
        var code = new StringBuilder();
        WriteFileHeaders(settings, code);

        foreach (var kv in document.Paths)
        {
            foreach (var operations in kv.Value)
            {
                code.AppendLine(
                    GenerateRequest(
                        document,
                        kv,
                        operationNameGenerator,
                        settings,
                        baseUrl,
                        operations.Key.CapitalizeFirstCharacter(),
                        operations.Value));
            }
        }

        return new GeneratorResult(
            new[] { new HttpFile("Requests.http", code.ToString()) });
    }

    private static void WriteFileHeaders(GeneratorSettings settings, StringBuilder code)
    {
        code.AppendLine($"@contentType = {settings.ContentType}");

        if (!string.IsNullOrWhiteSpace(settings.AuthorizationHeader) && 
            !settings.AuthorizationHeaderFromEnvironmentVariable)
        {
            code.AppendLine($"@{settings.AuthorizationHeaderVariableName} = {settings.AuthorizationHeader}");
        }

        code.AppendLine();
    }

    private static GeneratorResult GenerateMultipleFiles(
        GeneratorSettings settings,
        OpenApiDocument document,
        string baseUrl,
        IOperationNameGenerator operationNameGenerator)
    {
        var files = new List<HttpFile>();
        foreach (var kv in document.Paths)
        {
            foreach (var operations in kv.Value)
            {
                var operation = operations.Value;
                var verb = operations.Key.CapitalizeFirstCharacter();
                var name = operationNameGenerator.GetOperationName(document, kv.Key, verb, operation);
                var filename = $"{name.CapitalizeFirstCharacter()}.http";

                var code = new StringBuilder();
                WriteFileHeaders(settings, code);
                code.AppendLine(
                    GenerateRequest(document, kv, operationNameGenerator, settings, baseUrl, verb, operation));

                files.Add(new HttpFile(filename, code.ToString()));
            }
        }

        return new GeneratorResult(files);
    }

    private static string GenerateRequest(
        OpenApiDocument document,
        KeyValuePair<string, OpenApiPathItem> operationPath,
        IOperationNameGenerator operationNameGenerator,
        GeneratorSettings settings,
        string baseUrl,
        string verb,
        OpenApiOperation operation)
    {
        var code = new StringBuilder();
        AppendSummary(verb, operationPath, operation, code);

        var parameterNameMap = AppendParameters(
            document,
            operationPath,
            settings,
            operation,
            verb,
            operationNameGenerator,
            code);

        var url = operationPath.Key.Replace("{", "{{").Replace("}", "}}");
        foreach (var parameterName in parameterNameMap)
        {
            url = url.Replace($"{{{parameterName.Key}}}", $"{{{parameterName.Value}}}");
        }

        code.AppendLine($"{verb.ToUpperInvariant()} {baseUrl}{url}");
        code.AppendLine("Content-Type: {{contentType}}");

        if (!string.IsNullOrWhiteSpace(settings.AuthorizationHeader) ||
            settings.AuthorizationHeaderFromEnvironmentVariable)
        {
            code.AppendLine($"Authorization: {{{{{settings.AuthorizationHeaderVariableName}}}}}");
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

    private static void AppendSummary(
        string verb,
        KeyValuePair<string, OpenApiPathItem> kv,
        OpenApiOperation operation,
        StringBuilder code)
    {
        const int padding = 2;
        const string summary = "### Summary: ";
        const string description = "### Description: ";

        var request = $"### Request: {verb.ToUpperInvariant()} {kv.Key}";
        var length = request.Length + padding;
        length = Math.Max(
            length,
            Math.Max(
                (operation.Summary?.Length ?? 0) + summary.Length + padding,
                (operation.Description?.Length ?? 0) + description.Length + padding));

        for (var i = 0; i < length; i++)
        {
            code.Insert(0, "#");
        }

        code.AppendLine();
        code.AppendLine(request);

        if (!string.IsNullOrWhiteSpace(operation.Summary))
        {
            code.AppendLine($"{summary}{operation.Summary}");
        }

        if (!string.IsNullOrWhiteSpace(operation.Description))
        {
            code.AppendLine($"{description}{operation.Description}");
        }

        for (var i = 0; i < length; i++)
        {
            code.Insert(code.Length, "#");
        }

        code.AppendLine(Environment.NewLine);
    }

    private static Dictionary<string, string> AppendParameters(
        OpenApiDocument document,
        KeyValuePair<string, OpenApiPathItem> operationPath,
        GeneratorSettings settings,
        OpenApiOperation operation,
        string verb,
        IOperationNameGenerator operationNameGenerator,
        StringBuilder code)
    {
        var parameters = operation
            .Parameters
            .Where(c => c.Kind is OpenApiParameterKind.Path or OpenApiParameterKind.Query)
            .ToArray();

        var parameterNameMap = new Dictionary<string, string>();
        foreach (var parameter in parameters)
        {
            var parameterName = GetParameterName(
                settings,
                document,
                operation,
                operationNameGenerator,
                verb,
                operationPath.Key,
                parameter);
            parameterNameMap[parameter.Name] = parameterName;

            code.AppendLine(
                $"""
                 ### {parameter.Kind} Parameter: {parameter.Description?.PrefixLineBreaks() ?? parameterName}
                 @{parameterName} = {(parameter.ActualSchema.Type == JsonObjectType.Integer ? 0 : "str")}

                 """);
        }

        code.AppendLine();
        return parameterNameMap;
    }

    private static string GetParameterName(
        GeneratorSettings settings,
        OpenApiDocument document,
        OpenApiOperation operation,
        IOperationNameGenerator operationNameGenerator,
        string verb,
        string operationPathKey,
        OpenApiParameter parameter)
    {
        if (settings.OutputType == OutputType.OneRequestPerFile)
            return parameter.Name;

        var name = operationNameGenerator.GetOperationName(
            document,
            operationPathKey,
            verb,
            operation);

        return $"{name}_{parameter.Name}";
    }
}