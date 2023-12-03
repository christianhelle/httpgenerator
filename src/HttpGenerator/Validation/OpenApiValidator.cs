﻿using System.Net;
using System.Security;
using Microsoft.OpenApi.Readers;
using Microsoft.OpenApi.Services;

namespace HttpGenerator.Validation;

public static class OpenApiValidator
{
    public static async Task<OpenApiValidationResult> Validate(string openApiPath)
    {
        var result = await ParseOpenApi(openApiPath);

        var statsVisitor = new OpenApiStats();
        var walker = new OpenApiWalker(statsVisitor);
        walker.Walk(result.OpenApiDocument);

        return new(
            result.OpenApiDiagnostic,
            statsVisitor);
    }

    private static async Task<Stream> GetStream(
        string input,
        CancellationToken cancellationToken)
    {
        if (input.StartsWith("http"))
        {
            try
            {
                var httpClientHandler = new HttpClientHandler()
                {
                    SslProtocols = System.Security.Authentication.SslProtocols.Tls12,
                    AutomaticDecompression = DecompressionMethods.GZip | DecompressionMethods.Deflate,
                    ServerCertificateCustomValidationCallback = HttpClientHandler.DangerousAcceptAnyServerCertificateValidator,
                };
                using var httpClient = new HttpClient(httpClientHandler);
                httpClient.DefaultRequestVersion = HttpVersion.Version20;
                return await httpClient.GetStreamAsync(input, cancellationToken);
            }
            catch (HttpRequestException ex)
            {
                throw new InvalidOperationException($"Could not download the file at {input}", ex);
            }
        }

        try
        {
            var fileInput = new FileInfo(input);
            return fileInput.OpenRead();
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
        var directoryName = new FileInfo(openApiFile).DirectoryName;
        var openApiReaderSettings = new OpenApiReaderSettings
        {
            BaseUrl = openApiFile.StartsWith("http", StringComparison.OrdinalIgnoreCase)
                ? new Uri(openApiFile)
                : new Uri($"file://{directoryName}{Path.DirectorySeparatorChar}")
        };

        await using var stream = await GetStream(openApiFile, CancellationToken.None);
        var reader = new OpenApiStreamReader(openApiReaderSettings);
        return await reader.ReadAsync(stream, CancellationToken.None);
    }
}