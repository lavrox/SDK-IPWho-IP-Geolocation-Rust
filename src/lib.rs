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
    #[serde(default, rename = "responseArray")]
    pub response_array: Option<Vec<IpGeoResponse>>,
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
    /// `api_key` is required by the API as the `apiKey` query parameter.
    pub fn new<S: Into<String>>(api_key: S) -> Result<Self, IpWhoError> {
        let http = reqwest::Client::builder()
            .user_agent("ipwho-rust-sdk/1.0.0")
            .build()?;
        let base_url = Url::parse("https://api.ipwho.org")?;
        Ok(Self {
            http,
            base_url,
            api_key: api_key.into(),
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
            api_key: api_key.into(),
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
    pub async fn bulk(&self, ips: &[&str]) -> Result<BulkResponse, IpWhoError> {
        if ips.is_empty() {
            return Err(IpWhoError::Validation("IP list must not be empty".into()));
        }
        let bulk_param = ips.join(",");
        let path = format!("/bulk/{bulk_param}");

        let mut url = self.base_url.join(&path).map_err(IpWhoError::Url)?;
        url.query_pairs_mut()
            .append_pair("apiKey", &self.api_key);

        let resp = self.http.get(url.clone()).send().await?;
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

        {
            let mut qp = url.query_pairs_mut();
            qp.append_pair("apiKey", &self.api_key);
            if let Some(f) = format {
                if f != "json" {
                    qp.append_pair("format", f);
                }
            }
            if let Some(f) = fields {
                if !f.is_empty() {
                    qp.append_pair("get", f);
                }
            }
        }

        let resp = self.http.get(url.clone()).send().await?;
        let status = resp.status();
        let text = resp.text().await?;

        if !status.is_success() {
            return Err(self.parse_error(status, &text));
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

    fn parse_error(&self, status: reqwest::StatusCode, body: &str) -> IpWhoError {
        if let Ok(err) = serde_json::from_str::<ErrorResponse>(body) {
            IpWhoError::Api {
                status,
                message: err.message.unwrap_or_else(|| "unknown error".into()),
            }
        } else {
            IpWhoError::Api {
                status,
                message: body.to_string(),
            }
        }
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
                    "continent_code": "NA",
                    "country": "United States",
                    "country_code": "US",
                    "capital": "Washington",
                    "region": "California",
                    "region_code": "CA",
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

