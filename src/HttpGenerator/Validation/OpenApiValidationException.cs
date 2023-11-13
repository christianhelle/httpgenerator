using System.Runtime.Serialization;

namespace HttpGenerator.Validation;

[Serializable]
public class OpenApiValidationException : Exception
{
    public OpenApiValidationResult ValidationResult { get; } = null!;

    public OpenApiValidationException(
        OpenApiValidationResult validationResult) 
        : base("OpenAPI validation failed")
    {
        ValidationResult = validationResult;
    }
}