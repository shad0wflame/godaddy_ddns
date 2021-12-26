use std::collections::HashMap;

use log::{debug, info};
use strfmt::strfmt;

use crate::file_handler::get_records_file;
use crate::records_handler::dns_record::{DNSRecord, DNSRecordsHolder};
use crate::ip_handler::get_ip_to_publish;

mod dns_record;

/// Updates the DNS records if the IP has changed and returns the result of the execution.
///
/// # Arguments
///
/// * `domain` - A &str holding the domain to update.
///
/// * `key` - A &str holding the GoDaddy developer key.
///
/// * `secret` - A &str holding the GoDaddy developer secret.
pub async fn update(domain: &str, key: &str, secret: &str) -> Result<(), Box<dyn std::error::Error>> {
    let records = get_records();

    info!("Checking if the IP has changed.");
    let new_ip = get_ip_to_publish().await;

    // There's no need to do anything here. So we stop the execution.
    if new_ip.is_none() {
        info!("The IP hasn't changed. Let's stop the execution here.");
        return Ok(());
    }

    info!("The IP has changed. Let's update the DNS records.");
    for record in records {
        debug!("{:?}", record);
        update_record(record, &new_ip.clone().unwrap(), domain, key, secret).await;
    }

    Ok(())
}

/// Gets a vector of DNSRecord from RECORDS_FILE_NAME and returns it.
fn get_records() -> Vec<DNSRecord> {
    let content = get_records_file();

    let base: DNSRecordsHolder =
        serde_json::from_str(&content).expect("Failed to deserialize JSON");

    base.records
}

/// Sends a put request to the GoDaddy API to update a DNS record.
///
/// # Arguments
///
/// * `record` - A DNSRecord holding the record to update.
///
/// * `value` - A &str holding the current WAN ip.
///
/// * `domain` - A &str holding the domain to update.
///
/// * `key` - A &str holding the GoDaddy developer key.
///
/// * `secret` - A &str holding the GoDaddy developer secret.
async fn update_record(
    record: DNSRecord,
    value: &str,
    domain: &str,
    key: &str,
    secret: &str,
) -> () {
    let url = format!(
        "https://api.godaddy.com/v1/domains/{domain}/records/{record_type}/{name}",
        domain = domain,
        record_type = record.record_type.unwrap(),
        name = record.name.unwrap()
    );

    let data = match &record.data {
        Some(x) => {
            if record.interpolate.is_some() && record.interpolate.unwrap() == true {
                let mut vars = HashMap::new();
                vars.insert("ip".to_string(), value);

                strfmt(x, &vars).expect("Error interpolating {ip} from data.")
            } else {
                String::from(x)
            }
        }
        None => String::from(value),
    };

    let body = vec![DNSRecord {
        name: None,
        record_type: None,
        data: Some(data),
        port: record.port,
        priority: record.priority,
        protocol: record.protocol,
        service: record.service,
        ttl: record.ttl,
        interpolate: None,
        weight: record.weight,
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
