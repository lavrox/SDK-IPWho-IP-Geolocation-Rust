use reqwest::Url;
use serde::{Deserialize, Serialize};
use thiserror::Error;

// ═══════════════════════════════════════════════════════════════════════════
// Error types
// ═══════════════════════════════════════════════════════════════════════════

/// Errors that can occur when using the IPWho API.
#[derive(Debug, Error)]
pub enum IpWhoError {
    /// Network / HTTP transport error.
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    /// The API returned an unsuccessful HTTP status.
    #[error("API error (status {status}): {message}")]
    Api { status: reqwest::StatusCode, message: String },

    /// The API responded with `success: false` and an optional message.
    #[error("API returned success=false: {0}")]
    ApiLogical(String),

    /// Failed to parse the JSON response body.
    #[error("JSON deserialization error: {0}")]
    Json(#[from] serde_json::Error),

    /// URL construction error.
    #[error("URL parse error: {0}")]
    Url(#[from] url::ParseError),

    /// Client-side validation error.
    #[error("{0}")]
    Validation(String),
}

// ═══════════════════════════════════════════════════════════════════════════
// Domain models — exact schema from ipwho-openapi.yaml
// ═══════════════════════════════════════════════════════════════════════════

/// Geographic location data for the queried IP.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoLocation {
    #[serde(default, rename = "continent")]
    pub continent: Option<String>,
    #[serde(default, rename = "continentCode")]
    pub continent_code: Option<String>,
    #[serde(default, rename = "country")]
    pub country: Option<String>,
    #[serde(default, rename = "countryCode")]
    pub country_code: Option<String>,
    #[serde(default, rename = "capital")]
    pub capital: Option<String>,
    #[serde(default, rename = "region")]
    pub region: Option<String>,
    #[serde(default, rename = "regionCode")]
    pub region_code: Option<String>,
    #[serde(default, rename = "city")]
    pub city: Option<String>,
    #[serde(default, rename = "postal_Code")]
    pub postal_code: Option<String>,
    #[serde(default, rename = "dial_code")]
    pub dial_code: Option<String>,
    #[serde(default, rename = "is_in_eu")]
    pub is_in_eu: Option<bool>,
    #[serde(default, rename = "latitude")]
    pub latitude: Option<f64>,
    #[serde(default, rename = "longitude")]
    pub longitude: Option<f64>,
    #[serde(default, rename = "accuracy_radius")]
    pub accuracy_radius: Option<f64>,
}

/// Timezone information for the queried IP.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Timezone {
    #[serde(default, rename = "time_zone")]
    pub time_zone: Option<String>,
    #[serde(default, rename = "abbr")]
    pub abbr: Option<String>,
    #[serde(default, rename = "offset")]
    pub offset: Option<i64>,
    #[serde(default, rename = "is_dst")]
    pub is_dst: Option<bool>,
    #[serde(default, rename = "utc")]
    pub utc: Option<String>,
    #[serde(default, rename = "current_time")]
    pub current_time: Option<String>,
}

/// Flag information for the IP's country.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Flag {
    #[serde(default, rename = "flag_Icon")]
    pub flag_icon: Option<String>,
    #[serde(default, rename = "flag_unicode")]
    pub flag_unicode: Option<String>,
}

/// Currency information for the IP's country.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Currency {
    #[serde(default, rename = "code")]
    pub code: Option<String>,
    #[serde(default, rename = "symbol")]
    pub symbol: Option<String>,
    #[serde(default, rename = "name")]
    pub name: Option<String>,
    #[serde(default, rename = "name_plural")]
    pub name_plural: Option<String>,
    #[serde(default, rename = "hex_unicode")]
    pub hex_unicode: Option<String>,
}

/// Network connection details for the queried IP.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Connection {
    #[serde(default, rename = "asn_number")]
    pub asn_number: Option<i64>,
    #[serde(default, rename = "asn_org")]
    pub asn_org: Option<String>,
    #[serde(default, rename = "isp")]
    pub isp: Option<String>,
    #[serde(default, rename = "org")]
    pub org: Option<String>,
    #[serde(default, rename = "domain")]
    pub domain: Option<String>,
    #[serde(default, rename = "connection_type")]
    pub connection_type: Option<String>,
}

