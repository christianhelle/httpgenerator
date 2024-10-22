using FluentAssertions;
using FluentAssertions.Execution;
using HttpGenerator.Core;
using HttpGenerator.Tests.Resources;

namespace HttpGenerator.Tests;

public class SwaggerPetstoreTests
{
    private const string Https = "https";
    private const string Http = "http";

    private const string HttpsUrlPrefix =
        Https + "://raw.githubusercontent.com/christianhelle/httpgenerator/main/test/OpenAPI/v3.0/";

    private const string HttpUrlPrefix =
        Http + "://raw.githubusercontent.com/christianhelle/httpgenerator/main/test/OpenAPI/v3.0/";

    [Theory]
    [InlineData(Samples.PetstoreJsonV3, "SwaggerPetstore.json", OutputType.OneRequestPerFile)]
    [InlineData(Samples.PetstoreYamlV3, "SwaggerPetstore.yaml", OutputType.OneRequestPerFile)]
    [InlineData(Samples.PetstoreJsonV2, "SwaggerPetstore.json", OutputType.OneRequestPerFile)]
    [InlineData(Samples.PetstoreYamlV2, "SwaggerPetstore.yaml", OutputType.OneRequestPerFile)]
    [InlineData(Samples.PetstoreJsonV3, "SwaggerPetstore.json", OutputType.OneFile)]
    [InlineData(Samples.PetstoreYamlV3, "SwaggerPetstore.yaml", OutputType.OneFile)]
    [InlineData(Samples.PetstoreJsonV2, "SwaggerPetstore.json", OutputType.OneFile)]
    [InlineData(Samples.PetstoreYamlV2, "SwaggerPetstore.yaml", OutputType.OneFile)]
    [InlineData(Samples.PetstoreJsonV3WithDifferentHeaders, "SwaggerPetstore.json", OutputType.OneRequestPerFile)]
    [InlineData(Samples.PetstoreYamlV3WithDifferentHeaders, "SwaggerPetstore.yaml", OutputType.OneRequestPerFile)]
    [InlineData(Samples.PetstoreJsonV2WithDifferentHeaders, "SwaggerPetstore.json", OutputType.OneRequestPerFile)]
    [InlineData(Samples.PetstoreYamlV2WithDifferentHeaders, "SwaggerPetstore.yaml", OutputType.OneRequestPerFile)]
    [InlineData(Samples.PetstoreJsonV3WithDifferentHeaders, "SwaggerPetstore.json", OutputType.OneFile)]
    [InlineData(Samples.PetstoreYamlV3WithDifferentHeaders, "SwaggerPetstore.yaml", OutputType.OneFile)]
    [InlineData(Samples.PetstoreJsonV2WithDifferentHeaders, "SwaggerPetstore.json", OutputType.OneFile)]
    [InlineData(Samples.PetstoreYamlV2WithDifferentHeaders, "SwaggerPetstore.yaml", OutputType.OneFile)]
    public async Task Can_Generate_Code(Samples version, string filename, OutputType outputType)
    {
        var generateCode = await GenerateCode(version, filename, outputType);

        using var scope = new AssertionScope();
        generateCode.Should().NotBeNull();
        generateCode.Files.Should().NotBeNullOrEmpty();
        generateCode.Files
            .All(file => file.Content.Count(c => c == '#') >= 6)
            .Should()
            .BeTrue();
        generateCode.Files
            .All(file => file.Content.Contains("client.assert(response.status === 200"))
            .Should()
            .BeTrue();
    }

    [Theory]
    [InlineData(HttpsUrlPrefix + "petstore.json", OutputType.OneRequestPerFile)]
    [InlineData(HttpsUrlPrefix + "petstore.yaml", OutputType.OneRequestPerFile)]
    [InlineData(HttpUrlPrefix + "petstore.json", OutputType.OneRequestPerFile)]
    [InlineData(HttpUrlPrefix + "petstore.yaml", OutputType.OneRequestPerFile)]
    [InlineData(HttpsUrlPrefix + "petstore.json", OutputType.OneFile)]
    [InlineData(HttpsUrlPrefix + "petstore.yaml", OutputType.OneFile)]
    [InlineData(HttpUrlPrefix + "petstore.json", OutputType.OneFile)]
    [InlineData(HttpUrlPrefix + "petstore.yaml", OutputType.OneFile)]
    [InlineData(HttpsUrlPrefix + "petstore.json", OutputType.OneFilePerTag)]
    [InlineData(HttpsUrlPrefix + "petstore.yaml", OutputType.OneFilePerTag)]
    [InlineData(HttpUrlPrefix + "petstore.json", OutputType.OneFilePerTag)]
    [InlineData(HttpUrlPrefix + "petstore.yaml", OutputType.OneFilePerTag)]
    public async Task Can_Generate_Code_From_Url(string url, OutputType outputType)
    {
        var generateCode = await HttpFileGenerator.Generate(
            new GeneratorSettings
            {
                OpenApiPath = url,
                OutputType = outputType,
                AuthorizationHeader = "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9",
            });

        using var scope = new AssertionScope();
        generateCode.Should().NotBeNull();
        generateCode.Files.Should().NotBeNullOrEmpty();
        generateCode.Files
            .All(file => file.Content.Count(c => c == '#') >= 6)
            .Should()
            .BeTrue();
        generateCode.Files
            .All(file => file.Content.Contains("client.assert(response.status === 200"))
            .Should()
            .BeTrue();
    }

