namespace VstsPipelineSync
{
    using System;
    using System.Collections.Generic;
    using System.Threading;
    using System.Threading.Tasks;
    using DevOpsLib;
    using Microsoft.Azure.Services.AppAuthentication;
    using Microsoft.Azure.KeyVault;

    class Program
    {
        static async Task Main(string[] args)
        {
            (HashSet<string> branches, TimeSpan waitPeriodBeforeNextUpdate, string msazurePAT, string iotedgePAT, string dbConnectionString) = GetInputsFromArgs(args);
            Console.WriteLine($"Wait period before next update=[{waitPeriodBeforeNextUpdate}]");

            HashSet<BugWiqlQuery> bugQueries = BugWiqlQueryGenerator.GenerateBugQueries();

            VstsBuildBatchUpdate vstsBuildBatchUpdate = new VstsBuildBatchUpdate(new DevOpsAccessSetting(msazurePAT, iotedgePAT), dbConnectionString, branches, bugQueries);
            await vstsBuildBatchUpdate.RunAsync(waitPeriodBeforeNextUpdate, CancellationToken.None);
        }

        private static (HashSet<string> branches, TimeSpan waitPeriodBeforeNextUpdate, string msazurePAT, string iotedgePAT, string dbConnectionString) GetInputsFromArgs(string[] args)
        {
            if (args.Length != 2)
            {
                Console.WriteLine("*** This service will ingest vsts data and upload to the database used by the iotedge test dashboard. Will also generate bugs from failing builds.");
                Console.WriteLine("Authenticates with the database and vsts using secrets from keyvault.");
                Console.WriteLine("VstsBuildBatchUpdate.exe <branches> <wait-period>");
                Console.WriteLine("Usage:");
                Console.WriteLine(" branches: comma deliminated name of branches");
                Console.WriteLine(" wait-period: time between db updates (e.g. 00:01:00)");
                Environment.Exit(1);
            }

            HashSet<string> branches = new HashSet<string>(args[0].Split(","));
            TimeSpan waitPeriodBeforeNextUpdate = TimeSpan.Parse(args[1]);
            string msazurePAT;
            string iotedgePAT;
            string dbConnectionString;

            msazurePAT = GetSecretFromKeyVault_ManagedIdentity_TokenProvider("TestDashboardVstsPat");
            iotedgePAT = GetSecretFromKeyVault_ManagedIdentity_TokenProvider("iotedgeDevOpsProjectPAT");
            dbConnectionString = GetSecretFromKeyVault_ManagedIdentity_TokenProvider("TestDashboardDbConnectionString");

            return (branches, waitPeriodBeforeNextUpdate, msazurePAT, iotedgePAT, dbConnectionString);
        }

        // Reference from https://zimmergren.net/azure-container-instances-managed-identity-key-vault-dotnet-core/
        private static string GetSecretFromKeyVault_ManagedIdentity_TokenProvider(string secretName)
        {
            Console.WriteLine($"Getting secret from keyvault: {secretName}");

            AzureServiceTokenProvider tokenProvider = new AzureServiceTokenProvider();
            var keyVault = new KeyVaultClient(new KeyVaultClient.AuthenticationCallback(tokenProvider.KeyVaultTokenCallback));
            var secretResult = keyVault.GetSecretAsync("https://edgebuildkv.vault.azure.net/", secretName).Result;

            return secretResult.Value;
        }
    }
}