/// Security / threat intelligence for the queried IP.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Security {
    #[serde(default, rename = "isVpn")]
    pub is_vpn: Option<bool>,
    #[serde(default, rename = "isTor")]
    pub is_tor: Option<bool>,
    /// One of: "low", "medium", "high"
    #[serde(default, rename = "isThreat")]
    pub is_threat: Option<String>,
}

// User-agent sub-types

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Browser {
    #[serde(default, rename = "name")]
    pub name: Option<String>,
    #[serde(default, rename = "version")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Engine {
    #[serde(default, rename = "name")]
    pub name: Option<String>,
    #[serde(default, rename = "version")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OS {
    #[serde(default, rename = "name")]
    pub name: Option<String>,
    #[serde(default, rename = "version")]
    pub version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Device {
    #[serde(default, rename = "type")]
    pub type_field: Option<String>,
    #[serde(default, rename = "vendor")]
    pub vendor: Option<String>,
    #[serde(default, rename = "model")]
    pub model: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CPU {
    #[serde(default, rename = "architecture")]
    pub architecture: Option<String>,
}

/// User-agent details parsed from the IP's traffic.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAgent {
    #[serde(default, rename = "browser")]
    pub browser: Option<Browser>,
    #[serde(default, rename = "engine")]
    pub engine: Option<Engine>,
    #[serde(default, rename = "os")]
    pub os: Option<OS>,
    #[serde(default, rename = "device")]
    pub device: Option<Device>,
    #[serde(default, rename = "cpu")]
    pub cpu: Option<CPU>,
}

/// The `data` payload inside a successful `IpGeoResponse`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeoData {
    #[serde(default, rename = "ip")]
    pub ip: String,
    #[serde(default, rename = "geoLocation")]
    pub geo_location: Option<GeoLocation>,
    #[serde(default, rename = "timezone")]
    pub timezone: Option<Timezone>,
    #[serde(default, rename = "flag")]
    pub flag: Option<Flag>,
    #[serde(default, rename = "currency")]
    pub currency: Option<Currency>,
    #[serde(default, rename = "connection")]
    pub connection: Option<Connection>,
    #[serde(default, rename = "security")]
    pub security: Option<Security>,
    #[serde(default, rename = "userAgent")]
    pub user_agent: Option<UserAgent>,
}

impl GeoData {
    fn from_raw_text(text: String) -> Self {
        Self {
            ip: text,
            geo_location: None,
            timezone: None,
            flag: None,
            currency: None,
            connection: None,
            security: None,
            user_agent: None,
        }
    }
}

/// Top-level API response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpGeoResponse {
    #[serde(default, rename = "success")]
    pub success: bool,
    #[serde(default, rename = "data")]
    pub data: Option<GeoData>,
    /// Present only when `success` is `false`.
    #[serde(default, rename = "message")]
    pub message: Option<String>,
}

/// Error payload returned on non-200 responses.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    #[serde(default, rename = "success")]
    pub success: bool,
    #[serde(default, rename = "message")]
    pub message: Option<String>,
}

/// Wraps the `responseArray` from the bulk endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkData {
    #[serde(
        default,
        rename = "responseArray",
        deserialize_with = "deserialize_bulk_rows"
    )]
    pub response_array: Option<Vec<IpGeoResponse>>,
}

fn deserialize_bulk_rows<'de, D>(deserializer: D) -> Result<Option<Vec<IpGeoResponse>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let rows: Option<Vec<serde_json::Value>> = Option::deserialize(deserializer)?;
    rows.map(|rows| rows.into_iter().map(bulk_row).collect::<Result<Vec<_>, _>>())
        .transpose()
        .map_err(serde::de::Error::custom)
}

/// Reads one bulk result. The API sends each row flat:
/// `{success, ip, geoLocation, ...}` or `{success: false, ip, message}`.
/// A row already nested under `data` is accepted too.
fn bulk_row(row: serde_json::Value) -> Result<IpGeoResponse, serde_json::Error> {
    if row.get("data").is_some() {
        return serde_json::from_value(row);
    }
    let success = row
        .get("success")
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false);
    let message = row
        .get("message")
        .and_then(serde_json::Value::as_str)
        .map(str::to_owned);
    let data: GeoData = serde_json::from_value(row)?;
    Ok(IpGeoResponse {
        success,
        data: Some(data),
        message,
    })
}

