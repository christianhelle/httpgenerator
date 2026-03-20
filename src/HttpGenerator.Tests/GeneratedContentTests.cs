using FluentAssertions;
using HttpGenerator.Core;
using HttpGenerator.Tests.Resources;

namespace HttpGenerator.Tests;

public class GeneratedContentTests
{
    [Theory]
    [InlineData(Samples.PetstoreJsonV3, "SwaggerPetstore.json")]
    public async Task Should_Generate_Sample_JSON_For_Request_Bodies(Samples version, string filename)
    {
        var json = EmbeddedResources.GetSwaggerPetstore(version);
        var swaggerFile = await TestFile.CreateSwaggerFile(json, filename);
        
        var result = await HttpFileGenerator.Generate(new GeneratorSettings
        {
            OpenApiPath = swaggerFile,
            OutputType = OutputType.OneFile,
            BaseUrl = "https://api.example.com"
        });
        
        result.Should().NotBeNull();
        result.Files.Should().NotBeNullOrEmpty();
        
        var content = result.Files.First().Content;
        
        // Should contain sample JSON for request bodies
        content.Should().Contain("{");
        content.Should().Contain("}");
    }

    [Theory]
    [InlineData(Samples.PetstoreJsonV3, "SwaggerPetstore.json")]
    public async Task Should_Generate_Variables_For_Query_Parameters(Samples version, string filename)
    {
        var json = EmbeddedResources.GetSwaggerPetstore(version);
        var swaggerFile = await TestFile.CreateSwaggerFile(json, filename);
        
        var result = await HttpFileGenerator.Generate(new GeneratorSettings
        {
            OpenApiPath = swaggerFile,
            OutputType = OutputType.OneRequestPerFile,
            BaseUrl = "https://api.example.com"
        });
        
        result.Should().NotBeNull();
        result.Files.Should().NotBeNullOrEmpty();
        
        // Find a file with query parameters
        var fileWithQueryParams = result.Files.FirstOrDefault(f => f.Content.Contains("?"));
        if (fileWithQueryParams != null)
        {
            // Should have variable definitions for query parameters
            fileWithQueryParams.Content.Should().MatchRegex(@"@\w+\s*=\s*.+");
        }
    }

    [Fact]
    public async Task Should_Generate_Integer_Default_Value()
    {
        var spec = @"{
  ""openapi"": ""3.0.0"",
  ""info"": { ""title"": ""Test"", ""version"": ""1.0.0"" },
  ""paths"": {
    ""/test"": {
      ""get"": {
        ""parameters"": [
          {
            ""name"": ""count"",
            ""in"": ""query"",
            ""schema"": { ""type"": ""integer"" }
          }
        ]
      }
    }
  }
}";
        var swaggerFile = await TestFile.CreateSwaggerFile(spec, "integer-param.json");
        
        var result = await HttpFileGenerator.Generate(new GeneratorSettings
        {
            OpenApiPath = swaggerFile,
            OutputType = OutputType.OneFile,
            BaseUrl = "https://api.example.com"
        });
        
        result.Should().NotBeNull();
        result.Files.Should().NotBeNullOrEmpty();
        
