use crate::{ReleaseManifestPlatform, RemoteRelease, RemoteReleaseData, UpdateFormat};
use serde::ser::{Serialize, Serializer};
use std::collections::HashMap;
use url::Url;

// ----------------------------
// 3) We'll define an "InnerRemoteRelease" that matches
//    the same JSON shape as our custom deserializer.
// ----------------------------
#[derive(serde::Serialize)]
struct InnerRemoteRelease {
    #[serde(rename = "name")] // Our custom deserializer used alias = "name"
    version: String, // e.g. "v0.1.0"
    notes: Option<String>,
    pub_date: Option<String>, // RFC3339 date as string
    platforms: Option<HashMap<String, ReleaseManifestPlatform>>,
    url: Option<Url>,
    signature: Option<String>,
    format: Option<UpdateFormat>,
}

// ----------------------------
// 4) Custom Serialize for RemoteRelease
// ----------------------------
impl Serialize for RemoteRelease {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Convert version to "v1.2.3" string
        let version_str = format!("v{}", self.version);

        // Convert pub_date to RFC3339 if present
        let pub_date_str = self.pub_date.map(|dt| {
            // If formatting somehow fails, fall back to `dt.to_string()`
            dt.format(&time::format_description::well_known::Rfc3339)
                .unwrap_or_else(|_| dt.to_string())
        });

        // We only store platforms if "Static", or url/signature/format if "Dynamic"
        let (platforms, url, signature, format_) = match &self.data {
            RemoteReleaseData::Static { platforms } => (Some(platforms.clone()), None, None, None),
            RemoteReleaseData::Dynamic(rmp) => (
                None,
                Some(rmp.url.clone()),
                Some(rmp.signature.clone()),
                Some(rmp.format.clone()),
            ),
        };

        // Build the "inner" struct that has the correct shape
        let inner = InnerRemoteRelease {
            version: version_str,
            notes: self.notes.clone(),
            pub_date: pub_date_str,
            platforms,
            url,
            signature,
            format: format_,
        };

        // Now serialize our inner struct
        inner.serialize(serializer)
    }
}
