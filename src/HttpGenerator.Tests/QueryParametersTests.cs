using FluentAssertions;
using FluentAssertions.Execution;
using HttpGenerator.Core;
using HttpGenerator.Tests.Resources;

namespace HttpGenerator.Tests;

public class QueryParametersTests
{
    [Fact]
    public async Task Generate_Should_Include_Query_Parameters_For_Query_Only_Operation_OneRequestPerFile()
    {
        // In OneRequestPerFile mode, variable names are unqualified (just the param name)
        var result = await GenerateCode(OutputType.OneRequestPerFile);

        using var scope = new AssertionScope();
        result.Should().NotBeNull();
        result.Files.Should().NotBeNullOrEmpty();

        var searchContent = GetContentContaining(result, "/search");
        searchContent.Should().Contain("GET {{baseUrl}}/search?q={{q}}&page={{page}}");
    }

    [Theory]
    [InlineData(OutputType.OneFile)]
    [InlineData(OutputType.OneFilePerTag)]
    public async Task Generate_Should_Include_Query_Parameters_For_Query_Only_Operation_MultiFile(OutputType outputType)
    {
        // In OneFile/OneFilePerTag modes, variable names are operation-qualified to avoid collisions
        var result = await GenerateCode(outputType);

        using var scope = new AssertionScope();
        result.Should().NotBeNull();
        result.Files.Should().NotBeNullOrEmpty();

        var searchContent = GetContentContaining(result, "/search");
        searchContent.Should().Contain("GET {{baseUrl}}/search?q={{GetSearch_q}}&page={{GetSearch_page}}");
    }

    [Fact]
    public async Task Generate_Should_Include_Path_Parameters_For_Path_Only_Operation_OneRequestPerFile()
    {
        // In OneRequestPerFile mode, variable names are unqualified (just the param name)
        var result = await GenerateCode(OutputType.OneRequestPerFile);

        using var scope = new AssertionScope();
        result.Should().NotBeNull();
        result.Files.Should().NotBeNullOrEmpty();

        var userContent = GetContentContaining(result, "/users");
        userContent.Should().Contain("GET {{baseUrl}}/users/{{userId}}");
        userContent.Should().NotContain("?");
    }

    [Theory]
    [InlineData(OutputType.OneFile)]
    [InlineData(OutputType.OneFilePerTag)]
    public async Task Generate_Should_Include_Path_Parameters_For_Path_Only_Operation_MultiFile(OutputType outputType)
    {
        // In OneFile/OneFilePerTag modes, variable names are operation-qualified to avoid collisions
        var result = await GenerateCode(outputType);

        using var scope = new AssertionScope();
        result.Should().NotBeNull();
        result.Files.Should().NotBeNullOrEmpty();

        var userContent = GetContentContaining(result, "/users");
        // Verify the /users path uses the correct operation-qualified variable
        userContent.Should().Contain("GET {{baseUrl}}/users/{{GetUserById_userId}}");
        // Note: NotContain("?") is intentionally omitted for multi-file modes because the fixture has no
        // tags, so OneFilePerTag produces a single "Default.http" containing ALL operations (including
        // /repos which has query params). Only OneRequestPerFile guarantees isolated per-operation files.
    }

    [Fact]
    public async Task Generate_Should_Include_Both_Path_And_Query_Parameters_OneRequestPerFile()
    {
        // In OneRequestPerFile mode, variable names are unqualified (just the param name)
        var result = await GenerateCode(OutputType.OneRequestPerFile);

        using var scope = new AssertionScope();
        result.Should().NotBeNull();
        result.Files.Should().NotBeNullOrEmpty();

        var repoContent = GetContentContaining(result, "/repos");
        repoContent.Should().Contain("GET {{baseUrl}}/repos/{{owner}}/{{repo}}/issues?state={{state}}&per_page={{per_page}}");
    }

    [Theory]
    [InlineData(OutputType.OneFile)]
    [InlineData(OutputType.OneFilePerTag)]
    public async Task Generate_Should_Include_Both_Path_And_Query_Parameters_MultiFile(OutputType outputType)
    {
        // In OneFile/OneFilePerTag modes, variable names are operation-qualified to avoid collisions
        var result = await GenerateCode(outputType);

        using var scope = new AssertionScope();
        result.Should().NotBeNull();
        result.Files.Should().NotBeNullOrEmpty();

        var repoContent = GetContentContaining(result, "/repos");
        repoContent.Should().Contain("GET {{baseUrl}}/repos/{{GetRepoIssues_owner}}/{{GetRepoIssues_repo}}/issues?state={{GetRepoIssues_state}}&per_page={{GetRepoIssues_per_page}}");
    }

