using System.Diagnostics.CodeAnalysis;
using Spectre.Console.Cli;

namespace HttpGenerator;

[ExcludeFromCodeCoverage]
internal static class Program
{
    private const string InputFilename = "./openapi.json";

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
                    .AddExample(InputFilename);

                configuration
                    .AddExample(
                        InputFilename,
                        "--output",
                        "./");

                configuration
                    .AddExample(
                        InputFilename,
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
                        InputFilename,
                        "--authorization-header",
                        "Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c"
                    );

                configuration
                    .AddExample(
                        InputFilename,
                        "--azure-scope",
                        "[Some Application ID URI]/.default"
                    );

                configuration
                    .AddExample(
                        InputFilename,
                        "--generate-intellij-tests");

                configuration
                    .AddExample(
                        InputFilename,
                        "--custom-header",
                        "X-Custom-Header: Value",
                        "--custom-header",
                        "X-Another-Header: AnotherValue");

            });

        return app.Run(args);
    }
}
