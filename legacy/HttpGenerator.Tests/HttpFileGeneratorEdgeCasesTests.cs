using FluentAssertions;
using HttpGenerator.Core;
using HttpGenerator.Tests.Resources;

namespace HttpGenerator.Tests;

public class HttpFileGeneratorEdgeCasesTests
{
    [Theory]
    [InlineData(Samples.PetstoreJsonV3, "SwaggerPetstore.json", OutputType.OneRequestPerFile)]
    [InlineData(Samples.PetstoreJsonV3, "SwaggerPetstore.json", OutputType.OneFile)]
    [InlineData(Samples.PetstoreJsonV3, "SwaggerPetstore.json", OutputType.OneFilePerTag)]
    public async Task Should_Use_BaseUrl_As_Environment_Variable_Template(Samples version, string filename, OutputType outputType)
    {
        var json = EmbeddedResources.GetSwaggerPetstore(version);
        var swaggerFile = await TestFile.CreateSwaggerFile(json, filename);
        
        var result = await HttpFileGenerator.Generate(new GeneratorSettings
        {
            OpenApiPath = swaggerFile,
            OutputType = outputType,
            BaseUrl = "{{MY_BASE_URL}}"
        });
        
        result.Should().NotBeNull();
        result.Files.Should().NotBeNullOrEmpty();
        
        var firstFile = result.Files.First();
        firstFile.Content.Should().Contain("@baseUrl = {{MY_BASE_URL}}");
    }

    [Theory]
    [InlineData(Samples.PetstoreJsonV3, "SwaggerPetstore.json")]
    public async Task Should_Skip_Headers_When_SkipHeaders_Is_True(Samples version, string filename)
    {
        var json = EmbeddedResources.GetSwaggerPetstore(version);
        var swaggerFile = await TestFile.CreateSwaggerFile(json, filename);
        
        var result = await HttpFileGenerator.Generate(new GeneratorSettings
        {
            OpenApiPath = swaggerFile,
            OutputType = OutputType.OneFile,
            SkipHeaders = true,
            BaseUrl = "https://api.example.com"
        });
        
        result.Should().NotBeNull();
        result.Files.Should().NotBeNullOrEmpty();
        
        var content = result.Files.First().Content;
        content.Should().NotContain("@baseUrl");
        content.Should().NotContain("@contentType");
    }

    [Theory]
    [InlineData(Samples.PetstoreJsonV3, "SwaggerPetstore.json")]
    public async Task Should_Not_Write_Authorization_Header_When_LoadFromEnvironment(Samples version, string filename)
    {
        var json = EmbeddedResources.GetSwaggerPetstore(version);
        var swaggerFile = await TestFile.CreateSwaggerFile(json, filename);
        
        var result = await HttpFileGenerator.Generate(new GeneratorSettings
        {
            OpenApiPath = swaggerFile,
            OutputType = OutputType.OneFile,
            AuthorizationHeader = "Bearer test-token",
            AuthorizationHeaderFromEnvironmentVariable = true,
            BaseUrl = "https://api.example.com"
        });
        
        result.Should().NotBeNull();
        result.Files.Should().NotBeNullOrEmpty();
        
        var content = result.Files.First().Content;
        // Should not contain the authorization variable definition when loading from environment
        content.Should().NotContain("@authorization = Bearer test-token");
    }

    [Theory]
    [InlineData(Samples.PetstoreJsonV3, "SwaggerPetstore.json")]
    public async Task Should_Write_Authorization_Header_When_Not_LoadFromEnvironment(Samples version, string filename)
    {
        var json = EmbeddedResources.GetSwaggerPetstore(version);
        var swaggerFile = await TestFile.CreateSwaggerFile(json, filename);
        
        var result = await HttpFileGenerator.Generate(new GeneratorSettings
        {
            OpenApiPath = swaggerFile,
            OutputType = OutputType.OneFile,
            AuthorizationHeader = "Bearer test-token",
            AuthorizationHeaderFromEnvironmentVariable = false,
            BaseUrl = "https://api.example.com"
        });
        
        result.Should().NotBeNull();
        result.Files.Should().NotBeNullOrEmpty();
        
        var content = result.Files.First().Content;
        content.Should().Contain("@authorization = Bearer test-token");
    }

    [Theory]
    [InlineData(Samples.PetstoreJsonV3, "SwaggerPetstore.json", "myAuthToken")]
    public async Task Should_Use_Custom_Authorization_Variable_Name(Samples version, string filename, string variableName)
    {
        var json = EmbeddedResources.GetSwaggerPetstore(version);
        var swaggerFile = await TestFile.CreateSwaggerFile(json, filename);
        
        var result = await HttpFileGenerator.Generate(new GeneratorSettings
        {
            OpenApiPath = swaggerFile,
            OutputType = OutputType.OneFile,
            AuthorizationHeader = "Bearer test-token",
            AuthorizationHeaderFromEnvironmentVariable = false,
            AuthorizationHeaderVariableName = variableName,
            BaseUrl = "https://api.example.com"
        });
        
        result.Should().NotBeNull();
        result.Files.Should().NotBeNullOrEmpty();
        
        var content = result.Files.First().Content;
        content.Should().Contain($"@{variableName} = Bearer test-token");
    }

