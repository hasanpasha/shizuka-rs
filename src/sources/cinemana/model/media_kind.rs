use std::fmt;
use serde::{de::{self, Visitor}, Deserialize, Deserializer};
use crate::model::MediaKind as CrateMediaKind;

#[derive(Debug, Clone, PartialEq)]
pub enum MediaKind {
    Movies,
    Series,
    Unknown,
}

impl From<CrateMediaKind> for MediaKind {
    fn from(value: CrateMediaKind) -> Self {
        match value {
            CrateMediaKind::Movies => Self::Movies,
            CrateMediaKind::Series => Self::Series,
            CrateMediaKind::Unknown => Self::Unknown,
        }
    }
}

impl Into<CrateMediaKind> for MediaKind {
    fn into(self) -> CrateMediaKind {
        match self {
            MediaKind::Movies => CrateMediaKind::Movies,
            MediaKind::Series => CrateMediaKind::Series,
            MediaKind::Unknown => CrateMediaKind::Unknown,
        }
    }
}

impl fmt::Display for MediaKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Movies => write!(f, "movies"),
            Self::Series => write!(f, "series"),
            Self::Unknown => write!(f, "unknown"),
        }
    }
}

impl<'de> Deserialize<'de> for MediaKind {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        struct IdVisitor;

        impl<'de> Visitor<'de> for IdVisitor {
            type Value = MediaKind;

            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("kind as a string")
            }

            fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
                where
                    E: de::Error, {
                self.visit_str(v.as_str())
            }

            fn visit_str<E>(self, kind: &str) -> Result<Self::Value, E>
                where E: de::Error
            {
                match String::from(kind).to_lowercase().as_str() {
                    "1" | "movie" | "movies" => Ok(MediaKind::Movies),
                    "2" | "series" => Ok(MediaKind::Series),
                    _ => Ok(MediaKind::Unknown),
                }
            }
        }

        deserializer.deserialize_any(IdVisitor)
    }
}