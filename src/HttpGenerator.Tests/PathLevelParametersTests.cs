using FluentAssertions;
using FluentAssertions.Execution;
using HttpGenerator.Core;
using HttpGenerator.Tests.Resources;

namespace HttpGenerator.Tests;

public class PathLevelParametersTests
{
    [Theory]
    [InlineData(OutputType.OneRequestPerFile)]
    [InlineData(OutputType.OneFile)]
    [InlineData(OutputType.OneFilePerTag)]
    public async Task Generate_Should_Not_Throw_When_PathItem_Has_Parameters(OutputType outputType)
    {
        var act = async () => await GenerateCode(outputType);

        await act.Should().NotThrowAsync();
    }

    [Theory]
    [InlineData(OutputType.OneRequestPerFile)]
    [InlineData(OutputType.OneFile)]
    [InlineData(OutputType.OneFilePerTag)]
    public async Task Generate_Should_Return_Valid_Files_When_PathItem_Has_Parameters(OutputType outputType)
    {
        var result = await GenerateCode(outputType);

        using var scope = new AssertionScope();
        result.Should().NotBeNull();
        result.Files.Should().NotBeNullOrEmpty();
    }

    [Fact]
    public async Task GET_Operation_Should_Include_Query_Parameter_Variable_Definition()
    {
        var result = await GenerateCode(OutputType.OneFile);

        using var scope = new AssertionScope();
        result.Should().NotBeNull();
        result.Files.Should().ContainSingle();
        
        var content = result.Files.First().Content;
        // Operation-level query parameter should get a variable definition
        content.Should().Contain("@GetListIssues_state");
        content.Should().Contain("GET {{baseUrl}}/repos/{{owner}}/{{repo}}/issues");
    }

    [Fact]
    public async Task GET_Operation_Should_Use_PathLevel_Parameters_In_URL()
    {
        var result = await GenerateCode(OutputType.OneFile);

        using var scope = new AssertionScope();
        result.Should().NotBeNull();
        result.Files.Should().ContainSingle();
        
        var content = result.Files.First().Content;
        content.Should().Contain("{{owner}}");
        content.Should().Contain("{{repo}}");
        content.Should().Match("*/repos/{{owner}}/{{repo}}/issues*");
    }

    [Fact]
    public async Task GET_Operation_Should_Include_Query_Parameter_Definition()
    {
        var result = await GenerateCode(OutputType.OneFile);

        using var scope = new AssertionScope();
        result.Should().NotBeNull();
        result.Files.Should().ContainSingle();
        
        var content = result.Files.First().Content;
        // Query parameters may not be in URL if optional, but should have variable definition
        content.Should().Contain("@GetListIssues_state");
    }

    [Fact]
    public async Task POST_Operation_Should_Use_PathLevel_Parameters_In_URL()
    {
        var result = await GenerateCode(OutputType.OneFile);

        using var scope = new AssertionScope();
        result.Should().NotBeNull();
        result.Files.Should().ContainSingle();
        
        var content = result.Files.First().Content;
        
        // The POST operation should use path-level parameters directly in URL
        content.Should().Match("*POST*{{baseUrl}}/repos/{{owner}}/{{repo}}/issues*");
    }

    [Fact]
    public async Task Operation_Parameters_Should_Override_PathLevel_Parameters_With_Same_Name()
    {
        var result = await GenerateCode(OutputType.OneFile);

        using var scope = new AssertionScope();
        result.Should().NotBeNull();
        result.Files.Should().ContainSingle();
        
        var content = result.Files.First().Content;
        
        // The /users/{username} path has username at both path and operation level
        // The operation-level definition should appear (with operation prefix)
        content.Should().Contain("/users/{{GetUser_username}}");
        content.Should().Contain("@GetUser_username");
    }

    [Fact]
    public async Task OneRequestPerFile_Should_Create_Separate_Files_With_PathLevel_Parameters()
    {
        var result = await GenerateCode(OutputType.OneRequestPerFile);

        using var scope = new AssertionScope();
        result.Should().NotBeNull();
        result.Files.Should().HaveCountGreaterOrEqualTo(3, "should have files for GET issues, POST issues, and GET user");
        
        var getIssuesFile = result.Files.FirstOrDefault(f => 
            f.Content.Contains("GET") && f.Content.Contains("/repos/{{owner}}/{{repo}}/issues"));
        getIssuesFile.Should().NotBeNull("GET issues operation should have a file");
        getIssuesFile!.Content.Should().Contain("{{owner}}");
        getIssuesFile.Content.Should().Contain("{{repo}}");
        
        var postIssuesFile = result.Files.FirstOrDefault(f => 
            f.Content.Contains("POST") && f.Content.Contains("/repos/{{owner}}/{{repo}}/issues"));
        postIssuesFile.Should().NotBeNull("POST issues operation should have a file");
        postIssuesFile!.Content.Should().Contain("{{owner}}");
        postIssuesFile.Content.Should().Contain("{{repo}}");
    }

    [Fact]
    public async Task PathLevel_And_OperationLevel_Parameters_Should_Both_Be_Included()
    {
        var result = await GenerateCode(OutputType.OneFile);

        using var scope = new AssertionScope();
        result.Should().NotBeNull();
        result.Files.Should().ContainSingle();
        
        var content = result.Files.First().Content;
        
        // The GET /repos/{owner}/{repo}/issues has:
        // - owner, repo from path level (used directly in URL)
        // - state from operation level (gets variable definition)
        content.Should().Contain("{{owner}}");
        content.Should().Contain("{{repo}}");
        content.Should().Contain("@GetListIssues_state");
    }

    private static async Task<GeneratorResult> GenerateCode(OutputType outputType)
    {
        var json = EmbeddedResources.GetSwaggerPetstore(Samples.PathLevelParametersJsonV3);
        var swaggerFile = await TestFile.CreateSwaggerFile(json, "PathLevelParameters.json");
        return await HttpFileGenerator.Generate(
            new GeneratorSettings
            {
                OpenApiPath = swaggerFile,
                OutputType = outputType,
                GenerateIntelliJTests = true,
            });
    }
}