    [Theory]
    [InlineData(OutputType.OneRequestPerFile)]
    [InlineData(OutputType.OneFile)]
    [InlineData(OutputType.OneFilePerTag)]
    public async Task Generate_Should_Not_Add_Query_String_For_No_Parameters_Operation(OutputType outputType)
    {
        var result = await GenerateCode(outputType);

        using var scope = new AssertionScope();
        result.Should().NotBeNull();
        result.Files.Should().NotBeNullOrEmpty();
        
        var healthContent = GetContentContaining(result, "/health");
        healthContent.Should().Contain("GET {{baseUrl}}/health");
        healthContent.Should().NotContain("GET {{baseUrl}}/health?");
    }

    [Fact]
    public async Task Generate_Should_Define_Variables_For_Query_Parameters()
    {
        var result = await GenerateCode(OutputType.OneFile);

        using var scope = new AssertionScope();
        result.Should().NotBeNull();
        result.Files.Should().ContainSingle();
        
        var content = result.Files.First().Content;
        content.Should().Contain("@GetSearch_q = ");
        content.Should().Contain("@GetSearch_page = ");
    }

    [Fact]
    public async Task Generate_Should_Define_Variables_For_Path_Parameters()
    {
        var result = await GenerateCode(OutputType.OneFile);

        using var scope = new AssertionScope();
        result.Should().NotBeNull();
        result.Files.Should().ContainSingle();
        
        var content = result.Files.First().Content;
        content.Should().Contain("@GetUserById_userId = ");
    }

    [Fact]
    public async Task Generate_Should_Define_Variables_For_Both_Path_And_Query_Parameters()
    {
        var result = await GenerateCode(OutputType.OneFile);

        using var scope = new AssertionScope();
        result.Should().NotBeNull();
        result.Files.Should().ContainSingle();
        
        var content = result.Files.First().Content;
        content.Should().Contain("@GetRepoIssues_owner = ");
        content.Should().Contain("@GetRepoIssues_repo = ");
        content.Should().Contain("@GetRepoIssues_state = ");
        content.Should().Contain("@GetRepoIssues_per_page = ");
    }

    [Fact]
    public async Task Generate_OneRequestPerFile_Should_Create_Separate_Files_For_Each_Operation()
    {
        var result = await GenerateCode(OutputType.OneRequestPerFile);

        using var scope = new AssertionScope();
        result.Should().NotBeNull();
        result.Files.Should().HaveCount(4);
        
        result.Files.Should().Contain(f => f.Content.Contains("/search"));
        result.Files.Should().Contain(f => f.Content.Contains("/users"));
        result.Files.Should().Contain(f => f.Content.Contains("/repos"));
        result.Files.Should().Contain(f => f.Content.Contains("/health"));
    }

    [Fact]
    public async Task Generate_OneFile_Should_Include_All_Operations_In_Single_File()
    {
        var result = await GenerateCode(OutputType.OneFile);

        using var scope = new AssertionScope();
        result.Should().NotBeNull();
        result.Files.Should().ContainSingle();
        
        var content = result.Files.First().Content;
        content.Should().Contain("/search");
        content.Should().Contain("/users");
        content.Should().Contain("/repos");
        content.Should().Contain("/health");
    }

    [Theory]
    [InlineData(OutputType.OneRequestPerFile)]
    [InlineData(OutputType.OneFile)]
    [InlineData(OutputType.OneFilePerTag)]
    public async Task Generate_Should_Not_Throw_For_Any_Parameter_Combination(OutputType outputType)
    {
        var act = async () => await GenerateCode(outputType);

        await act.Should().NotThrowAsync();
    }

    private static string GetContentContaining(GeneratorResult result, string text)
    {
        var file = result.Files.FirstOrDefault(f => f.Content.Contains(text));
        file.Should().NotBeNull($"expected to find file containing '{text}'");
        return file!.Content;
    }

    private static async Task<GeneratorResult> GenerateCode(OutputType outputType)
    {
        var json = EmbeddedResources.GetSwaggerPetstore(Samples.QueryParametersJsonV3);
        var swaggerFile = await TestFile.CreateSwaggerFile(json, "QueryParameters.json");
        return await HttpFileGenerator.Generate(
            new GeneratorSettings
            {
                OpenApiPath = swaggerFile,
                OutputType = outputType,
                GenerateIntelliJTests = true,
            });
    }
}