    [Theory]
    [InlineData(Samples.PetstoreJsonV3WithMultlineDescriptions, "SwaggerPetstore.json", OutputType.OneRequestPerFile)]
    [InlineData(Samples.PetstoreYamlV3WithMultlineDescriptions, "SwaggerPetstore.yaml", OutputType.OneRequestPerFile)]
    [InlineData(Samples.PetstoreJsonV3WithMultlineDescriptions, "SwaggerPetstore.json", OutputType.OneFile)]
    [InlineData(Samples.PetstoreYamlV3WithMultlineDescriptions, "SwaggerPetstore.yaml", OutputType.OneFile)]
    public async Task Can_Generate_Code_With_Multiline_Descriptions(
        Samples version,
        string filename,
        OutputType outputType)
    {
        var generateCode = await GenerateCode(version, filename, outputType);

        using var scope = new AssertionScope();
        generateCode.Should().NotBeNull();
        generateCode.Files.Should().NotBeNullOrEmpty();
        generateCode.Files
            .Any(
                file => file.Content.Contains(
                    $"### Description: {Environment.NewLine}",
                    StringComparison.OrdinalIgnoreCase))
            .Should()
            .BeTrue();
        generateCode.Files
            .All(file => file.Content.Contains("client.assert(response.status === 200"))
            .Should()
            .BeTrue();
    }

    [Theory]
    [InlineData(HttpsUrlPrefix + "petstore.json", OutputType.OneRequestPerFile)]
    [InlineData(HttpsUrlPrefix + "petstore.yaml", OutputType.OneRequestPerFile)]
    [InlineData(HttpUrlPrefix + "petstore.json", OutputType.OneRequestPerFile)]
    [InlineData(HttpUrlPrefix + "petstore.yaml", OutputType.OneRequestPerFile)]
    [InlineData(HttpsUrlPrefix + "petstore.json", OutputType.OneFile)]
    [InlineData(HttpsUrlPrefix + "petstore.yaml", OutputType.OneFile)]
    [InlineData(HttpUrlPrefix + "petstore.json", OutputType.OneFile)]
    [InlineData(HttpUrlPrefix + "petstore.yaml", OutputType.OneFile)]
    [InlineData(HttpsUrlPrefix + "petstore.json", OutputType.OneFilePerTag)]
    [InlineData(HttpsUrlPrefix + "petstore.yaml", OutputType.OneFilePerTag)]
    [InlineData(HttpUrlPrefix + "petstore.json", OutputType.OneFilePerTag)]
    [InlineData(HttpUrlPrefix + "petstore.yaml", OutputType.OneFilePerTag)]
    public async Task Files_Generated_From_Url_Uses_OpenApiPath_Authority_As_For_BaseUrl(
        string url,
        OutputType outputType)
    {
        var generateCode = await HttpFileGenerator.Generate(
            new GeneratorSettings
            {
                OpenApiPath = url,
                OutputType = outputType,
                GenerateIntelliJTests = true,
            });

        generateCode
            .Files
            .All(file => file.Content.Contains(new Uri(url).GetLeftPart(UriPartial.Authority)))
            .Should()
            .BeTrue();
        generateCode.Files
            .All(file => file.Content.Contains("client.assert(response.status === 200"))
            .Should()
            .BeTrue();
    }

    [Theory]
    [InlineData(Samples.PetstoreJsonV2, "SwaggerPetstore.json", OutputType.OneFile)]
    [InlineData(Samples.PetstoreYamlV2, "SwaggerPetstore.yaml", OutputType.OneFile)]
    [InlineData(Samples.PetstoreJsonV3, "SwaggerPetstore.json", OutputType.OneFile)]
    [InlineData(Samples.PetstoreYamlV3, "SwaggerPetstore.yaml", OutputType.OneFile)]
    public async Task Can_Generate_Code_With_Query_String_Parameters(
        Samples version,
        string filename,
        OutputType outputType)
    {
        var generateCode = await GenerateCode(version, filename, outputType);

        using var scope = new AssertionScope();
        generateCode.Should().NotBeNull();
        generateCode.Files.Should().NotBeNullOrEmpty();
        generateCode.Files
            .Any(file => file.Content.Contains("?status={{") && file.Content.Contains("}}"))
            .Should()
            .BeTrue();
        generateCode.Files
            .All(file => file.Content.Contains("client.assert(response.status === 200"))
            .Should()
            .BeTrue();
    }

    private static async Task<GeneratorResult> GenerateCode(
        Samples version,
        string filename,
        OutputType outputType)
    {
        var json = EmbeddedResources.GetSwaggerPetstore(version);
        var swaggerFile = await TestFile.CreateSwaggerFile(json, filename);
        return await HttpFileGenerator.Generate(
            new GeneratorSettings
            {
                OpenApiPath = swaggerFile,
                OutputType = outputType,
                AuthorizationHeader = "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9",
                GenerateIntelliJTests = true,
            });
    }
}
