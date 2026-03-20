using Atc.Test;
using FluentAssertions;
using HttpGenerator.Tests.Resources;
using HttpGenerator.Validation;
using Microsoft.OpenApi;
using HttpOpenApiValidator = HttpGenerator.Validation.OpenApiValidator;

namespace HttpGenerator.Tests;

public class OpenApiValidatorTests
{
    private const string Https = "https";
    private const string Http = "http";

    private const string HttpsUrlPrefix =
        Https + "://raw.githubusercontent.com/christianhelle/httpgenerator/main/test/OpenAPI/v3.0/";

    private const string HttpUrlPrefix =
        Http + "://raw.githubusercontent.com/christianhelle/httpgenerator/main/test/OpenAPI/v3.0/";
    
    [Theory]
    [InlineData(Samples.PetstoreJsonV3, "SwaggerPetstore.json")]
    [InlineData(Samples.PetstoreYamlV3, "SwaggerPetstore.yaml")]
    [InlineData(Samples.PetstoreJsonV2, "SwaggerPetstore.json")]
    [InlineData(Samples.PetstoreYamlV2, "SwaggerPetstore.yaml")]
    public async Task Should_Return_True_For_Local_Files(Samples sample, string filename)
    {
        var json = EmbeddedResources.GetSwaggerPetstore(sample);
        var swaggerFile = await TestFile.CreateSwaggerFile(json, filename);
        var result = await HttpOpenApiValidator.Validate(swaggerFile);
        result.IsValid.Should().BeTrue();
    }

    [Theory]
    [InlineData(HttpsUrlPrefix + "petstore.json")]
    [InlineData(HttpsUrlPrefix + "petstore.yaml")]
    [InlineData(HttpUrlPrefix + "petstore.json")]
    [InlineData(HttpUrlPrefix + "petstore.yaml")]
    public async Task Should_Return_True_For_Remote_Files(string url)
    {
        var result = await HttpOpenApiValidator.Validate(url);
        result.IsValid.Should().BeTrue();
    }

    [Theory]
    [InlineData(HttpUrlPrefix)]
    public Task Should_Throw_For_Bad_Url(string url)
    {
        return new Func<Task>(()=> HttpOpenApiValidator.Validate(url))
            .Should()
            .ThrowExactlyAsync<InvalidOperationException>();
    }

    [Theory]
    [InlineData("V31.non-oauth-scopes.json")]
    [InlineData("V31.non-oauth-scopes.yaml")]
    [InlineData("V31.webhook-example.json")]
    [InlineData("V31.webhook-example.yaml")]
    public async Task Should_Throw_For_V31_Specs(string manifestResourceStreamName)
    {
        var json = EmbeddedResources.GetStringFromEmbeddedResource(manifestResourceStreamName);
        var swaggerFile = await TestFile.CreateSwaggerFile(json, manifestResourceStreamName);

        await new Func<Task>(() => HttpOpenApiValidator.Validate(swaggerFile))
            .Should()
            .ThrowExactlyAsync<OpenApiUnsupportedSpecVersionException>();
    }

    [Theory, AutoNSubstituteData]
    public async Task Should_Return_Invalid_Result_For_Invalid_OpenApi(string json)
    {
        var swaggerFile = await TestFile.CreateSwaggerFile(json, $"{Guid.NewGuid():N}.json");
        var result = await HttpOpenApiValidator.Validate(swaggerFile);
        result.IsValid.Should().BeFalse();
        result.Diagnostics.Errors.Should().NotBeEmpty();
    }

    [Theory, AutoNSubstituteData]
    public void Should_ThrowIfInvalid(OpenApiValidationResult sut)
    {
        new Action(sut.ThrowIfInvalid)
            .Should()
            .Throw<OpenApiValidationException>();
    }

    [Theory, AutoNSubstituteData]
    public void ThrowIfInvalid_Does_Nothing(OpenApiValidationResult sut)
    {
        sut.Diagnostics.Errors.Clear();
        new Action(sut.ThrowIfInvalid)
            .Should()
            .NotThrow();
    }
}
