using FluentAssertions;
using HttpGenerator.Core.Tests.Resources;

namespace HttpGenerator.Core.Tests;

public class SwaggerPetstoreTests
{
    [Theory]
    [InlineData(SampleOpenSpecifications.SwaggerPetstoreJsonV3, "SwaggerPetstore.json")]
    [InlineData(SampleOpenSpecifications.SwaggerPetstoreYamlV3, "SwaggerPetstore.yaml")]
    [InlineData(SampleOpenSpecifications.SwaggerPetstoreJsonV2, "SwaggerPetstore.json")]
    [InlineData(SampleOpenSpecifications.SwaggerPetstoreYamlV2, "SwaggerPetstore.yaml")]
    public async Task Can_Generate_Code(SampleOpenSpecifications version, string filename)
    {
        var generateCode = await GenerateCode(version, filename);
        generateCode.Should().NotBeNull();
        generateCode.Files.Should().NotBeNullOrEmpty();
    }

    [Theory]
    [InlineData("http://raw.githubusercontent.com/christianhelle/httpgenerator/main/test/OpenAPI/v3.0/petstore.json")]
    [InlineData("http://raw.githubusercontent.com/christianhelle/httpgenerator/main/test/OpenAPI/v3.0/petstore.yaml")]
    [InlineData("https://raw.githubusercontent.com/christianhelle/httpgenerator/main/test/OpenAPI/v3.0/petstore.json")]
    [InlineData("https://raw.githubusercontent.com/christianhelle/httpgenerator/main/test/OpenAPI/v3.0/petstore.yaml")]
    public async Task Can_Build_Generated_Code_From_Url(string url)
    {
        var generateCode = await HttpFileGenerator.Generate(
            new GeneratorSettings
            {
                OpenApiPath = url
            });

        generateCode.Should().NotBeNull();
        generateCode.Files.Should().NotBeNullOrEmpty();
    }

    private static async Task<GeneratorResult> GenerateCode(
        SampleOpenSpecifications version,
        string filename)
    {
        var swaggerFile = await TestFile.CreateSwaggerFile(EmbeddedResources.GetSwaggerPetstore(version), filename);
        return await HttpFileGenerator.Generate(
            new GeneratorSettings
            {
                OpenApiPath = swaggerFile
            });
    }
}