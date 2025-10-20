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
            .All(file => file.Content.Contains("client.assert"))
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
                GenerateIntelliJTests = true,
            });

        using var scope = new AssertionScope();
        generateCode.Should().NotBeNull();
        generateCode.Files.Should().NotBeNullOrEmpty();
        generateCode.Files
            .All(file => file.Content.Count(c => c == '#') >= 6)
            .Should()
            .BeTrue();
        generateCode.Files
            .All(file => file.Content.Contains("client.assert"))
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
            .All(file => file.Content.Contains("client.assert"))
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
            .All(file => file.Content.Contains("client.assert"))
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
            .All(file => file.Content.Contains("client.assert"))
            .Should()
            .BeTrue();
    }

    [Theory]
    [InlineData(Samples.PetstoreJsonV2, "SwaggerPetstore.json", OutputType.OneFile, "x-api-key: 12345", "x-api-key: 54321")]
    [InlineData(Samples.PetstoreYamlV2, "SwaggerPetstore.yaml", OutputType.OneFile, "x-api-key: 12345", "x-api-key: 54321")]
    [InlineData(Samples.PetstoreJsonV3, "SwaggerPetstore.json", OutputType.OneFile, "x-api-key: 12345", "x-api-key: 54321")]
    [InlineData(Samples.PetstoreYamlV3, "SwaggerPetstore.yaml", OutputType.OneFile, "x-api-key: 12345", "x-api-key: 54321")]
    public async Task Can_Generate_Code_With_Custom_Headers(
        Samples version,
        string filename,
        OutputType outputType,
        string customHeaderA,
        string customHeaderB)
    {
        var generatedCode = await GenerateCode(
            version, 
            filename, 
            outputType, 
            customHeaderA, 
            customHeaderB);

        using var scope = new AssertionScope();
        generatedCode.Should().NotBeNull();
        generatedCode.Files.Should().NotBeNullOrEmpty();
        generatedCode.Files.All(file => file.Content.Contains(customHeaderA)).Should().BeTrue();
        generatedCode.Files.All(file => file.Content.Contains(customHeaderB)).Should().BeTrue();
    }

    [Theory]
    [InlineData(Samples.PetstoreJsonV2, "SwaggerPetstore.json", OutputType.OneFile)]
    [InlineData(Samples.PetstoreYamlV2, "SwaggerPetstore.yaml", OutputType.OneFile)]
    public async Task Ignores_Host_When_BaseUrl_Specified(
        Samples version,
        string filename,
        OutputType outputType)
    {
        var json = EmbeddedResources.GetSwaggerPetstore(version);
        var swaggerFile = await TestFile.CreateSwaggerFile(json, filename);
        var generatedCode = await HttpFileGenerator.Generate(
            new GeneratorSettings
            {
                OpenApiPath = swaggerFile,
                OutputType = outputType,
                AuthorizationHeader = "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9",
                BaseUrl = "https://api.example.com",
            });

        using var scope = new AssertionScope();
        generatedCode.Should().NotBeNull();
        generatedCode.Files.Select(c=>c.Content).Should().NotContain(c => c.Contains("https://petstore.swagger.io"));
        generatedCode.Files.Select(c=>c.Content).Should().Contain(c => c.Contains("https://api.example.com"));
    }

    [Theory]
    [InlineData(Samples.PetstoreJsonV2, "SwaggerPetstore.json", OutputType.OneFile)]
    [InlineData(Samples.PetstoreYamlV2, "SwaggerPetstore.yaml", OutputType.OneFile)]
    public async Task Uses_Host_When_BaseUrl_Not_Specified(
        Samples version,
        string filename,
        OutputType outputType)
    {
        var json = EmbeddedResources.GetSwaggerPetstore(version);
        var swaggerFile = await TestFile.CreateSwaggerFile(json, filename);
        var generatedCode = await HttpFileGenerator.Generate(
            new GeneratorSettings
            {
                OpenApiPath = swaggerFile,
                OutputType = outputType,
                AuthorizationHeader = "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9",
            });

        using var scope = new AssertionScope();
        generatedCode.Should().NotBeNull();
        generatedCode.Files.Select(c=>c.Content).Should().Contain(c => c.Contains("https://petstore.swagger.io"));
    }

    [Theory]
    [InlineData("int64-overflow-test.json", OutputType.OneRequestPerFile)]
    [InlineData("int64-overflow-yaml-test.yaml", OutputType.OneRequestPerFile)]
    [InlineData("int64-overflow-test.json", OutputType.OneFile)]
    [InlineData("int64-overflow-yaml-test.yaml", OutputType.OneFile)]
    public async Task Can_Generate_Code_For_Int64_With_Overflow_Values(
        string filename,
        OutputType outputType)
    {
        var testFilePath = Path.Combine(
            Path.GetDirectoryName(typeof(SwaggerPetstoreTests).Assembly.Location)!,
            "..", "..", "..", "..", "..", "test", "OpenAPI", "v3.0", filename);

        var generatedCode = await HttpFileGenerator.Generate(
            new GeneratorSettings
            {
                OpenApiPath = testFilePath,
                OutputType = outputType,
                GenerateIntelliJTests = true,
            });

        using var scope = new AssertionScope();
        generatedCode.Should().NotBeNull();
        generatedCode.Files.Should().NotBeNullOrEmpty();
        generatedCode.Files.Should().HaveCountGreaterThan(0);
        generatedCode.Files
            .All(file => !string.IsNullOrWhiteSpace(file.Content))
            .Should()
            .BeTrue();
    }

    private static async Task<GeneratorResult> GenerateCode(
        Samples version,
        string filename,
        OutputType outputType,
        params string[] customHeaders)
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
                CustomHeaders = customHeaders.Length == 0
                    ? null
                    : customHeaders,
            });
    }
}
