mod utils;

use serde::{Deserialize, Serialize};

pub use utils::comp_version;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Manifest {
    name: String,

    author: String,

    version: String,

    description: String,

    #[serde(alias = "UniqueID")]
    unique_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    update_keys: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    entry_dll: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    content_pack_for: Option<ModDependency>,

    #[serde(skip_serializing_if = "Option::is_none")]
    minimum_api_version: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    dependencies: Option<Vec<ModDependency>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    private_aessemblies: Option<Vec<PrivateAssembly>>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ModDependency {
    #[serde(rename = "UniqueID", alias = "UniqueId")]
    pub unique_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_required: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_version: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct PrivateAssembly {
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub used_dynamically: Option<bool>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ModCompatibility {
    pub name: String,
    pub author: String,
    pub compatibility: Compatibility,
    // slug: String,
    pub mod_pages: Vec<ModPage>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Compatibility {
    pub status: CompatibilityStatus,
    pub summary: String,
    pub broke_in: Option<String>,
    pub unofficial_version: Option<ModPage>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum CompatibilityStatus {
    Ok,
    Unofficial,
    Workaround,
    Broken,
    Obsolete,
    Abandoned,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ModPage {
    pub url: String,
    pub text: String,
}

impl Manifest {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn version(&self) -> &str {
        &self.version
    }

    pub fn unique_id(&self) -> &str {
        &self.unique_id
    }

    pub fn dependencies(&self) -> &[ModDependency] {
        self.dependencies.as_deref().unwrap_or_default()
    }
}

impl CompatibilityStatus {
    pub fn inspect(&self) -> bool {
        matches!(self, Self::Ok)
    }
}
