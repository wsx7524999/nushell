# Azure REST API Integration Example
# This script demonstrates how to use the Azure commands in Nushell

# Prerequisites:
# 1. Install Azure CLI: https://docs.microsoft.com/cli/azure/install-azure-cli
# 2. Log in to Azure: az login

# ============================================================================
# Setup Authentication
# ============================================================================

# Option 1: Using environment variables (Recommended for scripts)
$env.AZURE_ACCESS_TOKEN = (az account get-access-token --query accessToken --output tsv)
$env.AZURE_SUBSCRIPTION_ID = (az account show --query id --output tsv)

# ============================================================================
# List All Azure Resources
# ============================================================================

print "Listing all Azure resources..."
let resources = azure resource list

# Display the resource count
print $"Found ($resources.value | length) resources"

# Show resource names and types
$resources.value | select name type location | table

# ============================================================================
# List Virtual Machines
# ============================================================================

print "\nListing Azure virtual machines..."
let vms = azure vm list

# Check if any VMs exist
if ($vms.value | length) > 0 {
    print $"Found ($vms.value | length) virtual machines"
    
    # Display VM details
    $vms.value | select name location properties.hardwareProfile.vmSize | table
    
    # Group VMs by location
    print "\nVMs by location:"
    $vms.value | group-by location | transpose location count | table
} else {
    print "No virtual machines found in this subscription"
}

# ============================================================================
# Filter and Query Examples
# ============================================================================

# Filter resources by type
print "\nStorage accounts:"
$resources.value | where type == "Microsoft.Storage/storageAccounts" | select name location | table

# Filter resources by location
print "\nResources in East US:"
$resources.value | where location == "eastus" | select name type | table

# Count resources by type
print "\nResource count by type:"
$resources.value | group-by type | transpose type count | sort-by count --reverse | first 10 | table

# ============================================================================
# Using Explicit Credentials (Alternative Method)
# ============================================================================

# Instead of using environment variables, you can pass credentials directly:
# let token = (az account get-access-token --query accessToken --output tsv)
# let subscription = (az account show --query id --output tsv)
# 
# azure resource list --token $token --subscription $subscription
# azure vm list --token $token --subscription $subscription

# ============================================================================
# Error Handling Example
# ============================================================================

# Handle authentication errors gracefully
try {
    azure resource list
} catch {
    print "Error: Failed to authenticate with Azure"
    print "Please ensure you have:"
    print "1. Logged in with 'az login'"
    print "2. Set AZURE_ACCESS_TOKEN and AZURE_SUBSCRIPTION_ID environment variables"
}

# ============================================================================
# Advanced Usage with API Versioning
# ============================================================================

# Use a specific API version
print "\nUsing specific API version:"
azure resource list --api-version "2021-04-01" | length

# ============================================================================
# Export Results
# ============================================================================

# Export resources to JSON
$resources | to json | save azure_resources.json

# Export VM list to CSV
if ($vms.value | length) > 0 {
    $vms.value | select name location properties.hardwareProfile.vmSize | to csv | save azure_vms.csv
}

print "\nDone! Results saved to azure_resources.json and azure_vms.csv"
