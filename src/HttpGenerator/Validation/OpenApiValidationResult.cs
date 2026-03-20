using Microsoft.OpenApi.Reader;

namespace HttpGenerator.Validation;

public record OpenApiValidationResult(
    OpenApiDiagnostic Diagnostics,
    OpenApiStats Statistics)
{
    public bool IsValid => Diagnostics.Errors.Count == 0;
    
    public void ThrowIfInvalid()
    {
        if (!IsValid)
            throw new OpenApiValidationException(this);
    }
}
