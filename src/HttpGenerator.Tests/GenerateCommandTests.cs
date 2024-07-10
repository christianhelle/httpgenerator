using FluentAssertions;
using HttpGenerator.Core;
using HttpGenerator.Tests.Resources;
using Spectre.Console.Cli;
using Inline = Atc.Test.InlineAutoNSubstituteDataAttribute;

namespace HttpGenerator.Tests;

public class GenerateCommandTests
{
    private const string Https = "https";
    private const string Http = "http";

    private const string HttpsUrlPrefix =
        Https + "://raw.githubusercontent.com/christianhelle/httpgenerator/main/test/OpenAPI/v3.0/";

    private const string HttpUrlPrefix =
        Http + "://raw.githubusercontent.com/christianhelle/httpgenerator/main/test/OpenAPI/v3.0/";

    [Theory]
    [Inline(Samples.PetstoreJsonV3, "SwaggerPetstore.json")]
    [Inline(Samples.PetstoreYamlV3, "SwaggerPetstore.yaml")]
    [Inline(Samples.PetstoreJsonV2, "SwaggerPetstore.json")]
    [Inline(Samples.PetstoreYamlV2, "SwaggerPetstore.yaml")]
    [Inline(Samples.PetstoreJsonV3, "SwaggerPetstore.json")]
    [Inline(Samples.PetstoreYamlV3, "SwaggerPetstore.yaml")]
    [Inline(Samples.PetstoreJsonV2, "SwaggerPetstore.json")]
    [Inline(Samples.PetstoreYamlV2, "SwaggerPetstore.yaml")]
    [Inline(Samples.PetstoreJsonV3WithDifferentHeaders, "SwaggerPetstore.json")]
    [Inline(Samples.PetstoreYamlV3WithDifferentHeaders, "SwaggerPetstore.yaml")]
    [Inline(Samples.PetstoreJsonV2WithDifferentHeaders, "SwaggerPetstore.json")]
    [Inline(Samples.PetstoreYamlV2WithDifferentHeaders, "SwaggerPetstore.yaml")]
    [Inline(Samples.PetstoreJsonV3WithDifferentHeaders, "SwaggerPetstore.json")]
    [Inline(Samples.PetstoreYamlV3WithDifferentHeaders, "SwaggerPetstore.yaml")]
    [Inline(Samples.PetstoreJsonV2WithDifferentHeaders, "SwaggerPetstore.json")]
    [Inline(Samples.PetstoreYamlV2WithDifferentHeaders, "SwaggerPetstore.yaml")]
    public async Task Should_Generate_Code_From_File(
        Samples version,
        string filename,
        GenerateCommand sut,
        CommandContext context,
        Settings settings)
    {
        var json = EmbeddedResources.GetSwaggerPetstore(version);
        settings.OpenApiPath = await TestFile.CreateSwaggerFile(json, filename);
        settings.NoLogging = true;

        (await sut.ExecuteAsync(context, settings))
            .Should()
            .Be(0);
    }

    [Theory]
    [Inline("V31.non-oauth-scopes.json", OutputType.OneRequestPerFile)]
    [Inline("V31.non-oauth-scopes.yaml", OutputType.OneRequestPerFile)]
    [Inline("V31.webhook-example.json", OutputType.OneRequestPerFile)]
    [Inline("V31.webhook-example.yaml", OutputType.OneRequestPerFile)]
    [Inline("V31.non-oauth-scopes.json", OutputType.OneFile)]
    [Inline("V31.non-oauth-scopes.yaml", OutputType.OneFile)]
    [Inline("V31.webhook-example.json", OutputType.OneFile)]
    [Inline("V31.webhook-example.yaml", OutputType.OneFile)]
    public async Task Should_Generate_Code_From_File_V31_Spec_When_Validation_Skipped(
        string manifestResourceStreamName,
        OutputType outputType,
        GenerateCommand sut,
        CommandContext context,
        Settings settings)
    {
        var json = EmbeddedResources.GetStringFromEmbeddedResource(manifestResourceStreamName);
        settings.OpenApiPath = await TestFile.CreateSwaggerFile(json, manifestResourceStreamName);
        settings.NoLogging = true;
        settings.SkipValidation = true;

        (await sut.ExecuteAsync(context, settings))
            .Should()
            .Be(0);
    }

    [Theory]
    [Inline("V31.non-oauth-scopes.json")]
    [Inline("V31.non-oauth-scopes.yaml")]
    [Inline("V31.webhook-example.json")]
    [Inline("V31.webhook-example.yaml")]
    [Inline("V31.non-oauth-scopes.json")]
    [Inline("V31.non-oauth-scopes.yaml")]
    [Inline("V31.webhook-example.json")]
    [Inline("V31.webhook-example.yaml")]
    public async Task Should_Fail_Validating_V31_Spec(
        string manifestResourceStreamName,
        GenerateCommand sut,
        CommandContext context,
        Settings settings)
    {
        var json = EmbeddedResources.GetStringFromEmbeddedResource(manifestResourceStreamName);
        settings.OpenApiPath = await TestFile.CreateSwaggerFile(json, manifestResourceStreamName);
        settings.NoLogging = true;
        settings.SkipValidation = false;

        (await sut.ExecuteAsync(context, settings))
            .Should()
            .NotBe(0);
    }

    [Theory]
    [Inline(HttpsUrlPrefix + "petstore.json", OutputType.OneRequestPerFile)]
    [Inline(HttpsUrlPrefix + "petstore.yaml", OutputType.OneRequestPerFile)]
    [Inline(HttpUrlPrefix + "petstore.json", OutputType.OneRequestPerFile)]
    [Inline(HttpUrlPrefix + "petstore.yaml", OutputType.OneRequestPerFile)]
    [Inline(HttpsUrlPrefix + "petstore.json", OutputType.OneFile)]
    [Inline(HttpsUrlPrefix + "petstore.yaml", OutputType.OneFile)]
    [Inline(HttpUrlPrefix + "petstore.json", OutputType.OneFile)]
    [Inline(HttpUrlPrefix + "petstore.yaml", OutputType.OneFile)]
    public async Task Can_Generate_Code_From_Url(
        string url,
        OutputType outputType,
        GenerateCommand sut,
        CommandContext context,
        Settings settings)
    {
        settings.OpenApiPath = url;
        settings.NoLogging = true;

        (await sut.ExecuteAsync(context, settings))
            .Should()
            .Be(0);
    }

    [Theory]
    [Inline(HttpsUrlPrefix)]
    public async Task Fails_With_Bad_Url(
        string url,
        OutputType outputType,
        GenerateCommand sut,
        CommandContext context,
        Settings settings)
    {
        settings.OpenApiPath = url;
        settings.NoLogging = true;

        (await sut.ExecuteAsync(context, settings))
            .Should()
            .NotBe(0);
    }
}