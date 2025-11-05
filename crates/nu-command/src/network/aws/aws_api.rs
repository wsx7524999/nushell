use chrono::Utc;
use nu_engine::command_prelude::*;
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

#[derive(Clone)]
pub struct AwsApi;

impl Command for AwsApi {
    fn name(&self) -> &str {
        "aws api"
    }

    fn signature(&self) -> Signature {
        Signature::build("aws api")
            .input_output_types(vec![(Type::Nothing, Type::Any)])
            .required(
                "service",
                SyntaxShape::String,
                "AWS service name (e.g., 's3', 'ec2').",
            )
            .required(
                "region",
                SyntaxShape::String,
                "AWS region (e.g., 'us-east-1').",
            )
            .required(
                "method",
                SyntaxShape::String,
                "HTTP method (GET, POST, PUT, DELETE).",
            )
            .required("path", SyntaxShape::String, "API endpoint path.")
            .named(
                "body",
                SyntaxShape::String,
                "Request body for POST/PUT requests.",
                Some('b'),
            )
            .named(
                "headers",
                SyntaxShape::Record(vec![]),
                "Additional headers.",
                Some('H'),
            )
            .category(Category::Network)
    }

    fn description(&self) -> &str {
        "Make authenticated AWS REST API requests (requires AWS credentials in environment)."
    }

    fn extra_description(&self) -> &str {
        r#"This command requires AWS credentials to be set in environment variables:
- AWS_ACCESS_KEY_ID: Your AWS access key ID
- AWS_SECRET_ACCESS_KEY: Your AWS secret access key

This functionality is only available to the repository owner through GitHub Secrets."#
    }

    fn search_terms(&self) -> Vec<&str> {
        vec!["cloud", "amazon", "s3", "ec2"]
    }

    fn run(
        &self,
        engine_state: &EngineState,
        stack: &mut Stack,
        call: &Call,
        _input: PipelineData,
    ) -> Result<PipelineData, ShellError> {
        let service: String = call.req(engine_state, stack, 0)?;
        let region: String = call.req(engine_state, stack, 1)?;
        let method: String = call.req(engine_state, stack, 2)?;
        let path: String = call.req(engine_state, stack, 3)?;

        let body: Option<String> = call.get_flag(engine_state, stack, "body")?;
        let headers: Option<Record> = call.get_flag(engine_state, stack, "headers")?;

        // Check for AWS credentials in environment variables
        let access_key = std::env::var("AWS_ACCESS_KEY_ID").map_err(|_| {
            ShellError::GenericError {
                error: "AWS credentials not found".into(),
                msg: "AWS_ACCESS_KEY_ID environment variable is not set".into(),
                span: Some(call.head),
                help: Some("This command requires AWS credentials to be configured. Contact the repository owner.".into()),
                inner: vec![],
            }
        })?;

        let secret_key = std::env::var("AWS_SECRET_ACCESS_KEY").map_err(|_| {
            ShellError::GenericError {
                error: "AWS credentials not found".into(),
                msg: "AWS_SECRET_ACCESS_KEY environment variable is not set".into(),
                span: Some(call.head),
                help: Some("This command requires AWS credentials to be configured. Contact the repository owner.".into()),
                inner: vec![],
            }
        })?;

        // Build the host
        let host = format!("{service}.{region}.amazonaws.com");

        // Prepare request
        let now = Utc::now();
        let date_stamp = now.format("%Y%m%d").to_string();
        let amz_date = now.format("%Y%m%dT%H%M%SZ").to_string();

        // Create canonical request
        let body_str = body.as_deref().unwrap_or("");
        let payload_hash = format!("{:x}", Sha256::digest(body_str.as_bytes()));

        let mut canonical_headers = BTreeMap::new();
        canonical_headers.insert("host".to_string(), host.clone());
        canonical_headers.insert("x-amz-date".to_string(), amz_date.clone());

        // Add custom headers
        if let Some(header_record) = headers {
            for (key, value) in header_record.iter() {
                if let Ok(val_str) = value.coerce_str() {
                    canonical_headers.insert(key.to_lowercase(), val_str.to_string());
                }
            }
        }

        let signed_headers: Vec<String> = canonical_headers.keys().cloned().collect();
        let canonical_headers_str: String = canonical_headers
            .iter()
            .map(|(k, v)| format!("{k}:{v}\n"))
            .collect();

        let method_upper = method.to_uppercase();
        let signed_headers_joined = signed_headers.join(";");
        let canonical_request = format!(
            "{method_upper}\n{path}\n\n{canonical_headers_str}\n{signed_headers_joined}\n{payload_hash}"
        );

        // Create string to sign
        let algorithm = "AWS4-HMAC-SHA256";
        let credential_scope = format!("{date_stamp}/{region}/{service}/aws4_request");
        let canonical_request_hash = format!("{:x}", Sha256::digest(canonical_request.as_bytes()));

        let string_to_sign = format!(
            "{algorithm}\n{amz_date}\n{credential_scope}\n{canonical_request_hash}"
        );

        // Calculate signature
        let k_date = hmac_sha256(
            format!("AWS4{secret_key}").as_bytes(),
            date_stamp.as_bytes(),
        );
        let k_region = hmac_sha256(&k_date, region.as_bytes());
        let k_service = hmac_sha256(&k_region, service.as_bytes());
        let k_signing = hmac_sha256(&k_service, b"aws4_request");
        let signature = hmac_sha256(&k_signing, string_to_sign.as_bytes());
        let signature_hex = hex::encode(signature);

        // Build authorization header
        let authorization = format!(
            "{algorithm} Credential={access_key}/{credential_scope}, SignedHeaders={signed_headers_joined}, Signature={signature_hex}"
        );

        // Build the URL
        let url = format!("https://{host}{path}");

        // Return the request info as a record
        Ok(PipelineData::Value(
            Value::record(
                record! {
                    "url" => Value::string(url, call.head),
                    "method" => Value::string(method, call.head),
                    "authorization" => Value::string(authorization, call.head),
                    "x-amz-date" => Value::string(amz_date, call.head),
                    "info" => Value::string(
                        "AWS API request prepared. Use 'http' commands to execute the request with these headers.",
                        call.head
                    ),
                },
                call.head,
            ),
            None,
        ))
    }

    fn examples(&self) -> Vec<Example> {
        vec![
            Example {
                description: "List S3 buckets (requires AWS credentials)",
                example: r#"aws api s3 us-east-1 GET /"#,
                result: None,
            },
            Example {
                description: "Get EC2 instances in a region",
                example: r#"aws api ec2 us-west-2 POST / --body "Action=DescribeInstances&Version=2016-11-15""#,
                result: None,
            },
        ]
    }
}

// HMAC-SHA256 helper function
fn hmac_sha256(key: &[u8], data: &[u8]) -> Vec<u8> {
    use hmac::{Hmac, Mac};
    type HmacSha256 = Hmac<Sha256>;

    let mut mac = HmacSha256::new_from_slice(key).expect("HMAC can take key of any size");
    mac.update(data);
    mac.finalize().into_bytes().to_vec()
}
