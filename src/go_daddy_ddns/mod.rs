use std::fs::read_to_string;
use std::path::Path;
use log::{debug, info};

use serde::{Deserialize, Serialize};

use crate::ip_handler::get_ip_to_publish;

const RECORDS_FILE_NAME: &'static str = "records.json";

#[derive(Debug, Deserialize)]
struct DNSRecordsHolder {
    records: Vec<DNSRecord>,
}

#[derive(Debug, Serialize, Deserialize)]
struct DNSRecord {
    name: String,
    record_type: String,
    data: Option<String>,
    ttl: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct DNSRecordCreateTypeName {
    data: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    port: Option<u16>, // SRV Only.

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    priority: Option<u32>, // MX and SRV only.

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    protocol: Option<String>, // SRV only.

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    service: Option<String>, // SRV only.

    ttl: u32,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    weight: Option<u32>, // SRV only.
}

/// Updates the DNS records if the IP has changed and returns the result of the execution.
///
/// # Arguments
///
/// * `domain` - A &str holding the domain to update.
///
/// * `key` - A &str holding the GoDaddy developer key.
///
/// * `secret` - A &str holding the GoDaddy developer secret.
pub async fn exec(domain: &str, key: &str, secret: &str) -> Result<(), Box<dyn std::error::Error>> {
    info!("Checking if the IP has changed.");
    let new_ip = get_ip_to_publish().await;

    // There's no need to do anything here. So we stop the execution.
    if Option::is_none(&new_ip) {
        info!("The IP hasn't changed. Let's stop the execution here.");
        return Ok(());
    }

    info!("The IP has changed. Let's update the DNS records.");
    for record in get_records() {
        debug!("{:?}", record);
        update_record(&record, &new_ip.clone().unwrap(), domain, key, secret).await;
    }

    Ok(())
}

/// Gets a vector of DNSRecord from RECORDS_FILE_NAME and returns it.
fn get_records() -> Vec<DNSRecord> {
    let path = Path::new(RECORDS_FILE_NAME);
    let content = read_to_string(path).unwrap();

    let base: DNSRecordsHolder =
        serde_json::from_str(&content).expect("Failed to deserialize JSON");

    base.records
}

/// Sends a put request to the GoDaddy API to update a DNS record.
///
/// # Arguments
///
/// * `record` - A &DNSRecord holding the record to update.
///
/// * `value` - A &str holding the current WAN ip.
///
/// * `domain` - A &str holding the domain to update.
///
/// * `key` - A &str holding the GoDaddy developer key.
///
/// * `secret` - A &str holding the GoDaddy developer secret.
async fn update_record(
    record: &DNSRecord,
    value: &str,
    domain: &str,
    key: &str,
    secret: &str,
) -> () {
    let url = format!(
        "https://api.godaddy.com/v1/domains/{domain}/records/{record_type}/{name}",
        domain = domain,
        record_type = record.record_type,
        name = record.name
    );

    let data = match &record.data {
        Some(x) => String::from(x),
        None => String::from(value),
    };

    let body = vec![DNSRecordCreateTypeName {
        data,
        port: None,
        priority: None,
        protocol: None,
        service: None,
        ttl: record.ttl,
        weight: None,
    }];

    let header = format!("sso-key {}:{}", key, secret);

    let client = reqwest::Client::new();

    let req = client
        .put(url)
        .json(&body)
        .header("accept", "application/json")
        .header("content-type", "application/json")
        .header("authorization", &header);

    req.send().await.expect("Error updating record.");
}
