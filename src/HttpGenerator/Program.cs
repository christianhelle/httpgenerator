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
                    .AddExample(
                        "./openapi.json",
                        "--output",
                        "./");

                configuration
                    .AddExample(
                        "./openapi.json",
                        "--output-type",
                        "onefile");

                configuration
                    .AddExample("https://petstore.swagger.io/v2/swagger.json");

                configuration
                    .AddExample(
                        "https://petstore3.swagger.io/api/v3/openapi.json",
                        "--base-url",
                        "https://petstore3.swagger.io"
                    );

                configuration
                    .AddExample(
                        "./openapi.json",
                        "--authorization-header",
                        "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c"
                    );

                configuration
                    .AddExample(
                        "./openapi.json",
                        "--azure-scope",
                        "[Some Application ID URI]/.default"
                    );
            });

        return app.Run(args);
    }
}