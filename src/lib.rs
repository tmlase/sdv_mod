mod utils;

use std::fmt;

use serde::{Deserialize, Serialize, de, ser};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Manifest {
    name: String,

    author: String,

    version: Version,

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

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Version {
    major: u16,
    minor: u16,
    patch: u16,
    build: String,
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

    pub fn version(&self) -> &Version {
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

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}.{}.{}{}",
            self.major, self.minor, self.patch, self.build
        )
    }
}

impl Serialize for Version {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> de::Deserialize<'de> for Version {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct Visit;

        impl<'d> de::Visitor<'d> for Visit {
            type Value = Version;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter, "like 1.2.3")
            }

            fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let mut parts = s.split('.');
                let major = if let Some(ma) = parts.next() {
                    ma.parse::<u16>().map_err(E::custom)?
                } else {
                    return Err(E::custom(format!("invalid format `{s}`")));
                };
                let minor = if let Some(mi) = parts.next() {
                    mi.parse::<u16>().map_err(E::custom)?
                } else {
                    0
                };
                let (patch, other) = if let Some(pa) = parts.next() {
                    utils::split_patch(pa)?
                } else {
                    (0, "")
                };
                Ok(Version {
                    major,
                    minor,
                    patch,
                    build: other.to_owned(),
                })
            }
        }

        deserializer.deserialize_str(Visit {})
    }
}
