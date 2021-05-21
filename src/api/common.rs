use std::{borrow::Cow, collections::BTreeMap, fmt};

use chrono::{DateTime, Utc};
use serde::Deserialize;

mod enums;
mod structs;

pub use enums::{
    AuthorAssociation, EventKind, IssueState, LockReason, RepoCreationType,
    RepoPermission, RepoSelection, Type,
};
pub use structs::{
    AccessPermissions, App, Base, Branch, Changes, Commit, CommitInner, CommitTree,
    Committer, Head, Label, Links, Milestone, Org, Permissions, Plan, Repo, ShortUser,
    Team, User, Verification,
};

pub type Dt = DateTime<Utc>;

#[derive(Clone, Debug)]
pub struct UrlMap {
    urls: BTreeMap<String, String>,
}

#[allow(unused)]
impl UrlMap {
    pub fn get(&self, k: &str) -> Option<&str> { self.urls.get(k).map(|s| s.as_str()) }
    pub fn len(&self) -> usize { self.urls.len() }
    pub fn is_empty(&self) -> bool { self.urls.is_empty() }
}

impl<'de> Deserialize<'de> for UrlMap {
    fn deserialize<D>(d: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::{MapAccess, Visitor};

        struct UrlMapVisitor;
        impl<'a> Visitor<'a> for UrlMapVisitor {
            type Value = UrlMap;

            // TODO: finish list
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("one of User, Owner, TODO")
            }

            fn visit_map<A: MapAccess<'a>>(
                self,
                mut map: A,
            ) -> Result<Self::Value, A::Error> {
                let mut urls = BTreeMap::new();
                // While there are entries remaining in the input, add them
                // into our map.
                while let Some((key, value)) =
                    map.next_entry::<String, Option<String>>()?
                {
                    if let Some(val) = key.ends_with("_url").then(|| value).flatten() {
                        urls.insert(key, val);
                    }
                }
                Ok(UrlMap { urls })
            }
        }

        d.deserialize_any(UrlMapVisitor)
    }
}

pub(crate) const fn true_fn() -> bool { true }

pub(crate) fn default_null<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    T: Default + Deserialize<'de>,
    D: serde::de::Deserializer<'de>,
{
    let opt = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or_default())
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum StringOrUInt<'a> {
    UInt(i64),
    String(Cow<'a, str>),
}

pub fn datetime<'de, D>(deser: D) -> Result<Dt, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    use serde::de::Error;

    let ts = StringOrUInt::deserialize(deser)?;
    match ts {
        StringOrUInt::UInt(timestamp) => Ok(Dt::from_utc(
            chrono::NaiveDateTime::from_timestamp_opt(timestamp, 0)
                .ok_or_else(|| D::Error::custom("timestamp exceeded bounds"))?,
            chrono::Utc,
        )),
        StringOrUInt::String(datetime) => datetime.parse().map_err(D::Error::custom),
    }
}

pub fn datetime_opt<'de, D>(deser: D) -> Result<Option<Dt>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    use serde::de::Error;

    let ts = StringOrUInt::deserialize(deser);
    Ok(Some(match ts {
        Ok(StringOrUInt::UInt(timestamp)) => Dt::from_utc(
            chrono::NaiveDateTime::from_timestamp_opt(timestamp, 0)
                .ok_or_else(|| D::Error::custom("timestamp exceeded bounds"))?,
            chrono::Utc,
        ),
        Ok(StringOrUInt::String(datetime)) => {
            datetime.parse().map_err(D::Error::custom)?
        }
        _ => return Ok(None),
    }))
}