/// Response from the `/bulk/{bulkIP}` endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BulkResponse {
    #[serde(default, rename = "success")]
    pub success: bool,
    #[serde(default = "default_data")]
    pub data: Option<BulkData>,
}

fn default_data() -> Option<BulkData> {
    None
}

// ═══════════════════════════════════════════════════════════════════════════
// Client
// ═══════════════════════════════════════════════════════════════════════════

/// Rust client for the IPWho IP Geolocation API.
#[derive(Clone, Debug)]
pub struct IPWhoClient {
    http: reqwest::Client,
    base_url: Url,
    api_key: String,
}

impl IPWhoClient {
    /// Create a new client with the given API key.
    ///
    /// `api_key` is sent as `Authorization: Bearer`. Surrounding whitespace is removed.
    pub fn new<S: Into<String>>(api_key: S) -> Result<Self, IpWhoError> {
        let api_key = api_key.into().trim().to_owned();
        if api_key.is_empty() {
            return Err(IpWhoError::Validation("api_key is required".into()));
        }
        // The key is sent in a header. Reject it here, without repeating it.
        if !api_key.chars().all(|c| (' '..='~').contains(&c)) {
            return Err(IpWhoError::Validation(
                "api_key contains characters that are not allowed".into(),
            ));
        }
        let http = reqwest::Client::builder()
            .user_agent("ipwho-rust-sdk/1.0.0")
            .build()?;
        let base_url = Url::parse("https://api.ipwho.org")?;
        Ok(Self {
            http,
            base_url,
            api_key,
        })
    }

    /// Create a new client with a custom HTTP client and base URL.
    pub fn with_client<S: Into<String>>(
        api_key: S,
        http: reqwest::Client,
        base_url: Url,
    ) -> Self {
        Self {
            http,
            base_url,
            api_key: api_key.into().trim().to_owned(),
        }
    }

    // ── Public API ─────────────────────────────────────────────────────

    /// Look up geolocation data for a specific IP address.
    ///
    /// - `ip`: IPv4 or IPv6 address.
    /// - `format`: Response format (`"json"`, `"xml"`, `"csv"`). Default is `"json"`.
    /// - `fields`: Optional comma-separated field filter (e.g. `"geoLocation,timezone"`).
    pub async fn lookup(
        &self,
        ip: &str,
        format: Option<&str>,
        fields: Option<&str>,
    ) -> Result<IpGeoResponse, IpWhoError> {
        let path = format!("/ip/{ip}");
        self.request(&path, format, fields).await
    }

    /// Look up geolocation data for the caller's own IP address.
    ///
    /// - `format`: Response format.
    /// - `fields`: Optional comma-separated field filter.
    pub async fn me(
        &self,
        format: Option<&str>,
        fields: Option<&str>,
    ) -> Result<IpGeoResponse, IpWhoError> {
        self.request("/me", format, fields).await
    }

    /// Perform a bulk IP lookup.
    ///
    /// - `ips`: Slice of IPv4/IPv6 addresses.
    ///
    /// Returns one result per address. Check each row's `success`. A failed row
    /// has a `message`, and its `data` holds only the `ip`.
    pub async fn bulk(&self, ips: &[&str]) -> Result<BulkResponse, IpWhoError> {
        if ips.is_empty() {
            return Err(IpWhoError::Validation("IP list must not be empty".into()));
        }
        let bulk_param = ips.join(",");
        let path = format!("/bulk/{bulk_param}");

        let url = self.base_url.join(&path).map_err(IpWhoError::Url)?;

        let resp = self.send(url).await?;
        let status = resp.status();
        let text = resp.text().await?;

        if !status.is_success() {
            return Err(self.parse_error(status, &text));
        }

        let parsed: BulkResponse = serde_json::from_str(&text)?;
        if !parsed.success {
            return Err(IpWhoError::ApiLogical(
                "Bulk API returned success=false".into(),
            ));
        }
        Ok(parsed)
    }

    // ── Internal ───────────────────────────────────────────────────────

