using FluentAssertions;
using HttpGenerator.Tests.Resources;
using HttpGenerator.Validation;

namespace HttpGenerator.Tests;

public class OpenApiValidatorTests
{
    [Theory]
    [InlineData(Samples.PetstoreJsonV3, "SwaggerPetstore.json")]
    [InlineData(Samples.PetstoreYamlV3, "SwaggerPetstore.yaml")]
    [InlineData(Samples.PetstoreJsonV2, "SwaggerPetstore.json")]
    [InlineData(Samples.PetstoreYamlV2, "SwaggerPetstore.yaml")]
    [InlineData(Samples.PetstoreJsonV3, "SwaggerPetstore.json")]
    [InlineData(Samples.PetstoreYamlV3, "SwaggerPetstore.yaml")]
    [InlineData(Samples.PetstoreJsonV2, "SwaggerPetstore.json")]
    [InlineData(Samples.PetstoreYamlV2, "SwaggerPetstore.yaml")]
    public async Task Should_(Samples sample, string filename)
    {
        var json = EmbeddedResources.GetSwaggerPetstore(sample);
        var swaggerFile = await TestFile.CreateSwaggerFile(json, filename);
        var result = await OpenApiValidator.Validate(swaggerFile);
        result.IsValid.Should().BeTrue();
    }
}