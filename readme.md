# GoDaddy DDNS
![godaddy-ddns](https://user-images.githubusercontent.com/4237627/147422997-f72018be-7f60-4e56-9a52-d2ac81e0dbd5.png)

## What is this?
It's a Rust application to update GoDaddy DNS Records via REST API.

## Usage
It order for the application to work, there needs to be: 

* Environment variables:
  * `DOMAIN` - The domain to update
  * `KEY` - GoDaddy API Key
  * `SECRET` - GoDaddy API SECRET

Info about how to generate the **KEY** and the **SECRET** can be found here: https://developer.godaddy.com/getstarted 

* A file called `records.json` placed in `$HOME/.godaddy-ddns` with the following structure:

```json
{
  "records": 
  [
    {
      "name": string,
      "record_type": string,
      "data": string, // Optional
      "port": number, // Optional - SRV Only. 
      "priority": number, // Optional - MX and SRV Only. 
      "protocol": string, // Optional - SRV Only.
      "service": string, // Optional - SRV Only. 
      "ttl": number,
      "interpolate": boolean, // Optional
      "weight": number // Optional - SRV Only.
    }
  ]
}
```

### Fields of interest 

* name: `{string}` - Name of the record.
* record_type: `{string}` - Type of the record (A, AAAA, CAA, CNAME, MX, NS, SRV, TXT). 
* data: `{string}` (optional) - Value of the record. (Useful in case of a hard-coded record or interpolation)
* interpolate: `{boolean}` (optional) - When set to true, the application will write the current ip in the {ip} tag of the data field.

Example of use (with a given IP of 8.8.8.8) :

```json
{
  "records": [
    {
      "name": "@",
      "record_type": "A",
      "ttl": 600
    },
    {
      "name": "@",
      "record_type": "TXT",
      "ttl": 3600,
      "data": "\"v=spf1 ip4:{ip} -all\"", // Result: "v=spf1 ip4:8.8.8.8 -all"
      "interpolate": true
    }
  ]
}
```



 