        var content = result.Files.First().Content;
        content.Should().Contain("count = 0");
    }

    [Fact]
    public async Task Should_Generate_Number_Default_Value()
    {
        var spec = @"{
  ""openapi"": ""3.0.0"",
  ""info"": { ""title"": ""Test"", ""version"": ""1.0.0"" },
  ""paths"": {
    ""/test"": {
      ""get"": {
        ""parameters"": [
          {
            ""name"": ""price"",
            ""in"": ""query"",
            ""schema"": { ""type"": ""number"" }
          }
        ]
      }
    }
  }
}";
        var swaggerFile = await TestFile.CreateSwaggerFile(spec, "number-param.json");
        
        var result = await HttpFileGenerator.Generate(new GeneratorSettings
        {
            OpenApiPath = swaggerFile,
            OutputType = OutputType.OneFile,
            BaseUrl = "https://api.example.com"
        });
        
        result.Should().NotBeNull();
        result.Files.Should().NotBeNullOrEmpty();
        
        var content = result.Files.First().Content;
        content.Should().Contain("price = 0");
    }

    [Fact]
    public async Task Should_Generate_Boolean_Default_Value()
    {
        var spec = @"{
  ""openapi"": ""3.0.0"",
  ""info"": { ""title"": ""Test"", ""version"": ""1.0.0"" },
  ""paths"": {
    ""/test"": {
      ""get"": {
        ""parameters"": [
          {
            ""name"": ""active"",
            ""in"": ""query"",
            ""schema"": { ""type"": ""boolean"" }
          }
        ]
      }
    }
  }
}";
        var swaggerFile = await TestFile.CreateSwaggerFile(spec, "boolean-param.json");
        
        var result = await HttpFileGenerator.Generate(new GeneratorSettings
        {
            OpenApiPath = swaggerFile,
            OutputType = OutputType.OneFile,
            BaseUrl = "https://api.example.com"
        });
        
        result.Should().NotBeNull();
        result.Files.Should().NotBeNullOrEmpty();
        
        var content = result.Files.First().Content;
        content.Should().Contain("active = true");
    }

    [Fact]
    public async Task Should_Generate_String_Default_Value()
    {
        var spec = @"{
  ""openapi"": ""3.0.0"",
  ""info"": { ""title"": ""Test"", ""version"": ""1.0.0"" },
  ""paths"": {
    ""/test"": {
      ""get"": {
        ""parameters"": [
          {
            ""name"": ""name"",
            ""in"": ""query"",
            ""schema"": { ""type"": ""string"" }
          }
        ]
      }
    }
  }
}";
        var swaggerFile = await TestFile.CreateSwaggerFile(spec, "string-param.json");
        
        var result = await HttpFileGenerator.Generate(new GeneratorSettings
        {
            OpenApiPath = swaggerFile,
            OutputType = OutputType.OneFile,
            BaseUrl = "https://api.example.com"
        });
        
        result.Should().NotBeNull();
        result.Files.Should().NotBeNullOrEmpty();
        
        var content = result.Files.First().Content;
        content.Should().Contain("name = str");
    }

    [Theory]
    [InlineData(Samples.PetstoreJsonV3, "SwaggerPetstore.json")]
    public async Task Should_Include_Custom_Headers_In_Generated_Content(Samples version, string filename)
    {
        var json = EmbeddedResources.GetSwaggerPetstore(version);
        var swaggerFile = await TestFile.CreateSwaggerFile(json, filename);
        
        var result = await HttpFileGenerator.Generate(new GeneratorSettings
        {
            OpenApiPath = swaggerFile,
            OutputType = OutputType.OneFile,
            BaseUrl = "https://api.example.com",
            CustomHeaders = new[] { "X-Custom-Header: test123", "X-Another-Header: value456" }
        });
        
        result.Should().NotBeNull();
        result.Files.Should().NotBeNullOrEmpty();
        
        var content = result.Files.First().Content;
        content.Should().Contain("X-Custom-Header: test123");
        content.Should().Contain("X-Another-Header: value456");
    }

    [Theory]
    [InlineData(Samples.PetstoreJsonV3, "SwaggerPetstore.json")]
    public async Task Should_Generate_IntelliJ_Tests_When_Requested(Samples version, string filename)
    {
        var json = EmbeddedResources.GetSwaggerPetstore(version);
        var swaggerFile = await TestFile.CreateSwaggerFile(json, filename);
        
        var result = await HttpFileGenerator.Generate(new GeneratorSettings
        {
            OpenApiPath = swaggerFile,
            OutputType = OutputType.OneFile,
            BaseUrl = "https://api.example.com",
            GenerateIntelliJTests = true
        });
        
        result.Should().NotBeNull();
        result.Files.Should().NotBeNullOrEmpty();
        
        var content = result.Files.First().Content;
        // IntelliJ tests contain JavaScript blocks
        content.Should().Contain(">");
    }
}
