using FluentAssertions;
using HttpGenerator.Tests.Resources;
using HttpGenerator.Validation;

namespace HttpGenerator.Tests;

public class OpenApiStatsTests
{
    [Theory]
    [InlineData(Samples.PetstoreJsonV3, "SwaggerPetstore.json")]
    [InlineData(Samples.PetstoreYamlV3, "SwaggerPetstore.yaml")]
    [InlineData(Samples.PetstoreJsonV2, "SwaggerPetstore.json")]
    [InlineData(Samples.PetstoreYamlV2, "SwaggerPetstore.yaml")]
    public async Task Should_Collect_Statistics_From_Petstore(Samples version, string filename)
    {
        var json = EmbeddedResources.GetSwaggerPetstore(version);
        var swaggerFile = await TestFile.CreateSwaggerFile(json, filename);
        var result = await OpenApiValidator.Validate(swaggerFile);
        
        result.Statistics.Should().NotBeNull();
        result.Statistics.PathItemCount.Should().BeGreaterThan(0);
        result.Statistics.OperationCount.Should().BeGreaterThan(0);
        result.Statistics.ParameterCount.Should().BeGreaterThan(0);
        result.Statistics.SchemaCount.Should().BeGreaterThan(0);
    }

    [Theory]
    [InlineData(Samples.PetstoreJsonV3)]
    public async Task Should_Have_Non_Zero_Counts_For_Petstore(Samples version)
    {
        var json = EmbeddedResources.GetSwaggerPetstore(version);
        var swaggerFile = await TestFile.CreateSwaggerFile(json, "petstore.json");
        var result = await OpenApiValidator.Validate(swaggerFile);
        
        var stats = result.Statistics;
        stats.PathItemCount.Should().BeGreaterThan(0, "petstore has paths");
        stats.OperationCount.Should().BeGreaterThan(0, "petstore has operations");
        stats.ParameterCount.Should().BeGreaterThan(0, "petstore has parameters");
        stats.ResponseCount.Should().BeGreaterThan(0, "petstore has responses");
        stats.SchemaCount.Should().BeGreaterThan(0, "petstore has schemas");
    }

    [Fact]
    public async Task ToString_Should_Include_All_Counters()
    {
        var json = EmbeddedResources.GetSwaggerPetstore(Samples.PetstoreJsonV3);
        var swaggerFile = await TestFile.CreateSwaggerFile(json, "petstore.json");
        var result = await OpenApiValidator.Validate(swaggerFile);
        
        var statsString = result.Statistics.ToString();
        
        statsString.Should().Contain("Path Items:");
        statsString.Should().Contain("Operations:");
        statsString.Should().Contain("Parameters:");
        statsString.Should().Contain("Request Bodies:");
        statsString.Should().Contain("Responses:");
        statsString.Should().Contain("Links:");
        statsString.Should().Contain("Callbacks:");
        statsString.Should().Contain("Schemas:");
    }

    [Fact]
    public async Task Statistics_Should_Match_Actual_Document_Elements()
    {
        var json = EmbeddedResources.GetSwaggerPetstore(Samples.PetstoreJsonV3);
        var swaggerFile = await TestFile.CreateSwaggerFile(json, "petstore.json");
        var result = await OpenApiValidator.Validate(swaggerFile);
        
        // Petstore v3 has multiple paths
        result.Statistics.PathItemCount.Should().BeGreaterThan(0, "petstore has paths");
        
        // Petstore has multiple operations across these paths
        result.Statistics.OperationCount.Should().BeGreaterThan(0, "at least one operation");
    }

    [Fact]
    public void Empty_Stats_Should_Have_Zero_Counts()
    {
        var stats = new OpenApiStats();
        
        stats.PathItemCount.Should().Be(0);
        stats.OperationCount.Should().Be(0);
        stats.ParameterCount.Should().Be(0);
        stats.RequestBodyCount.Should().Be(0);
        stats.ResponseCount.Should().Be(0);
        stats.LinkCount.Should().Be(0);
        stats.CallbackCount.Should().Be(0);
        stats.SchemaCount.Should().Be(0);
        stats.HeaderCount.Should().Be(0);
    }

    [Fact]
    public void ToString_Should_Return_Formatted_Output_For_Empty_Stats()
    {
        var stats = new OpenApiStats();
        var output = stats.ToString();
        
        output.Should().NotBeNullOrWhiteSpace();
        output.Should().Contain("Path Items: 0");
        output.Should().Contain("Operations: 0");
    }
}
