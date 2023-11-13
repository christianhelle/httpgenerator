using System.Diagnostics.CodeAnalysis;
using Spectre.Console.Cli;

namespace HttpGenerator;

[ExcludeFromCodeCoverage]
internal static class Program
{
    private static int Main(string[] args)
    {
        if (args.Length == 0)
        {
            args = new[]
            {
                "--help"
            };
        }

        var app = new CommandApp<GenerateCommand>();
        app.Configure(
            configuration =>
            {
                configuration
                    .SetApplicationName("httpgenerator")
                    .SetApplicationVersion(typeof(GenerateCommand).Assembly.GetName().Version!.ToString());

                configuration
                    .AddExample("./openapi.json");

                configuration
                    .AddExample("https://petstore.swagger.io/v2/swagger.json");

                configuration
                    .AddExample(
                        "./openapi.json",
                        "--output",
                        "./");
            });

        return app.Run(args);
    }
}