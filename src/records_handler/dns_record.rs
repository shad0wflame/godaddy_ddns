use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct DNSRecordsHolder {
    pub records: Vec<DNSRecord>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DNSRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub record_type: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub data: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub port: Option<u16>, // SRV Only.

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub priority: Option<u32>, // MX and SRV only.

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub protocol: Option<String>, // SRV only.

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub service: Option<String>, // SRV only.

    pub ttl: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub interpolate: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub weight: Option<u32>, // SRV only.
}
