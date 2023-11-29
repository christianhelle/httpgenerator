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
    [Inline(Samples.PetstoreJsonV3, "SwaggerPetstore.json", OutputType.OneRequestPerFile)]
    [Inline(Samples.PetstoreYamlV3, "SwaggerPetstore.yaml", OutputType.OneRequestPerFile)]
    [Inline(Samples.PetstoreJsonV2, "SwaggerPetstore.json", OutputType.OneRequestPerFile)]
    [Inline(Samples.PetstoreYamlV2, "SwaggerPetstore.yaml", OutputType.OneRequestPerFile)]
    [Inline(Samples.PetstoreJsonV3, "SwaggerPetstore.json", OutputType.OneFile)]
    [Inline(Samples.PetstoreYamlV3, "SwaggerPetstore.yaml", OutputType.OneFile)]
    [Inline(Samples.PetstoreJsonV2, "SwaggerPetstore.json", OutputType.OneFile)]
    [Inline(Samples.PetstoreYamlV2, "SwaggerPetstore.yaml", OutputType.OneFile)]
    [Inline(Samples.PetstoreJsonV3WithDifferentHeaders, "SwaggerPetstore.json", OutputType.OneRequestPerFile)]
    [Inline(Samples.PetstoreYamlV3WithDifferentHeaders, "SwaggerPetstore.yaml", OutputType.OneRequestPerFile)]
    [Inline(Samples.PetstoreJsonV2WithDifferentHeaders, "SwaggerPetstore.json", OutputType.OneRequestPerFile)]
    [Inline(Samples.PetstoreYamlV2WithDifferentHeaders, "SwaggerPetstore.yaml", OutputType.OneRequestPerFile)]
    [Inline(Samples.PetstoreJsonV3WithDifferentHeaders, "SwaggerPetstore.json", OutputType.OneFile)]
    [Inline(Samples.PetstoreYamlV3WithDifferentHeaders, "SwaggerPetstore.yaml", OutputType.OneFile)]
    [Inline(Samples.PetstoreJsonV2WithDifferentHeaders, "SwaggerPetstore.json", OutputType.OneFile)]
    [Inline(Samples.PetstoreYamlV2WithDifferentHeaders, "SwaggerPetstore.yaml", OutputType.OneFile)]
    public async Task Should_Generate_Code_From_File(
        Samples version,
        string filename,
        OutputType outputType,
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