
using HttpGenerator;
using Microsoft.ApplicationInsights.Channel;
using Microsoft.ApplicationInsights.DataContracts;
using NSubstitute;

namespace HttpGenerator.Tests;

public class SupportKeyInitializerTests
{
    [Fact]
    public void Initialize_ShouldAddSupportKeyToTelemetry()
    {
        // Arrange
        var telemetry = Substitute.For<ITelemetry, ISupportProperties>();
        var properties = new Dictionary<string, string>();
        ((ISupportProperties)telemetry).Properties.Returns(properties);

        var initializer = new SupportKeyInitializer();

        // Act
        initializer.Initialize(telemetry);

        // Assert
        Assert.True(properties.ContainsKey("support-key"));
        Assert.Equal(SupportInformation.GetSupportKey(), properties["support-key"]);
    }
}
