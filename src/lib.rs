use std::fmt;

use serde::{Deserialize, Serialize, de, ser};

#[derive(Debug, Clone, Deserialize, Serialize)]
struct Manifest {
    version: Version,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Version {
    major: u16,
    minor: u16,
    patch: u16,
    build: String,
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
                    split_patch(pa)?
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

fn split_patch<E: de::Error>(s: &str) -> Result<(u16, &str), E> {
    if s.is_empty() {
        return Err(E::custom("Empty".to_string()));
    }
    let patch: u16;
    let other: &str;
    if let Some(pos) = s.chars().position(|c| !c.is_ascii_digit()) {
        let ps = &s[..pos];
        other = &s[pos..];
        if ps.is_empty() {
            return Err(E::custom("Invalid format".to_string()));
        }
        patch = ps.parse::<u16>().map_err(E::custom)?;
    } else {
        patch = s.parse::<u16>().map_err(E::custom)?;
        other = "";
    }
    Ok((patch, other))
}
