using HttpGenerator.Core;

namespace HttpGenerator.Tests;

public class StringExtensionsTests
{
    [Theory]
    [InlineData("kebab-case-string", "KebabCaseString")]
    [InlineData("another-kebab-case-string", "AnotherKebabCaseString")]
    [InlineData("string-with.dot", "StringWith_dot")]
    public void ConvertKebabCaseToPascalCase_ShouldConvert(string input, string expected)
    {
        var result = input.ConvertKebabCaseToPascalCase();
        Assert.Equal(expected, result);
    }

    [Theory]
    [InlineData("/route/to/resource", "routeToResource")]
    [InlineData("/another/route/to/resource", "anotherRouteToResource")]
    public void ConvertRouteToCamelCase_ShouldConvert(string input, string expected)
    {
        var result = input.ConvertRouteToCamelCase();
        Assert.Equal(expected, result);
    }

    [Theory]
    [InlineData("string", "String")]
    [InlineData("anotherString", "AnotherString")]
    [InlineData("a", "A")]
    [InlineData("", "")]
    public void CapitalizeFirstCharacter_ShouldCapitalize(string input, string expected)
    {
        var result = input.CapitalizeFirstCharacter();
        Assert.Equal(expected, result);
    }

    [Theory]
    [InlineData("string with spaces", "StringWithSpaces")]
    [InlineData("another string with spaces", "AnotherStringWithSpaces")]
    public void ConvertSpacesToPascalCase_ShouldConvert(string input, string expected)
    {
        var result = input.ConvertSpacesToPascalCase();
        Assert.Equal(expected, result);
    }

    [Theory]
    [InlineData("string", "prefix", "prefixstring")]
    [InlineData("prefixstring", "prefix", "prefixstring")]
    public void Prefix_ShouldAddPrefix(string input, string prefix, string expected)
    {
        var result = input.Prefix(prefix);
        Assert.Equal(expected, result);
    }

    [Fact]
    public void PrefixLineBreaks_ShouldAddPrefix()
    {
        var isUnix = Environment.OSVersion.Platform == PlatformID.MacOSX || Environment.OSVersion.Platform == PlatformID.Unix;
        var input = isUnix ? "line1\r\nline2\r\nline3" : "line1\nline2\nline3";
        var expected = $"line1{Environment.NewLine}### line2{Environment.NewLine}### line3";
        var result = input.PrefixLineBreaks();
        Assert.Equal(expected, result);
    }
}
