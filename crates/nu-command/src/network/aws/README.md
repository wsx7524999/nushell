# AWS REST API Private Functionality

## Overview

This module provides private AWS REST API access functionality that is only accessible to the repository owner. It implements AWS Signature Version 4 authentication for making authenticated requests to AWS services.

## Security Model

This functionality is "private" in the following ways:

1. **Requires AWS Credentials**: The `aws api` command requires AWS credentials to be present in environment variables:
   - `AWS_ACCESS_KEY_ID`
   - `AWS_SECRET_ACCESS_KEY`

2. **GitHub Secrets Protection**: 
   - AWS credentials are stored as GitHub Secrets in the repository settings
   - These secrets are only accessible to the repository owner
   - Forked repositories do NOT have access to these secrets
   - Secrets are never exposed in logs or pull requests

3. **Workflow Restrictions**: 
   - The GitHub Actions workflow is restricted to:
     - Repository: `wsx7524999/nushell`
     - Actor: `wsx7524999`
   - The workflow will not run for any other user or repository

4. **Runtime Checks**: 
   - The command checks for AWS credentials at runtime
   - Without valid credentials, the command returns a helpful error message
   - No functionality is available without proper authentication

## Usage

### Command Syntax

```nushell
aws api <service> <region> <method> <path> [--body <body>] [--headers <headers>]
```

### Parameters

- `service`: AWS service name (e.g., 's3', 'ec2', 'dynamodb')
- `region`: AWS region (e.g., 'us-east-1', 'eu-west-1')
- `method`: HTTP method (GET, POST, PUT, DELETE)
- `path`: API endpoint path
- `--body` / `-b`: Optional request body for POST/PUT requests
- `--headers` / `-H`: Optional additional headers as a record

### Examples

```nushell
# List S3 buckets
aws api s3 us-east-1 GET /

# Get EC2 instances
aws api ec2 us-west-2 POST / --body "Action=DescribeInstances&Version=2016-11-15"

# Access DynamoDB
aws api dynamodb us-east-1 POST / --headers {x-amz-target: "DynamoDB_20120810.ListTables"}
```

### Return Value

The command returns a record containing:
- `url`: The full AWS API URL
- `method`: The HTTP method
- `authorization`: The AWS Signature V4 authorization header
- `x-amz-date`: The timestamp used for the request
- `info`: Additional information about using the command

## GitHub Actions Workflow

A GitHub Actions workflow is provided at `.github/workflows/aws-api-private.yml` that demonstrates the usage of this functionality.

### Running the Workflow

1. Navigate to the Actions tab in the GitHub repository
2. Select "Private AWS API Access"
3. Click "Run workflow"
4. Fill in the required parameters:
   - AWS Service
   - AWS Region
   - HTTP Method
   - API Path
5. Click "Run workflow"

### Setting Up Secrets

The repository owner must configure the following secrets in the repository settings:

1. Go to Settings → Secrets and variables → Actions
2. Add the following repository secrets:
   - `AWS_ACCESS_KEY_ID`: Your AWS access key ID
   - `AWS_SECRET_ACCESS_KEY`: Your AWS secret access key

**Important**: These secrets are private and not accessible to:
- Users who fork the repository
- Users who clone the repository
- Pull requests from external contributors
- Anyone except the repository owner

## Implementation Details

### AWS Signature Version 4

The implementation follows the [AWS Signature Version 4 signing process](https://docs.aws.amazon.com/general/latest/gr/signature-version-4.html):

1. Create a canonical request
2. Create a string to sign
3. Calculate the signing key
4. Calculate the signature
5. Add the signature to the HTTP request

### Security Considerations

1. **No Hardcoded Credentials**: No AWS credentials are stored in the code
2. **Environment Variable Based**: Credentials are only read from environment variables
3. **Clear Error Messages**: Users without credentials receive clear guidance
4. **GitHub Secrets Integration**: Designed to work with GitHub's secret management
5. **Repository-Specific**: The workflow is locked to a specific repository and owner

## Limitations

- This is a basic implementation focusing on REST API signing
- Some AWS services may require additional headers or special handling
- The command prepares the signed request but doesn't execute it directly
- Users need to use the returned authorization header with HTTP commands

## Notes for Repository Owner

To use this functionality:

1. Set up AWS credentials as GitHub Secrets (see above)
2. Use the GitHub Actions workflow to make AWS API calls
3. For local testing, set environment variables:
   ```bash
   export AWS_ACCESS_KEY_ID="your-access-key"
   export AWS_SECRET_ACCESS_KEY="your-secret-key"
   ./target/release/nu
   ```

## Notes for Other Users

If you fork or clone this repository:

- The `aws api` command will be available in your build
- However, it will not function without AWS credentials
- You will need to provide your own AWS credentials to use it
- The GitHub Actions workflow will not run in your fork (due to missing secrets and repository restrictions)

This design ensures that while the code is open source, the actual AWS access is restricted to the repository owner through the secure handling of credentials.
