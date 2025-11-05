# AWS API Private Functionality - Setup Guide

## Overview

This guide is for the repository owner (`wsx7524999`) to set up and use the private AWS REST API functionality in Nushell.

## What Is This?

This PR adds a new `aws api` command to Nushell that allows making authenticated AWS REST API requests. The command implements AWS Signature Version 4 authentication and is designed to be "private" - it only works when AWS credentials are available in the environment.

## Security Model

The functionality is private through these mechanisms:

1. **GitHub Secrets**: AWS credentials stored as repository secrets (not visible to anyone else)
2. **Workflow Restrictions**: GitHub Actions workflow only runs for repository `wsx7524999/nushell` and user `wsx7524999`
3. **Runtime Checks**: The command requires environment variables to function
4. **No Code Secrets**: No credentials are hardcoded in the codebase

## Setup Instructions

### Step 1: Configure GitHub Secrets

1. Go to your repository on GitHub: `https://github.com/wsx7524999/nushell`
2. Click on **Settings** → **Secrets and variables** → **Actions**
3. Click **New repository secret**
4. Add these secrets:

   **Secret 1:**
   - Name: `AWS_ACCESS_KEY_ID`
   - Value: Your AWS access key ID (e.g., `AKIAIOSFODNN7EXAMPLE`)

   **Secret 2:**
   - Name: `AWS_SECRET_ACCESS_KEY`
   - Value: Your AWS secret access key

### Step 2: Verify Workflow Restrictions

The workflow file `.github/workflows/aws-api-private.yml` contains this check:

```yaml
if: github.repository == 'wsx7524999/nushell' && github.actor == 'wsx7524999'
```

This ensures only you can run the workflow, even if someone forks the repository.

## Using the AWS API Command

### Via GitHub Actions

1. Go to the **Actions** tab in your repository
2. Select **Private AWS API Access** workflow
3. Click **Run workflow**
4. Fill in the parameters:
   - **AWS Service**: e.g., `s3`, `ec2`, `dynamodb`
   - **AWS Region**: e.g., `us-east-1`, `us-west-2`
   - **HTTP Method**: `GET`, `POST`, `PUT`, or `DELETE`
   - **API Path**: e.g., `/` for S3, or specific resource path
5. Click **Run workflow**

The workflow will:
- Build Nushell with the AWS API feature
- Execute the command with your AWS credentials from secrets
- Display the signed request information

### Via Local Build

If you build Nushell locally, you can use the command directly:

```bash
# Set environment variables (don't commit these!)
export AWS_ACCESS_KEY_ID="your-access-key"
export AWS_SECRET_ACCESS_KEY="your-secret-key"

# Build Nushell
cargo build --release --features network

# Use the command
./target/release/nu -c "aws api s3 us-east-1 GET /"
```

### Command Syntax

```nushell
aws api <service> <region> <method> <path> [--body <body>] [--headers <headers>]
```

**Parameters:**
- `service`: AWS service name (e.g., 's3', 'ec2', 'dynamodb')
- `region`: AWS region (e.g., 'us-east-1')
- `method`: HTTP method (GET, POST, PUT, DELETE)
- `path`: API endpoint path
- `--body` / `-b`: Request body for POST/PUT (optional)
- `--headers` / `-H`: Additional headers as a record (optional)

### Examples

#### List S3 Buckets
```nushell
aws api s3 us-east-1 GET /
```

#### Get EC2 Instances
```nushell
aws api ec2 us-west-2 POST / --body "Action=DescribeInstances&Version=2016-11-15"
```

#### DynamoDB ListTables
```nushell
aws api dynamodb us-east-1 POST / --headers {x-amz-target: "DynamoDB_20120810.ListTables"}
```

### Command Output

The command returns a record with:
- `url`: The full AWS API URL
- `method`: The HTTP method used
- `authorization`: The AWS Signature V4 authorization header
- `x-amz-date`: The timestamp used for signing
- `info`: Instructions for using the signed request

Example output:
```
╭─────────────────────┬────────────────────────────────────────────────╮
│ url                 │ https://s3.us-east-1.amazonaws.com/            │
│ method              │ GET                                            │
│ authorization       │ AWS4-HMAC-SHA256 Credential=AKIA.../...        │
│ x-amz-date          │ 20251105T210002Z                               │
│ info                │ AWS API request prepared. Use 'http' ...       │
╰─────────────────────┴────────────────────────────────────────────────╯
```

## Using the Signed Request

The command prepares a signed AWS request. To actually make the API call, you can use the output with Nushell's `http` commands:

```nushell
# Get the signed request info
let request = (aws api s3 us-east-1 GET /)

# Make the HTTP request
http get $request.url --headers {
    Authorization: $request.authorization,
    x-amz-date: $request.x-amz-date
}
```

## Security Best Practices

1. **Never commit credentials**: Don't add AWS credentials to code or `.env` files
2. **Use GitHub Secrets**: Store credentials only in GitHub repository secrets
3. **Rotate credentials**: Regularly rotate your AWS access keys
4. **Minimal permissions**: Use AWS IAM to grant only necessary permissions
5. **Monitor usage**: Check AWS CloudTrail for any unexpected API calls

## Troubleshooting

### Error: AWS credentials not found

If you see this error:
```
Error: AWS credentials not found
AWS_ACCESS_KEY_ID environment variable is not set
```

**Solutions:**
- **In GitHub Actions**: Verify secrets are properly configured in repository settings
- **Locally**: Ensure environment variables are exported in your shell
- Check that secret names exactly match: `AWS_ACCESS_KEY_ID` and `AWS_SECRET_ACCESS_KEY`

### Workflow doesn't run

If the GitHub Actions workflow doesn't execute:
- Verify you're logged in as `wsx7524999`
- Check the repository is `wsx7524999/nushell` (not a fork)
- Ensure the workflow file is in the correct location
- Check Actions tab for any error messages

### Invalid signature errors

If AWS returns signature errors:
- Verify your credentials are correct
- Check system time is synchronized (AWS requires accurate timestamps)
- Ensure the region matches the AWS service endpoint

## What This Doesn't Do

This implementation is focused on REST API signing and doesn't:
- Execute the HTTP request automatically (returns signed request info)
- Support all AWS-specific features (like S3 multipart uploads)
- Handle AWS STS (temporary credentials)
- Parse AWS API responses

## Additional Resources

- [AWS Signature Version 4 Documentation](https://docs.aws.amazon.com/general/latest/gr/signature-version-4.html)
- [AWS REST API Reference](https://docs.aws.amazon.com/index.html)
- [GitHub Encrypted Secrets Documentation](https://docs.github.com/en/actions/security-guides/encrypted-secrets)

## Support

For issues or questions about this functionality:
1. Check this documentation
2. Review the code in `crates/nu-command/src/network/aws/`
3. Check GitHub Actions logs for workflow runs

## Privacy Notice

Remember: While the code is visible in this public repository, the functionality only works with valid AWS credentials that are securely stored in your GitHub repository secrets. Users who fork or clone the repository won't have access to your credentials.
