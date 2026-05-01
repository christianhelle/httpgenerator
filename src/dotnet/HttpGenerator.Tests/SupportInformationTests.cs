using System.Security.Cryptography;
using System.Text;
using FluentAssertions;

namespace HttpGenerator.Tests;

public class SupportInformationTests
{
    [Fact]
    public void GetSupportKey_Should_Return_7_Characters()
    {
        SupportInformation
            .GetSupportKey()
            .Should()
            .NotBeNullOrEmpty()
            .And
            .HaveLength(7);
    }

    [Fact]
    public void GetAnonymousIdentity_Should_Return_Sha256_Hash()
    {
        var identity = SupportInformation.GetAnonymousIdentity();
        var machineName = Environment.MachineName;
        var userName = Environment.UserName;
        var value = $"{userName}@{machineName}";
        var bytes = Encoding.UTF8.GetBytes(value);
        var hash = SHA256.HashData(bytes);
        var expected = Convert.ToBase64String(hash).ToLowerInvariant();

        identity.Should().Be(expected);
    }
}