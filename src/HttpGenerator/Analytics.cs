using System.Diagnostics.CodeAnalysis;
using System.Reflection;
using Exceptionless;
using Exceptionless.Plugins;
using Spectre.Console.Cli;
using HttpGenerator.Core;
using Exceptionless.Plugins.Default;
using Microsoft.ApplicationInsights;
using Microsoft.ApplicationInsights.Extensibility;

namespace HttpGenerator;

[ExcludeFromCodeCoverage]
public static class Analytics
{
    private static TelemetryClient telemetryClient = null!;

    public static void Configure(Settings settings)
    {
        if (!settings.NoLogging)
            return;

        ExceptionlessClient.Default.Configuration.SetUserIdentity(
            SupportInformation.GetAnonymousIdentity(),
            SupportInformation.GetSupportKey());

        ExceptionlessClient.Default.Configuration.UseSessions();
        ExceptionlessClient.Default.Configuration.RemovePlugin<EnvironmentInfoPlugin>();
        ExceptionlessClient.Default.Configuration.AddPlugin<RedactedEnvironmentInfoPlugin>();
        ExceptionlessClient.Default.Configuration.SetVersion(typeof(GenerateCommand).Assembly.GetName().Version!);
        ExceptionlessClient.Default.Startup("7VSRHLYiJdF7Xp0WaVwmEbJxVmrjqHnTIZNKkrkI");

        var configuration = TelemetryConfiguration.CreateDefault();
        configuration.ConnectionString = "InstrumentationKey=02a24a7d-348f-4dcc-94c4-461d4d90cd74;IngestionEndpoint=https://westeurope-5.in.applicationinsights.azure.com/;LiveEndpoint=https://westeurope.livediagnostics.monitor.azure.com/;ApplicationId=55a72138-8040-4bf1-8dfb-9929062a1a77";

        telemetryClient = new TelemetryClient(configuration);
        telemetryClient.Context.User.Id = SupportInformation.GetSupportKey();
        telemetryClient.Context.Session.Id = Guid.NewGuid().ToString();
        telemetryClient.Context.Operation.Id = Guid.NewGuid().ToString();
        telemetryClient.Context.Device.OperatingSystem = Environment.OSVersion.ToString();
        telemetryClient.Context.Component.Version = typeof(Analytics).Assembly.GetName().Version!.ToString();
        telemetryClient.TelemetryConfiguration.TelemetryInitializers.Add(new SupportKeyInitializer());
    }

    public static Task LogFeatureUsage(Settings settings)
    {
        if (settings.NoLogging)
            return Task.CompletedTask;

        foreach (var property in typeof(Settings).GetProperties())
        {
            if (!CanLogFeature(settings, property))
            {
                continue;
            }

            property.GetCustomAttributes(typeof(CommandOptionAttribute), true)
                .OfType<CommandOptionAttribute>()
                .Where(
                    attribute =>
                        !attribute.LongNames.Contains("output") &&
                        !attribute.LongNames.Contains("no-logging"))
                .ToList()
                .ForEach(attribute => LogFeatureUsage(attribute, property));
        }

        return ExceptionlessClient.Default.ProcessQueueAsync();
    }

    private static void LogFeatureUsage(CommandOptionAttribute attribute, PropertyInfo property)
    {
        var featureName = attribute.LongNames.FirstOrDefault() ?? property.Name;
        
        ExceptionlessClient
            .Default
            .CreateFeatureUsage(featureName)
            .Submit();

        telemetryClient.TrackEvent(featureName);
        telemetryClient.Flush();
    }

    private static bool CanLogFeature(Settings settings, PropertyInfo property)
    {
        var value = property.GetValue(settings);
        if (value is null or false)
            return false;

        if (property.PropertyType == typeof(string[]) && ((string[])value).Length == 0)
            return false;

        return true;
    }

    public static async Task LogError(Exception exception, Settings settings)
    {
        if (settings.NoLogging)
            return;
        
        string json = Serializer.Serialize(settings);
        var properties = Serializer.Deserialize<Dictionary<string, object>>(json)!;
        
        exception
            .ToExceptionless(new ContextData(properties))
            .Submit();

        await ExceptionlessClient.Default.ProcessQueueAsync();

        telemetryClient.TrackException(
            exception,
            new Dictionary<string, string>
            {
                { "settings", json }
            });
    }
}