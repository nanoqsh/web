use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use std::{
    fmt::{self, Debug, Display},
    ops::Deref,
};
use uuid::Uuid;

#[derive(Clone, Copy, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Id(Uuid);

impl Id {
    pub(in crate::app) fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Deref for Id {
    type Target = Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Debug for Id {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self.0)
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Serialize for Id {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let str = self.0.to_string();
        serializer.serialize_str(&str)
    }
}

struct IdVisitor;

impl<'d> de::Visitor<'d> for IdVisitor {
    type Value = Id;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "an uuid")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(Id(Uuid::parse_str(v).map_err(de::Error::custom)?))
    }
}

impl<'d> Deserialize<'d> for Id {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'d>,
    {
        deserializer.deserialize_string(IdVisitor)
    }
}
