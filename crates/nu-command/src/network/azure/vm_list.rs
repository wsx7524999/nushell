use super::client::{
    azure_get_request, azure_response_to_value, create_azure_client, get_azure_subscription,
    get_azure_token,
};
use nu_engine::command_prelude::*;

#[derive(Clone)]
pub struct AzureVmList;

impl Command for AzureVmList {
    fn name(&self) -> &str {
        "azure vm list"
    }

    fn signature(&self) -> Signature {
        Signature::build("azure vm list")
            .input_output_types(vec![(Type::Nothing, Type::Any)])
            .allow_variants_without_examples(true)
            .named(
                "token",
                SyntaxShape::String,
                "Azure access token (can also use AZURE_ACCESS_TOKEN env var)",
                Some('t'),
            )
            .named(
                "subscription",
                SyntaxShape::String,
                "Azure subscription ID (can also use AZURE_SUBSCRIPTION_ID env var)",
                Some('s'),
            )
            .named(
                "api-version",
                SyntaxShape::String,
                "Azure API version to use",
                Some('v'),
            )
            .category(Category::Network)
    }

    fn description(&self) -> &str {
        "List Azure virtual machines in a subscription."
    }

    fn extra_description(&self) -> &str {
        "Queries the Azure Compute REST API to list all virtual machines in the specified subscription.
        
Authentication:
    Requires an Azure access token. You can provide it via:
    - The --token flag
    - The AZURE_ACCESS_TOKEN environment variable
    
    You also need an Azure subscription ID via:
    - The --subscription flag
    - The AZURE_SUBSCRIPTION_ID environment variable"
    }

    fn search_terms(&self) -> Vec<&str> {
        vec!["azure", "cloud", "vm", "virtual", "machine", "compute"]
    }

    fn examples(&self) -> Vec<Example> {
        vec![
            Example {
                description: "List all VMs using environment variables for authentication",
                example: "azure vm list",
                result: None,
            },
            Example {
                description: "List VMs with explicit token and subscription",
                example: "azure vm list --token $token --subscription $subscription_id",
                result: None,
            },
        ]
    }

    fn run(
        &self,
        engine_state: &EngineState,
        stack: &mut Stack,
        call: &Call,
        _input: PipelineData,
    ) -> Result<PipelineData, ShellError> {
        let token = get_azure_token(engine_state, call, stack)?;
        let subscription = get_azure_subscription(engine_state, call, stack)?;
        let api_version = call
            .get_flag::<String>(engine_state, stack, "api-version")?
            .unwrap_or_else(|| "2023-03-01".to_string());

        let client = create_azure_client()?;
        
        let url = format!(
            "https://management.azure.com/subscriptions/{}/providers/Microsoft.Compute/virtualMachines?api-version={}",
            subscription, api_version
        );

        let response = azure_get_request(&client, &url, &token)?;
        let value = azure_response_to_value(response, call.head)?;

        Ok(PipelineData::Value(value, None))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        use crate::test_examples;
        test_examples(AzureVmList {})
    }
}
