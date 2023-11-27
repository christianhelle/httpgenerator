using System.Diagnostics.CodeAnalysis;
using System.Text.RegularExpressions;
using Exceptionless;
using Exceptionless.Dependency;
using Exceptionless.Logging;
using Exceptionless.Models;
using Exceptionless.Models.Data;
using Exceptionless.Plugins;

namespace HttpGenerator;

[ExcludeFromCodeCoverage]
[Priority(50)]
public class RedactedEnvironmentInfoPlugin : IEventPlugin
{
    public void Run(EventPluginContext context)
    {
        if (context.Event.Data.ContainsKey(Event.KnownDataKeys.EnvironmentInfo))
            return;

        try
        {
            var collector = context.Resolver.GetEnvironmentInfoCollector();
            var info = collector.GetEnvironmentInfo();
            RedactCommandLineInfo(info);

            info.IpAddress = "[REDACTED]";
            info.MachineName = null!;

            if (info.Data.ContainsKey("ApplicationBasePath"))
                info.Data.Remove("ApplicationBasePath");

            info.InstallId = context.Client.Configuration.GetInstallId();
            context.Event.Data[Event.KnownDataKeys.EnvironmentInfo] = info;
        }
        catch (Exception ex)
        {
            context.Log.FormattedError(
                typeof(RedactedEnvironmentInfoPlugin),
                ex,
                "Error adding environment information: {0}",
                ex.Message);
        }
    }

    private static void RedactCommandLineInfo(EnvironmentInfo info)
    {
        info.CommandLine = PrivacyHelper.RedactAuthorizationHeaders(info.CommandLine);

        try
        {
            info.CommandLine = Regex.Replace(
                info.CommandLine,
                @"^.*?httpgenerator\.dll",
                "httpgenerator",
                RegexOptions.IgnoreCase,
                TimeSpan.FromSeconds(1));
        }
        catch (Exception e)
        {
            Console.WriteLine(e);
            throw;
        }
    }
}