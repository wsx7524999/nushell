# Azure REST API Integration for Nushell

This module provides commands to interact with Azure REST APIs directly from Nushell, allowing you to manage and query Azure resources programmatically.

## 📋 Available Commands

### `azure resource list`
Lists all resources in an Azure subscription.

### `azure vm list`
Lists all virtual machines in an Azure subscription.

## 🔐 Authentication

The Azure commands require authentication via an Azure access token. There are two ways to provide credentials:

### Option 1: Environment Variables (Recommended)

Set the following environment variables:

```bash
export AZURE_ACCESS_TOKEN="your-access-token-here"
export AZURE_SUBSCRIPTION_ID="your-subscription-id-here"
```

In Nushell:
```nushell
$env.AZURE_ACCESS_TOKEN = "your-access-token-here"
$env.AZURE_SUBSCRIPTION_ID = "your-subscription-id-here"
```

### Option 2: Command-line Flags

Pass the credentials directly to each command:

```nushell
azure resource list --token "your-access-token" --subscription "your-subscription-id"
```

## 🔑 Getting an Azure Access Token

To obtain an Azure access token, you can use the Azure CLI:

```bash
az login
az account get-access-token --query accessToken --output tsv
```

Or use Azure PowerShell:

```powershell
Connect-AzAccount
(Get-AzAccessToken).Token
```

You can also obtain tokens programmatically using Azure's authentication libraries or OAuth 2.0 flows.

## 📝 Usage Examples

### List All Resources

Using environment variables:
```nushell
# Set environment variables first
$env.AZURE_ACCESS_TOKEN = (az account get-access-token --query accessToken --output tsv)
$env.AZURE_SUBSCRIPTION_ID = (az account show --query id --output tsv)

# List all resources
azure resource list
```

Using command-line flags:
```nushell
let token = (az account get-access-token --query accessToken --output tsv)
let subscription = (az account show --query id --output tsv)

azure resource list --token $token --subscription $subscription
```

### List Virtual Machines

```nushell
azure vm list
```

### Filter and Format Results

Since the commands return structured data, you can use Nushell's powerful data manipulation commands:

```nushell
# Get only resource names and types
azure resource list | get value | select name type

# Filter VMs by location
azure vm list | get value | where location == "eastus"

# Count resources by type
azure resource list | get value | group-by type | transpose | rename key count
```

## 🔒 Security Best Practices

1. **Never hardcode credentials** in your scripts or commit them to version control
2. **Use environment variables** or secure credential storage (like Azure Key Vault)
3. **Rotate tokens regularly** - Azure access tokens typically expire after 1 hour
4. **Use least-privilege access** - ensure tokens have only the permissions needed
5. **For GitHub Actions**, use GitHub Secrets to store credentials:
   - Add secrets: `AZURE_ACCESS_TOKEN` and `AZURE_SUBSCRIPTION_ID`
   - Reference them in workflows: `${{ secrets.AZURE_ACCESS_TOKEN }}`

## 🛠️ API Versions

Each command supports specifying the Azure API version via the `--api-version` flag:

```nushell
azure resource list --api-version "2021-04-01"
azure vm list --api-version "2023-03-01"
```

Default API versions are provided but you can override them as needed.

## 🔗 Azure REST API Documentation

For more information about the Azure REST APIs:
- [Azure REST API Reference](https://docs.microsoft.com/rest/api/azure/)
- [Azure Resource Manager REST API](https://docs.microsoft.com/rest/api/resources/)
- [Azure Compute REST API](https://docs.microsoft.com/rest/api/compute/)

## ⚠️ Error Handling

The commands will return clear error messages if:
- Authentication fails (invalid or expired token)
- Subscription ID is incorrect
- Network errors occur
- API returns errors

Example error handling in scripts:

```nushell
try {
    azure resource list
} catch {
    print "Failed to list resources. Check your credentials and network connection."
}
```

## 🚀 Extending the Integration

The Azure module is designed to be extensible. To add new Azure commands:

1. Create a new file in `crates/nu-command/src/network/azure/`
2. Implement the command following the existing patterns
3. Export it in `mod.rs`
4. Register it in `default_context.rs`

Contributions are welcome!
