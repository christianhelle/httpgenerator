using FluentAssertions;
using HttpGenerator.Core;
using HttpGenerator.Tests.Resources;

namespace HttpGenerator.Tests;

public class OpenApiDocumentFactoryTests
{
    [Theory]
    [InlineData("https://developers.intellihr.io/docs/v1/swagger.json")] // GZIP encoded
    [InlineData("http://raw.githubusercontent.com/christianhelle/httpgenerator/main/test/OpenAPI/v3.0/petstore.json")]
    public async Task Create_From_Uri_Returns_NotNull(string url)
    {
        (await OpenApiDocumentFactory.CreateAsync(url))
            .Should()
            .NotBeNull();
    }
    
    [Theory]
    [InlineData(Samples.PetstoreJsonV3, "SwaggerPetstore.json")]
    [InlineData(Samples.PetstoreYamlV3, "SwaggerPetstore.yaml")]
    [InlineData(Samples.PetstoreJsonV2, "SwaggerPetstore.json")]
    [InlineData(Samples.PetstoreYamlV2, "SwaggerPetstore.yaml")]
    public async Task Create_From_File_Returns_NotNull(Samples version, string filename)
    {
        var swaggerFile = await TestFile.CreateSwaggerFile(EmbeddedResources.GetSwaggerPetstore(version), filename);
        (await OpenApiDocumentFactory.CreateAsync(swaggerFile))
            .Should()
            .NotBeNull();
    }

    [Fact]
    public async Task Create_With_Int64_Overflow_Should_Not_Throw()
    {
        const string openApiSpec = @"{
  ""openapi"": ""3.0.0"",
  ""info"": {
    ""title"": ""Int64 Test API"",
    ""version"": ""1.0.0""
  },
  ""paths"": {
    ""/test"": {
      ""post"": {
        ""requestBody"": {
          ""content"": {
            ""application/json"": {
              ""schema"": {
                ""type"": ""object"",
                ""properties"": {
                  ""identifier"": {
                    ""type"": ""integer"",
                    ""format"": ""int64"",
                    ""minimum"": -9223372036854775808,
                    ""maximum"": 9223372036854775807
                  }
                }
              }
            }
          }
        },
        ""responses"": {
          ""200"": {
            ""description"": ""Success""
          }
        }
      }
    }
  }
}";
        var testFile = await TestFile.CreateSwaggerFile(openApiSpec, "int64-test.json");
        var document = await OpenApiDocumentFactory.CreateAsync(testFile);

        document.Should().NotBeNull();
        document.Paths.Should().ContainKey("/test");
    }

    [Fact]
    public async Task Create_With_Int64_Below_Int32_MinValue_Should_Not_Throw()
    {
        // Arrange
        const string openApiSpec = @"{
  ""openapi"": ""3.0.0"",
  ""info"": {
    ""title"": ""Int64 Test API"",
    ""version"": ""1.0.0""
  },
  ""paths"": {
    ""/test"": {
      ""post"": {
        ""requestBody"": {
          ""content"": {
            ""application/json"": {
              ""schema"": {
                ""type"": ""object"",
                ""properties"": {
                  ""identifier"": {
                    ""type"": ""integer"",
                    ""format"": ""int64"",
                    ""minimum"": -2147483649
                  }
                }
              }
            }
          }
        },
        ""responses"": {
          ""200"": {
            ""description"": ""Success""
          }
        }
      }
    }
  }
}";
        var testFile = await TestFile.CreateSwaggerFile(openApiSpec, "int64-test2.json");

        // Act
        var document = await OpenApiDocumentFactory.CreateAsync(testFile);

        // Assert
        document.Should().NotBeNull();
        document.Paths.Should().ContainKey("/test");
    }
}