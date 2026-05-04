using FluentAssertions;
using FluentAssertions.Execution;
using HttpGenerator.Core;
using HttpGenerator.Tests.Resources;

namespace HttpGenerator.Tests;

public class NullParametersTests
{
    [Theory]
    [InlineData(OutputType.OneRequestPerFile)]
    [InlineData(OutputType.OneFile)]
    [InlineData(OutputType.OneFilePerTag)]
    public async Task Generate_Should_Not_Throw_When_Operation_Has_Null_Parameters(OutputType outputType)
    {
        var act = async () => await GenerateCode(outputType);

        await act.Should().NotThrowAsync();
    }

    [Theory]
    [InlineData(OutputType.OneRequestPerFile)]
    [InlineData(OutputType.OneFile)]
    [InlineData(OutputType.OneFilePerTag)]
    public async Task Generate_Should_Return_Valid_Files_When_Operation_Has_Null_Parameters(OutputType outputType)
    {
        var result = await GenerateCode(outputType);

        using var scope = new AssertionScope();
        result.Should().NotBeNull();
        result.Files.Should().NotBeNullOrEmpty();
    }

    [Fact]
    public async Task Generate_Should_Include_Operation_Without_Parameters_In_Output()
    {
        var result = await GenerateCode(OutputType.OneFile);

        using var scope = new AssertionScope();
        result.Should().NotBeNull();
        result.Files.Should().ContainSingle();
        
        var content = result.Files.First().Content;
        content.Should().Contain("GET");
        content.Should().Contain("/no-parameters");
    }

    [Fact]
    public async Task Generate_Should_Include_Operation_With_Valid_Parameters_In_Output()
    {
        var result = await GenerateCode(OutputType.OneFile);

        using var scope = new AssertionScope();
        result.Should().NotBeNull();
        result.Files.Should().ContainSingle();
        
        var content = result.Files.First().Content;
        content.Should().Contain("GET");
        content.Should().Contain("/with-parameters");
        content.Should().Contain("@");
        content.Should().Contain("id");
        content.Should().Contain("status");
    }

    [Fact]
    public async Task Generate_Should_Create_Separate_Files_For_Each_Operation_In_OneRequestPerFile_Mode()
    {
        var result = await GenerateCode(OutputType.OneRequestPerFile);

        using var scope = new AssertionScope();
        result.Should().NotBeNull();
        result.Files.Should().HaveCount(2);
        
        var noParamsFile = result.Files.FirstOrDefault(f => f.Content.Contains("/no-parameters"));
        noParamsFile.Should().NotBeNull();
        
        var withParamsFile = result.Files.FirstOrDefault(f => f.Content.Contains("/with-parameters"));
        withParamsFile.Should().NotBeNull();
    }

    private static async Task<GeneratorResult> GenerateCode(OutputType outputType)
    {
        var json = EmbeddedResources.GetSwaggerPetstore(Samples.NullParametersJsonV3);
        var swaggerFile = await TestFile.CreateSwaggerFile(json, "NullParameters.json");
        return await HttpFileGenerator.Generate(
            new GeneratorSettings
            {
                OpenApiPath = swaggerFile,
                OutputType = outputType,
                GenerateIntelliJTests = true,
            });
    }
}
