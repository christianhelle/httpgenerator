using System.Net;
using System.Security;
using System.Text;
using Microsoft.OpenApi;
using Microsoft.OpenApi.Reader;

namespace HttpGenerator.Validation;

public static class OpenApiValidator
{
    public static async Task<OpenApiValidationResult> Validate(string openApiPath)
    {
        var result = await ParseOpenApi(openApiPath);
        var diagnostics = result.Diagnostic ?? new OpenApiDiagnostic();

        var statsVisitor = new OpenApiStats();
        if (result.Document is not null)
        {
            var walker = new OpenApiWalker(statsVisitor);
            walker.Walk(result.Document);
        }

        return new(
            diagnostics,
            statsVisitor);
    }

    private static async Task<string> GetOpenApiContent(
        string input,
        HttpClient httpClient,
        CancellationToken cancellationToken)
    {
        if (input.StartsWith("http", StringComparison.OrdinalIgnoreCase))
        {
            try
            {
                return await httpClient.GetStringAsync(input, cancellationToken);
            }
            catch (HttpRequestException ex)
            {
                throw new InvalidOperationException($"Could not download the file at {input}", ex);
            }
        }

        try
        {
            return await File.ReadAllTextAsync(input, cancellationToken);
        }
        catch (Exception ex) when (ex is FileNotFoundException ||
                                   ex is PathTooLongException ||
                                   ex is DirectoryNotFoundException ||
                                   ex is IOException ||
                                   ex is UnauthorizedAccessException ||
                                   ex is SecurityException ||
                                   ex is NotSupportedException)
        {
            throw new InvalidOperationException($"Could not open the file at {input}", ex);
        }
    }

    private static async Task<ReadResult> ParseOpenApi(string openApiFile)
    {
        using var httpClient = CreateHttpClient();
        var content = await GetOpenApiContent(openApiFile, httpClient, CancellationToken.None);
        var openApiReaderSettings = new OpenApiReaderSettings
        {
            BaseUrl = GetBaseUrl(openApiFile),
        };

        openApiReaderSettings.AddYamlReader();

        await using var stream = new MemoryStream(Encoding.UTF8.GetBytes(content));
        var result = await OpenApiDocument.LoadAsync(
            stream,
            GetFormat(openApiFile, content),
            openApiReaderSettings,
            CancellationToken.None);

        ThrowIfUnsupported(result.Diagnostic);
        return result;
    }

    private static void ThrowIfUnsupported(OpenApiDiagnostic? diagnostic)
    {
        if (diagnostic is null || diagnostic.SpecificationVersion <= OpenApiSpecVersion.OpenApi3_0)
            return;

        throw new OpenApiUnsupportedSpecVersionException(
            $"OpenAPI specification version '{GetVersionText(diagnostic.SpecificationVersion)}' is not supported.");
    }

    private static string GetVersionText(OpenApiSpecVersion specificationVersion)
    {
        return specificationVersion switch
        {
            OpenApiSpecVersion.OpenApi3_1 => "3.1.0",
            OpenApiSpecVersion.OpenApi3_2 => "3.2.0",
            _ => specificationVersion.ToString(),
        };
    }

    private static HttpClient CreateHttpClient()
    {
        var httpClientHandler = new HttpClientHandler
        {
            SslProtocols = System.Security.Authentication.SslProtocols.Tls12,
            AutomaticDecompression = DecompressionMethods.GZip | DecompressionMethods.Deflate,
            ServerCertificateCustomValidationCallback = HttpClientHandler.DangerousAcceptAnyServerCertificateValidator,
        };

        return new HttpClient(httpClientHandler)
        {
            DefaultRequestVersion = HttpVersion.Version20,
        };
    }

    private static Uri GetBaseUrl(string openApiFile)
    {
        if (openApiFile.StartsWith("http", StringComparison.OrdinalIgnoreCase))
            return new Uri(openApiFile);

        var directoryName = new FileInfo(openApiFile).DirectoryName;
        if (directoryName is null)
            throw new InvalidOperationException($"Could not determine the base path for {openApiFile}");

        return new Uri(directoryName + Path.DirectorySeparatorChar, UriKind.Absolute);
    }

    private static string GetFormat(
        string openApiPath,
        string content)
    {
        if (openApiPath.EndsWith(".json", StringComparison.OrdinalIgnoreCase))
            return "json";

        if (openApiPath.EndsWith(".yaml", StringComparison.OrdinalIgnoreCase) ||
            openApiPath.EndsWith(".yml", StringComparison.OrdinalIgnoreCase))
        {
            return "yaml";
        }

        var firstNonWhitespaceCharacter = content.FirstOrDefault(c => !char.IsWhiteSpace(c));
        return firstNonWhitespaceCharacter is '{' or '['
            ? "json"
            : "yaml";
    }
}
