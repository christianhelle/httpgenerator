using FluentAssertions;
using FluentAssertions.Execution;
using HttpGenerator.Core;

namespace HttpGenerator.Tests;

public class ExampleTests
{
    [Theory]
    [InlineData("https://demo.netbox.dev/api/schema", OutputType.OneFile)]
    [InlineData("https://demo.netbox.dev/api/schema", OutputType.OneFilePerTag)]
    [InlineData("https://demo.netbox.dev/api/schema", OutputType.OneRequestPerFile)]
    public async Task Should_Return_Valid_Code(string url, OutputType outputType)
    {
        var generateCode = await HttpFileGenerator.Generate(
            new()
            {
                OpenApiPath = url,
                OutputType = outputType,
                GenerateIntelliJTests = true,
            });

        using var scope = new AssertionScope();
        generateCode.Should().NotBeNull();
        generateCode.Files.Should().NotBeNullOrEmpty();
    }
}
