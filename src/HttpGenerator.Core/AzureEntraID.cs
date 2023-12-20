using Azure.Core;
using Azure.Identity;

namespace HttpGenerator.Core
{
    public static class AzureEntraID
    {
        public static async Task<string?> TryGetAccessTokenAsync(
            string tenantId,
            string scope,
            CancellationToken cancellationToken)
        {
            try
            {
                var request = new TokenRequestContext([scope], tenantId: tenantId);
                var credentials = new DefaultAzureCredential(
                    new DefaultAzureCredentialOptions
                    {
                        ExcludeEnvironmentCredential = true,
                        ExcludeWorkloadIdentityCredential = true,
                        ExcludeManagedIdentityCredential = true,
                    });
                var token = await credentials.GetTokenAsync(request, cancellationToken);
                return token.Token;
            }
            catch (OperationCanceledException)
            {
                // Ignore
            }

            return null;
        }
    }
}