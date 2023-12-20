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
                var credentials = new ChainedTokenCredential(
                    new AzureCliCredential(),
                    new VisualStudioCredential(),
                    new DefaultAzureCredential(
                        new DefaultAzureCredentialOptions
                        {
                            ExcludeWorkloadIdentityCredential = true,
                            ExcludeManagedIdentityCredential = true,
                            ExcludeVisualStudioCredential = true,
                            ExcludeEnvironmentCredential = true,
                            ExcludeAzureCliCredential = true,
                        }));
                
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