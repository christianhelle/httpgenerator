using System.Net;
using System.Security;
using Microsoft.OpenApi;
using Microsoft.OpenApi.Reader;

namespace HttpGenerator.Validation;

public static class OpenApiValidator
{
    public static async Task<OpenApiValidationResult> Validate(string openApiPath)
    {
        var result = await OpenApiMultiFileReader.Read(openApiPath);
        var statsVisitor = new OpenApiStats();
        var walker = new OpenApiWalker(statsVisitor);
        walker.Walk(result.OpenApiDocument);

        return new(
            result.OpenApiDiagnostic,
            statsVisitor);
    }
}