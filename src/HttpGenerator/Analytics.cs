using System.Diagnostics.CodeAnalysis;
using System.Reflection;
using Exceptionless;
using Exceptionless.Plugins;
using Spectre.Console.Cli;
using HttpGenerator.Core;
using Exceptionless.Plugins.Default;

namespace HttpGenerator;

[ExcludeFromCodeCoverage]
public static class Analytics
{
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
                .ForEach(
                    attribute =>
                        ExceptionlessClient.Default
                            .CreateFeatureUsage(attribute.LongNames.FirstOrDefault() ?? property.Name)
                            .Submit());
        }

        return ExceptionlessClient.Default.ProcessQueueAsync();
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

    public static Task LogError(Exception exception, Settings settings)
    {
        if (settings.NoLogging)
            return Task.CompletedTask;

        exception
            .ToExceptionless(
                new ContextData(
                    Serializer.Deserialize<Dictionary<string, object>>(
                        Serializer.Serialize(settings))!))
            .Submit();

        return ExceptionlessClient.Default.ProcessQueueAsync();
    }
}