    async fn request(
        &self,
        path: &str,
        format: Option<&str>,
        fields: Option<&str>,
    ) -> Result<IpGeoResponse, IpWhoError> {
        let mut url = self.base_url.join(path).map_err(IpWhoError::Url)?;

        let raw_format = format.filter(|f| !f.is_empty() && *f != "json");
        let fields = fields.filter(|f| !f.is_empty());
        // Only open the query when there is something to add, to avoid a bare "?".
        if raw_format.is_some() || fields.is_some() {
            let mut qp = url.query_pairs_mut();
            if let Some(f) = raw_format {
                qp.append_pair("format", f);
            }
            if let Some(f) = fields {
                qp.append_pair("get", f);
            }
        }

        let resp = self.send(url).await?;
        let status = resp.status();
        let text = resp.text().await?;

        if !status.is_success() {
            return Err(self.parse_error(status, &text));
        }

        // "xml" and "csv" replies aren't JSON: return the raw text in `data.ip`.
        // Go by the reply itself: `/me` ignores `format` and always answers in JSON.
        if raw_format.is_some() && !text.trim_start().starts_with('{') {
            return Ok(IpGeoResponse {
                success: true,
                data: Some(GeoData::from_raw_text(text)),
                message: None,
            });
        }

        let parsed: IpGeoResponse = serde_json::from_str(&text)?;
        if !parsed.success {
            return Err(IpWhoError::ApiLogical(
                parsed
                    .message
                    .clone()
                    .unwrap_or_else(|| "API returned success=false".into()),
            ));
        }
        Ok(parsed)
    }

    /// Sends one GET request. The API key goes in a header, not in the URL.
    async fn send(&self, url: Url) -> Result<reqwest::Response, reqwest::Error> {
        self.http.get(url).bearer_auth(&self.api_key).send().await
    }

    fn parse_error(&self, status: reqwest::StatusCode, body: &str) -> IpWhoError {
        let message = match serde_json::from_str::<serde_json::Value>(body) {
            // Auth errors carry a code in `error` and no `message`.
            Ok(value) => ["message", "error"]
                .iter()
                .find_map(|key| value.get(*key).and_then(serde_json::Value::as_str))
                .unwrap_or("unknown error")
                .to_owned(),
            Err(_) => body.to_string(),
        };
        IpWhoError::Api { status, message }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_single_ip_response() {
        let json = r#"{
            "success": true,
            "data": {
                "ip": "8.8.8.8",
                "geoLocation": {
                    "continent": "North America",
                    "continentCode": "NA",
                    "country": "United States",
                    "countryCode": "US",
                    "capital": "Washington",
                    "region": "California",
                    "regionCode": "CA",
                    "city": "Mountain View",
                    "postal_Code": "94043",
                    "dial_code": "1",
                    "is_in_eu": false,
                    "latitude": 37.4056,
                    "longitude": -122.0775,
                    "accuracy_radius": 10.0
                }
            }
        }"#;
        let resp: IpGeoResponse = serde_json::from_str(json).unwrap();
        assert!(resp.success);
        let data = resp.data.unwrap();
        assert_eq!(data.ip, "8.8.8.8");
        let gl = data.geo_location.unwrap();
        assert_eq!(gl.country.as_deref(), Some("United States"));
        assert_eq!(gl.city.as_deref(), Some("Mountain View"));
    }

    #[test]
    fn deserialize_bulk_response() {
        let json = r#"{
            "success": true,
            "data": {
                "responseArray": [
                    {
                        "success": true,
                        "data": {
                            "ip": "8.8.8.8"
                        }
                    },
                    {
                        "success": true,
                        "data": {
                            "ip": "1.1.1.1"
                        }
                    }
                ]
            }
        }"#;
        let resp: BulkResponse = serde_json::from_str(json).unwrap();
        assert!(resp.success);
        let items = resp.data.unwrap().response_array.unwrap();
        assert_eq!(items.len(), 2);
        assert_eq!(items[0].data.as_ref().unwrap().ip, "8.8.8.8");
        assert_eq!(items[1].data.as_ref().unwrap().ip, "1.1.1.1");
    }

    #[test]
    fn deserialize_error() {
        let json = r#"{"success": false, "message": "Invalid API key"}"#;
        let err: ErrorResponse = serde_json::from_str(json).unwrap();
        assert!(!err.success);
        assert_eq!(err.message.as_deref(), Some("Invalid API key"));
    }
}

