using FluentAssertions;
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
    [InlineData(SampleOpenSpecifications.SwaggerPetstoreJsonV3, "SwaggerPetstore.json", OutputType.OneRequestPerFile)]
    [InlineData(SampleOpenSpecifications.SwaggerPetstoreYamlV3, "SwaggerPetstore.yaml", OutputType.OneRequestPerFile)]
    [InlineData(SampleOpenSpecifications.SwaggerPetstoreJsonV2, "SwaggerPetstore.json", OutputType.OneRequestPerFile)]
    [InlineData(SampleOpenSpecifications.SwaggerPetstoreYamlV2, "SwaggerPetstore.yaml", OutputType.OneRequestPerFile)]
    [InlineData(SampleOpenSpecifications.SwaggerPetstoreJsonV3, "SwaggerPetstore.json", OutputType.OneFile)]
    [InlineData(SampleOpenSpecifications.SwaggerPetstoreYamlV3, "SwaggerPetstore.yaml", OutputType.OneFile)]
    [InlineData(SampleOpenSpecifications.SwaggerPetstoreJsonV2, "SwaggerPetstore.json", OutputType.OneFile)]
    [InlineData(SampleOpenSpecifications.SwaggerPetstoreYamlV2, "SwaggerPetstore.yaml", OutputType.OneFile)]
    public async Task Can_Generate_Code(SampleOpenSpecifications version, string filename, OutputType outputType)
    {
        var generateCode = await GenerateCode(version, filename, outputType);
        generateCode.Should().NotBeNull();
        generateCode.Files.Should().NotBeNullOrEmpty();
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
    public async Task Can_Generate_Code_From_Url(string url, OutputType outputType)
    {
        var generateCode = await HttpFileGenerator.Generate(
            new GeneratorSettings
            {
                OpenApiPath = url,
                OutputType = outputType,
                AuthorizationHeader = "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9"
            });

        generateCode.Should().NotBeNull();
        generateCode.Files.Should().NotBeNullOrEmpty();
    }

    private static async Task<GeneratorResult> GenerateCode(
        SampleOpenSpecifications version,
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
                AuthorizationHeader = "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9"
            });
    }
}