using System.ComponentModel;
using System.ComponentModel.DataAnnotations;
using System.Diagnostics.CodeAnalysis;
using HttpGenerator.Core;
using Spectre.Console.Cli;

namespace HttpGenerator;

[ExcludeFromCodeCoverage]
public class Settings : CommandSettings
{
    [Description("URL or file path to OpenAPI Specification file")]
    [CommandArgument(0, "[URL or input file]")]
    public string OpenApiPath { get; set; } = null!;

    [Description("Output directory")]
    [CommandOption("-o|--output <OUTPUT>")]
    [DefaultValue("./")]
    public string OutputFolder { get; set; } = "./";

    [Description("Don't log errors or collect telemetry")]
    [CommandOption("--no-logging")]
    [DefaultValue(false)]
    public bool NoLogging { get; set; }

    [Description("Skip validation of OpenAPI Specification file")]
    [CommandOption("--skip-validation")]
    [DefaultValue(false)]
    public bool SkipValidation { get; set; }
    
    [Description("Authorization header to use for all requests")]
    [CommandOption("--authorization-header <HEADER>")]
    public string? AuthorizationHeader { get; set; }

    [Description("Load the authorization header from an environment variable or define it in the .http file. " +
                 "You can use --authorization-header-variable-name to specify the environment variable name.")]
    [CommandOption("--load-authorization-header-from-environment")]
    public bool AuthorizationHeaderFromEnvironmentVariable { get; set; }

    [Description("Name of the environment variable to load the authorization header from")]
    [CommandOption("--authorization-header-variable-name <VARIABLE-NAME>")]
    [DefaultValue("authorization")]
    public string AuthorizationHeaderVariableName { get; set; } = "authorization";
    
    [Description("Default Content-Type header to use for all requests")]
    [CommandOption("--content-type <CONTENT-TYPE>")]
    [DefaultValue("application/json")]
    public string ContentType { get; set; } = "application/json";
    
    [Description("Default Base URL to use for all requests. Use this if the OpenAPI spec doesn't explicitly specify a server URL.")]
    [CommandOption("--base-url <BASE-URL>")]
    public string? BaseUrl { get; set; }
    
    [Description(
        $"{nameof(OutputType.OneRequestPerFile)} generates one .http file per request. " +
        $"{nameof(OutputType.OneFile)} generates a single .http file for all requests. " +
        $"{nameof(OutputType.OneFilePerTag)} generates one .http file per first tag associated with each request.")]
    [CommandOption("--output-type <OUTPUT-TYPE>")]
    [DefaultValue(OutputType.OneRequestPerFile)]
    [AllowedValues(nameof(OutputType.OneRequestPerFile), nameof(OutputType.OneFile))]
    public OutputType OutputType { get; set; } = OutputType.OneRequestPerFile;
    
    [Description("Azure Entra ID Scope to use for retrieving Access Token for Authorization header")]
    [CommandOption("--azure-scope <SCOPE>")]
    public string? AzureScope { get; set; }
    
    [Description("Azure Entra ID Tenant ID to use for retrieving Access Token for Authorization header")]
    [CommandOption("--azure-tenant-id <TENANT-ID>")]
    public string? AzureTenantId { get; set; }
    
    [Description("Timeout (in seconds) for writing files to disk")]
    [CommandOption("--timeout <SECONDS>")]
    [DefaultValue(120)]
    public int Timeout { get; set; } = 120;

    [Description("Generate IntelliJ tests that assert whether the response status code is 200")]
    [CommandOption("--generate-intellij-tests")]
    public bool GenerateIntelliJTests { get; set; }
}

