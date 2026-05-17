using System;
using System.IO;
using System.Linq;
using System.Threading;
using System.Threading.Tasks;
using HttpGenerator.Core;

namespace HttpGenerator.VSIX;

internal static class HttpGeneratorCli
{
    public static async Task<GenerateResult> ExecuteAsync(
        string openApiPath,
        string outputFolder,
        string? baseUrl,
        string contentType,
        string? authorizationHeader,
        bool generateMultipleFiles,
        IProgress<string>? progress,
        CancellationToken cancellationToken)
    {
        progress?.Report("Generating .http files...");

        var settings = new GeneratorSettings
        {
            OpenApiPath = openApiPath,
            BaseUrl = baseUrl,
            ContentType = contentType,
            AuthorizationHeader = authorizationHeader,
            AuthorizationHeaderFromEnvironmentVariable = false,
            GenerateIntelliJTests = false,
            OutputType = generateMultipleFiles ? OutputType.OneRequestPerFile : OutputType.OneFile,
            Timeout = 120,
        };

        var result = await HttpFileGenerator.Generate(settings).ConfigureAwait(false);

        if (!Directory.Exists(outputFolder))
            Directory.CreateDirectory(outputFolder);

        var writtenFiles = new System.Collections.Generic.List<string>();

        foreach (var file in result.Files ?? Array.Empty<HttpFile>())
        {
            var path = Path.Combine(outputFolder, file.Filename);
            await File.WriteAllTextAsync(path, file.Content, cancellationToken).ConfigureAwait(false);
            writtenFiles.Add(path);
        }

        if (writtenFiles.Count > 0)
        {
            progress?.Report($"Successfully generated {writtenFiles.Count} file(s).");
            return GenerateResult.SuccessResult(writtenFiles.Count, writtenFiles);
        }

        progress?.Report("Generation completed but no files were created.");
        return GenerateResult.UnknownResult();
    }
}
