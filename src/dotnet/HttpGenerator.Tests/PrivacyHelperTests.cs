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

    [Fact]
    public void RedactText_Should_Return_Empty_For_Empty_Input()
    {
        PrivacyHelper
            .RedactAuthorizationHeaders(string.Empty)
            .Should()
            .Be(string.Empty);
    }

    [Theory]
    [InlineData("--base-url https://api.example.com")]
    [InlineData("--output ./output")]
    [InlineData("some random text")]
    public void RedactText_Should_Pass_Through_Non_Authorization_Text(string input)
    {
        PrivacyHelper
            .RedactAuthorizationHeaders(input)
            .Should()
            .Be(input);
    }

    [Fact]
    public void RedactText_Should_Redact_Multiple_Authorization_Headers()
    {
        var input = "--authorization-header Bearer token1 --authorization-header Basic token2";
        var result = PrivacyHelper.RedactAuthorizationHeaders(input);
        
        result.Should().Contain("[REDACTED]");
        result.Should().NotContain("token1");
        result.Should().NotContain("token2");
    }

    
}
