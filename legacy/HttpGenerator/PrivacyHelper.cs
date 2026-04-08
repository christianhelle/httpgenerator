using System.Text.RegularExpressions;

namespace HttpGenerator;

public static class PrivacyHelper
{
    public static string RedactAuthorizationHeaders(string input)
    {
        const string replacement = "--authorization-header [REDACTED]";
        var patterns = new[]
        {
            "--authorization-header \"[^ ]+ [^ ]+\"",
            "--authorization-header '[^ ]+ [^ ]+'",
            "--authorization-header [^ ]+ [^ ]+",
            "--authorization-header \"[^ ]+\"",
            "--authorization-header '[^ ]+'",
            "--authorization-header [^ ]+",
        };

        return patterns
            .Aggregate(
                input,
                (current, pattern) =>
                    Replace(current, pattern, replacement));
    }

    private static string Replace(
        string input,
        string pattern,
        string replacement)
    {
        return Regex.Replace(
            input,
            pattern,
            replacement,
            RegexOptions.IgnoreCase,
            TimeSpan.FromSeconds(1));
    }
}