using System.Linq;
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

        var statsVisitor = new OpenApiStats();
        var walker = new OpenApiWalker(statsVisitor);
        walker.Walk(result.Document);

        return new(
            result.Diagnostic ?? new OpenApiDiagnostic(),
            statsVisitor);
    }

    private static OpenApiReaderSettings CreateReaderSettings(string input)
    {
        var settings = new OpenApiReaderSettings
        {
            BaseUrl = GetBaseUrl(input)
        };
        settings.AddYamlReader();
        return settings;
    }

    private static async Task<ReadResult> ParseOpenApi(string openApiFile)
    {
        try
        {
            var content = await GetContent(openApiFile);
            using var stream = new MemoryStream(content);
            var result = await OpenApiDocument.LoadAsync(
                stream,
                GetFormat(openApiFile, content),
                CreateReaderSettings(openApiFile),
                CancellationToken.None);

            if (result.Diagnostic?.SpecificationVersion == OpenApiSpecVersion.OpenApi3_1)
            {
                throw new OpenApiUnsupportedSpecVersionException("3.1.0");
            }

            if (result.Document is null)
            {
                throw CreateDocumentLoadException(openApiFile, result.Diagnostic);
            }

            return result;
        }
        catch (HttpRequestException ex)
        {
            throw new InvalidOperationException($"Could not download the file at {openApiFile}", ex);
        }
        catch (Exception ex) when (ex is FileNotFoundException ||
                                   ex is PathTooLongException ||
                                   ex is DirectoryNotFoundException ||
                                   ex is IOException ||
                                   ex is UnauthorizedAccessException ||
                                   ex is SecurityException ||
                                   ex is NotSupportedException)
        {
            throw new InvalidOperationException($"Could not open the file at {openApiFile}", ex);
        }
    }

    private static async Task<byte[]> GetContent(string input)
    {
        if (input.StartsWith("http", StringComparison.OrdinalIgnoreCase))
        {
            using var httpClient = CreateHttpClient();
            return await httpClient.GetByteArrayAsync(input);
        }

        return File.ReadAllBytes(input);
    }

    private static InvalidOperationException CreateDocumentLoadException(
        string openApiFile,
        OpenApiDiagnostic? diagnostic)
    {
        var messages = diagnostic?.Errors
            .Concat(diagnostic.Warnings ?? Array.Empty<OpenApiError>())
            .Select(error => error.ToString())
            .Where(message => !string.IsNullOrWhiteSpace(message))
            .ToArray() ?? Array.Empty<string>();

        if (messages.Length == 0)
        {
            return new InvalidOperationException($"Could not parse the OpenAPI document at {openApiFile}");
        }

        return new InvalidOperationException(
            $"Could not parse the OpenAPI document at {openApiFile}{Environment.NewLine}{string.Join(Environment.NewLine, messages)}");
    }

    private static Uri? GetBaseUrl(string openApiFile)
    {
        if (openApiFile.StartsWith("http", StringComparison.OrdinalIgnoreCase))
        {
            return new Uri(openApiFile);
        }

        var directoryName = Path.GetDirectoryName(Path.GetFullPath(openApiFile));
        if (string.IsNullOrWhiteSpace(directoryName))
        {
            return null;
        }

        var endsWithDirectorySeparator =
            directoryName.EndsWith(Path.DirectorySeparatorChar.ToString(), StringComparison.Ordinal) ||
            directoryName.EndsWith(Path.AltDirectorySeparatorChar.ToString(), StringComparison.Ordinal);

        var fullDirectoryPath = endsWithDirectorySeparator
            ? directoryName
            : directoryName + Path.DirectorySeparatorChar;

        return new Uri(fullDirectoryPath);
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
            DefaultRequestVersion = HttpVersion.Version20
        };
    }

    private static string GetFormat(string openApiFile, byte[] content)
    {
        var extension = Path.GetExtension(openApiFile);
        if (extension.Equals(".yaml", StringComparison.OrdinalIgnoreCase) ||
            extension.Equals(".yml", StringComparison.OrdinalIgnoreCase))
        {
            return "yaml";
        }

        if (extension.Equals(".json", StringComparison.OrdinalIgnoreCase))
        {
            return "json";
        }

        var text = Encoding.UTF8.GetString(content);
        var firstMeaningfulCharacter = text
            .SkipWhile(character => char.IsWhiteSpace(character) || character == '\uFEFF')
            .FirstOrDefault();

        return firstMeaningfulCharacter is '{' or '['
            ? "json"
            : "yaml";
    }
}
