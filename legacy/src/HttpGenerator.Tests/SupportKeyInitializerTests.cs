using FluentAssertions;
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
        properties.Should().ContainKey("support-key");
        properties["support-key"].Should().Be(SupportInformation.GetSupportKey());
    }

    [Fact]
    public void Initialize_ShouldNotThrow_WhenTelemetryDoesNotImplementISupportProperties()
    {
        // Arrange
        var telemetry = Substitute.For<ITelemetry>();
        var initializer = new SupportKeyInitializer();

        // Act - should not throw even though telemetry doesn't implement ISupportProperties
        var act = () => initializer.Initialize(telemetry);

        // Assert
        act.Should().NotThrow();
    }
}
