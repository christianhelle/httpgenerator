using Microsoft.ApplicationInsights.Channel;
using Microsoft.ApplicationInsights.DataContracts;
using Microsoft.ApplicationInsights.Extensibility;

namespace HttpGenerator;

public sealed class SupportKeyInitializer : ITelemetryInitializer
{
    public void Initialize(ITelemetry telemetry)
    {
        if (telemetry is ISupportProperties supportProperties)
            supportProperties.Properties["support-key"] = SupportInformation.GetSupportKey();
    }
}