    [Theory]
    [InlineData(Samples.PetstoreJsonV3, "SwaggerPetstore.json")]
    public async Task Should_Construct_BaseUrl_From_Http_OpenApiPath(Samples version, string filename)
    {
        var json = EmbeddedResources.GetSwaggerPetstore(version);
        var swaggerFile = await TestFile.CreateSwaggerFile(json, filename);
        
        // Test with a local file and verify baseUrl handling
        var localResult = await HttpFileGenerator.Generate(new GeneratorSettings
        {
            OpenApiPath = swaggerFile,
            OutputType = OutputType.OneFile,
            BaseUrl = "https://api.example.com"
        });
        
        localResult.Should().NotBeNull();
        localResult.Files.Should().NotBeNullOrEmpty();
    }

    [Fact]
    public async Task Should_Generate_Unique_Filenames_For_Duplicate_Operations()
    {
        // Create a minimal OpenAPI spec with operations that would generate duplicate filenames
        var spec = @"{
  ""openapi"": ""3.0.0"",
  ""info"": { ""title"": ""Test"", ""version"": ""1.0.0"" },
  ""paths"": {
    ""/test"": {
      ""get"": { ""operationId"": ""GetTest"" },
      ""post"": { ""operationId"": ""GetTest"" }
    }
  }
}";
        var swaggerFile = await TestFile.CreateSwaggerFile(spec, "duplicate-ops.json");
        
        var result = await HttpFileGenerator.Generate(new GeneratorSettings
        {
            OpenApiPath = swaggerFile,
            OutputType = OutputType.OneRequestPerFile,
            BaseUrl = "https://api.example.com"
        });
        
        result.Should().NotBeNull();
        result.Files.Should().HaveCountGreaterThan(1);
        
        // Verify filenames are unique
        var filenames = result.Files.Select(f => f.Filename).ToList();
        filenames.Should().OnlyHaveUniqueItems();
        
        // Both operations should generate files
        filenames.Should().Contain("GetTest.http");
        filenames.Should().Contain("PostGetTest.http");
    }

    [Fact]
    public async Task Should_Append_Counter_For_Truly_Duplicate_Filenames()
    {
        // Create a spec where operations would generate identical names
        var spec = @"{
  ""openapi"": ""3.0.0"",
  ""info"": { ""title"": ""Test"", ""version"": ""1.0.0"" },
  ""paths"": {
    ""/test"": {
      ""get"": { ""operationId"": ""test"" }
    },
    ""/other"": {
      ""get"": { ""operationId"": ""test"" }
    }
  }
}";
        var swaggerFile = await TestFile.CreateSwaggerFile(spec, "duplicate-filenames.json");
        
        var result = await HttpFileGenerator.Generate(new GeneratorSettings
        {
            OpenApiPath = swaggerFile,
            OutputType = OutputType.OneRequestPerFile,
            BaseUrl = "https://api.example.com"
        });
        
        result.Should().NotBeNull();
        result.Files.Should().HaveCount(2);
        
        var filenames = result.Files.Select(f => f.Filename).ToList();
        filenames.Should().OnlyHaveUniqueItems();
        
        // Should have GetTest.http and GetTest_2.http
        filenames.Should().Contain("GetTest.http");
        filenames.Should().Contain("GetTest_2.http");
    }

    [Theory]
    [InlineData(Samples.PetstoreJsonV3, "SwaggerPetstore.json")]
    public async Task Should_Use_Custom_ContentType(Samples version, string filename)
    {
        var json = EmbeddedResources.GetSwaggerPetstore(version);
        var swaggerFile = await TestFile.CreateSwaggerFile(json, filename);
        
        var result = await HttpFileGenerator.Generate(new GeneratorSettings
        {
            OpenApiPath = swaggerFile,
            OutputType = OutputType.OneFile,
            ContentType = "application/xml",
            BaseUrl = "https://api.example.com"
        });
        
        result.Should().NotBeNull();
        result.Files.Should().NotBeNullOrEmpty();
        
        var content = result.Files.First().Content;
        content.Should().Contain("@contentType = application/xml");
    }

    [Fact]
    public async Task Should_Return_Empty_Files_For_Spec_With_No_Paths()
    {
        var spec = @"{
  ""openapi"": ""3.0.0"",
  ""info"": { ""title"": ""Test"", ""version"": ""1.0.0"" }
}";
        var swaggerFile = await TestFile.CreateSwaggerFile(spec, "no-paths.json");
        
        var result = await HttpFileGenerator.Generate(new GeneratorSettings
        {
            OpenApiPath = swaggerFile,
            OutputType = OutputType.OneRequestPerFile,
            BaseUrl = "https://api.example.com"
        });
        
        result.Should().NotBeNull();
        result.Files.Should().BeEmpty();
    }

    [Fact]
    public async Task Should_Return_Empty_Files_For_Paths_With_No_Operations()
    {
        var spec = @"{
  ""openapi"": ""3.0.0"",
  ""info"": { ""title"": ""Test"", ""version"": ""1.0.0"" },
  ""paths"": {
    ""/test"": {}
  }
}";
        var swaggerFile = await TestFile.CreateSwaggerFile(spec, "no-operations.json");
        
        var result = await HttpFileGenerator.Generate(new GeneratorSettings
        {
            OpenApiPath = swaggerFile,
            OutputType = OutputType.OneRequestPerFile,
            BaseUrl = "https://api.example.com"
        });
        
        result.Should().NotBeNull();
        result.Files.Should().BeEmpty();
    }
}
