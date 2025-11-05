# Azure REST API Integration

This document describes the Azure REST API integration feature in Nushell, which allows you to interact with Azure services directly from the command line.

## Overview

The Azure integration provides built-in commands to query and manage Azure resources using the Azure Resource Manager REST API. This allows you to:

- List Azure resources in your subscriptions
- Query virtual machines and their configurations
- Filter and analyze Azure resources using Nushell's powerful data manipulation commands
- Integrate Azure operations into your Nushell scripts and workflows

## Commands

### `azure resource list`

Lists all resources in an Azure subscription.

**Usage:**
```nushell
azure resource list [--token <token>] [--subscription <subscription-id>] [--api-version <version>]
```

**Example:**
```nushell
# List all resources
azure resource list

# Filter storage accounts
azure resource list | get value | where type == "Microsoft.Storage/storageAccounts"
```

### `azure vm list`

Lists all virtual machines in an Azure subscription.

**Usage:**
```nushell
azure vm list [--token <token>] [--subscription <subscription-id>] [--api-version <version>]
```

**Example:**
```nushell
# List all VMs
azure vm list

# Show VM sizes
azure vm list | get value | select name properties.hardwareProfile.vmSize
```

## Authentication

The Azure commands require authentication via an Azure access token. You can provide credentials in two ways:

### Option 1: Environment Variables (Recommended)

Set these environment variables before running Azure commands:

```bash
# In Bash/Zsh
export AZURE_ACCESS_TOKEN=$(az account get-access-token --query accessToken --output tsv)
export AZURE_SUBSCRIPTION_ID=$(az account show --query id --output tsv)
```

```nushell
# In Nushell
$env.AZURE_ACCESS_TOKEN = (az account get-access-token --query accessToken --output tsv)
$env.AZURE_SUBSCRIPTION_ID = (az account show --query id --output tsv)
```

### Option 2: Command-Line Flags

Pass credentials directly to each command:

```nushell
let token = (az account get-access-token --query accessToken --output tsv)
let subscription = (az account show --query id --output tsv)

azure resource list --token $token --subscription $subscription
```

## Getting an Access Token

To obtain an Azure access token, you can use:

### Azure CLI (Recommended)
```bash
az login
az account get-access-token --query accessToken --output tsv
```

### Azure PowerShell
```powershell
Connect-AzAccount
(Get-AzAccessToken).Token
```

### Programmatically
You can also use Azure's authentication libraries or implement OAuth 2.0 flows to obtain tokens programmatically.

## Security Best Practices

1. **Never hardcode credentials** in your scripts or commit them to version control
2. **Use environment variables** or secure credential storage systems
3. **Rotate tokens regularly** - Azure access tokens typically expire after 1 hour
4. **Use least-privilege access** - ensure tokens have only the necessary permissions
5. **For CI/CD**, use GitHub Secrets or similar secure storage:
   ```yaml
   env:
     AZURE_ACCESS_TOKEN: ${{ secrets.AZURE_ACCESS_TOKEN }}
     AZURE_SUBSCRIPTION_ID: ${{ secrets.AZURE_SUBSCRIPTION_ID }}
   ```

## Example Usage

See [crates/nu-command/src/network/azure/EXAMPLE.nu](crates/nu-command/src/network/azure/EXAMPLE.nu) for a comprehensive example script demonstrating:

- Authentication setup
- Listing resources and VMs
- Filtering and querying data
- Error handling
- Exporting results

Quick example:

```nushell
# Set up authentication
$env.AZURE_ACCESS_TOKEN = (az account get-access-token --query accessToken --output tsv)
$env.AZURE_SUBSCRIPTION_ID = (az account show --query id --output tsv)

# List all resources and count by type
azure resource list 
  | get value 
  | group-by type 
  | transpose type count 
  | sort-by count --reverse

# Find all VMs in East US
azure vm list 
  | get value 
  | where location == "eastus" 
  | select name properties.hardwareProfile.vmSize
```

## Configuration Options

### API Versions

Each command supports custom API versions via the `--api-version` flag:

```nushell
azure resource list --api-version "2021-04-01"
azure vm list --api-version "2023-03-01"
```

Default API versions:
- Resource Manager API: `2021-04-01`
- Compute API: `2023-03-01`

## Troubleshooting

### Authentication Errors

If you see "Azure authentication required" errors:

1. Ensure you're logged in to Azure CLI: `az login`
2. Verify your token is valid: `az account get-access-token`
3. Check that environment variables are set: `$env.AZURE_ACCESS_TOKEN`

### Network Errors

If you encounter network errors:

1. Check your internet connection
2. Verify your subscription ID is correct
3. Ensure your token has appropriate permissions
4. Check if the Azure service is experiencing issues: [Azure Status](https://status.azure.com/)

### API Version Errors

If you get API version errors:

1. Use the `--api-version` flag to specify a different version
2. Check the [Azure REST API documentation](https://docs.microsoft.com/rest/api/azure/) for supported versions

## Further Documentation

- [Azure REST API Reference](https://docs.microsoft.com/rest/api/azure/)
- [Azure Resource Manager REST API](https://docs.microsoft.com/rest/api/resources/)
- [Azure Compute REST API](https://docs.microsoft.com/rest/api/compute/)
- [Module README](crates/nu-command/src/network/azure/README.md)
- [Example Script](crates/nu-command/src/network/azure/EXAMPLE.nu)

## Contributing

To add new Azure commands, see the [module README](crates/nu-command/src/network/azure/README.md) for development guidelines.
