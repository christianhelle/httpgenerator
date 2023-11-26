using FluentAssertions;

namespace HttpGenerator.Tests;

public class PrivacyHelperTests
{
    [Theory]
    [InlineData("--authorization-header XxxxXxxxXxxx")]
    [InlineData("--authorization-header \"XxxxXxxxXxxx\"")]
    [InlineData("--authorization-header \'XxxxXxxxXxxx\'")]
    [InlineData("--authorization-header Bearer XxxxXxxxXxxx")]
    [InlineData("--authorization-header Basic XxxxXxxxXxxx")]
    [InlineData("--authorization-header Token XxxxXxxxXxxx")]
    [InlineData("--authorization-header bearer XxxxXxxxXxxx")]
    [InlineData("--authorization-header basic XxxxXxxxXxxx")]
    [InlineData("--authorization-header token XxxxXxxxXxxx")]
    [InlineData("--authorization-header 'Bearer XxxxXxxxXxxx'")]
    [InlineData("--authorization-header 'Basic XxxxXxxxXxxx'")]
    [InlineData("--authorization-header 'Token XxxxXxxxXxxx'")]
    [InlineData("--authorization-header 'bearer XxxxXxxxXxxx'")]
    [InlineData("--authorization-header 'basic XxxxXxxxXxxx'")]
    [InlineData("--authorization-header 'token XxxxXxxxXxxx'")]
    [InlineData("--authorization-header \"Bearer XxxxXxxxXxxx\"")]
    [InlineData("--authorization-header \"Basic XxxxXxxxXxxx\"")]
    [InlineData("--authorization-header \"Token XxxxXxxxXxxx\"")]
    [InlineData("--authorization-header \"bearer XxxxXxxxXxxx\"")]
    [InlineData("--authorization-header \"basic XxxxXxxxXxxx\"")]
    [InlineData("--authorization-header \"token XxxxXxxxXxxx\"")]
    public void RedactText_Should_Redact_Authorization_Header(string input)
    {
        PrivacyHelper
            .RedactAuthorizationHeaders(input)
            .Should()
            .Be("--authorization-header [REDACTED]");
    }
}