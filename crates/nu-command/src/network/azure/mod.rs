mod client;
mod resource_list;
mod vm_list;

pub use resource_list::AzureResourceList;
pub use vm_list::AzureVmList;

/// Default Azure Resource Manager API version
pub const DEFAULT_ARM_API_VERSION: &str = "2021-04-01";

/// Default Azure Compute API version
pub const DEFAULT_COMPUTE_API_VERSION: &str = "2023-03-01";
