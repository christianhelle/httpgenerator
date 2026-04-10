using FluentAssertions;
using HttpGenerator.Tests.Resources;
using Spectre.Console.Cli;
using System.Reflection;
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

        (await InvokeExecuteAsync(sut, context, settings, TestContext.Current.CancellationToken))
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
    public async Task Should_Generate_Code_From_File_V31_Spec_When_Validation_Skipped(
        string manifestResourceStreamName,
        GenerateCommand sut,
        CommandContext context,
        Settings settings)
    {
        var json = EmbeddedResources.GetStringFromEmbeddedResource(manifestResourceStreamName);
        settings.OpenApiPath = await TestFile.CreateSwaggerFile(json, manifestResourceStreamName);
        settings.NoLogging = true;
        settings.SkipValidation = true;

        (await InvokeExecuteAsync(sut, context, settings, TestContext.Current.CancellationToken))
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
    public async Task Should_Succeed_Validating_V31_Spec(
        string manifestResourceStreamName,
        GenerateCommand sut,
        CommandContext context,
        Settings settings)
    {
        var json = EmbeddedResources.GetStringFromEmbeddedResource(manifestResourceStreamName);
        settings.OpenApiPath = await TestFile.CreateSwaggerFile(json, manifestResourceStreamName);
        settings.NoLogging = true;
        settings.SkipValidation = false;

        (await InvokeExecuteAsync(sut, context, settings, TestContext.Current.CancellationToken))
            .Should()
            .Be(0);
    }

    [Theory]
    [Inline(HttpsUrlPrefix + "petstore.json")]
    [Inline(HttpsUrlPrefix + "petstore.yaml")]
    [Inline(HttpUrlPrefix + "petstore.json")]
    [Inline(HttpUrlPrefix + "petstore.yaml")]
    [Inline(HttpsUrlPrefix + "petstore.json")]
    [Inline(HttpsUrlPrefix + "petstore.yaml")]
    [Inline(HttpUrlPrefix + "petstore.json")]
    [Inline(HttpUrlPrefix + "petstore.yaml")]
    public async Task Can_Generate_Code_From_Url(
        string url,
        GenerateCommand sut,
        CommandContext context,
        Settings settings)
    {
        settings.OpenApiPath = url;
        settings.NoLogging = true;

        (await InvokeExecuteAsync(sut, context, settings, TestContext.Current.CancellationToken))
            .Should()
            .Be(0);
    }

    [Theory]
    [Inline(HttpsUrlPrefix)]
    public async Task Fails_With_Bad_Url(
        string url,
        GenerateCommand sut,
        CommandContext context,
        Settings settings)
    {
        settings.OpenApiPath = url;
        settings.NoLogging = true;

        (await InvokeExecuteAsync(sut, context, settings, TestContext.Current.CancellationToken))
            .Should()
            .NotBe(0);
    }
    private static async Task<int> InvokeExecuteAsync(
        GenerateCommand command,
        CommandContext context,
        Settings settings,
        CancellationToken cancellationToken)
    {
        var method = typeof(GenerateCommand).GetMethod(
            "ExecuteAsync",
            BindingFlags.Instance | BindingFlags.NonPublic)!;
        return await (Task<int>)method.Invoke(command, new object[] { context, settings, cancellationToken })!;
    }
}