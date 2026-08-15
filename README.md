# IPWho ([ipwho.org](https://www.ipwho.org)) Rust SDK

[![crates.io](https://img.shields.io/crates/v/ipwho-sdk?style=flat-square)](https://crates.io/crates/ipwho-sdk) [![docs.rs](https://img.shields.io/docsrs/ipwho-sdk?style=flat-square)](https://docs.rs/ipwho-sdk) [![license](https://img.shields.io/badge/license-MIT-green.svg)](https://github.com/lavrox/SDK-IPWho-IP-Geolocation-Rust/blob/main/LICENSE)

Official Rust SDK for the [IPWho](https://www.ipwho.org) **IP geolocation API** — geoip lookup, IP location, IP to country / latitude / longitude, ASN/ISP, timezone, currency, flag, and proxy/VPN detection. Works as an IP lookup / ip-geolocation client for IPv4 and IPv6 (`lookup`, `me`, `bulk`).

- Product: [ipwho.org](https://www.ipwho.org)
- API docs: [ipwho.org/docs](https://www.ipwho.org/docs)
- Get an API key: [ipwho.org/free-plan](https://www.ipwho.org/free-plan) (free [Lavrox](https://lavrox.com) account)
- Live API host: `https://api.ipwho.org`

## API key

Open a free [Lavrox](https://lavrox.com) account to get an API key for [IPWho](https://www.ipwho.org). Create your key at [ipwho.org/free-plan](https://www.ipwho.org/free-plan) — no credit card required.

## Installation

```bash
cargo add ipwho-sdk
```

Tokio + reqwest. Client: `IPWhoClient`.

## Quick Start

```rust
let client = IPWhoClient::new(std::env::var("IPWHO_API_KEY")?)?;

let resp = client.lookup("8.8.8.8", None, None).await?; // GET /ip/{ip}
let me = client.me(None, None).await?;                  // GET /me
let bulk = client.bulk(&["8.8.8.8", "1.1.1.1"]).await?; // BulkResponse
```

```
IpGeoResponse
├── success
├── message
└── data: Option<GeoData>
    ├── ip
    ├── geo_location
    ├── timezone
    ├── flag
    ├── currency
    ├── connection
    ├── security
    └── user_agent
```

## Reading the full response (8.8.8.8)

Live [IPWho](https://www.ipwho.org) values: United States, ASN 15169, America/Chicago, dial code +1. Fields are `Option`.

```rust
let data = client.lookup("8.8.8.8", None, None).await?.data.unwrap();
println!("{}", data.ip); // 8.8.8.8

let geo = data.geo_location.as_ref().unwrap();
println!("{:?}", geo.country);        // Some("United States")
println!("{:?}", geo.country_code);   // Some("US")
println!("{:?}", geo.continent);
println!("{:?}", geo.continent_code);
println!("{:?}", geo.capital);
println!("{:?}", geo.region);
println!("{:?}", geo.region_code);
println!("{:?}", geo.city);
println!("{:?}", geo.postal_code);
println!("{:?}", geo.dial_code);      // Some("+1")
println!("{:?}", geo.is_in_eu);
println!("{:?}", geo.latitude);
println!("{:?}", geo.longitude);
println!("{:?}", geo.accuracy_radius);

let tz = data.timezone.as_ref().unwrap();
println!("{:?}", tz.time_zone); // Some("America/Chicago")
println!("{:?}", tz.abbr);
println!("{:?}", tz.offset);
println!("{:?}", tz.is_dst);
println!("{:?}", tz.utc);
println!("{:?}", tz.current_time);

let flag = data.flag.as_ref().unwrap();
println!("{:?}", flag.flag_icon);    // 🇺🇸
println!("{:?}", flag.flag_unicode);

let cur = data.currency.as_ref().unwrap();
println!("{:?}", cur.code);
println!("{:?}", cur.name_plural); // US dollars

let conn = data.connection.as_ref().unwrap();
println!("{:?}", conn.asn_number);      // 15169
println!("{:?}", conn.asn_org);         // Google LLC
println!("{:?}", conn.isp);
println!("{:?}", conn.org);
println!("{:?}", conn.domain);
println!("{:?}", conn.connection_type); // Corporate

let sec = data.security.as_ref().unwrap();
println!("{:?}", sec.is_vpn);
println!("{:?}", sec.is_tor);
println!("{:?}", sec.is_threat);

if let Some(ua) = &data.user_agent {
    println!("{:?}", ua.browser);
}

let me = client.me(None, None).await?;
println!("{:?}", me.data.as_ref().map(|d| &d.ip));

let bulk = client.bulk(&["8.8.8.8", "1.1.1.1"]).await?;
println!("{:?}", bulk.data);
```

`format` / `fields` are `Option<&str>` (`json`/`xml`/`csv`; comma-separated filter).

## API Reference

### `IPWhoClient::new(api_key) -> Result<Self, IpWhoError>`

Also `with_client`. Query `apiKey`.

### Errors

`IpWhoError`: `Http`, `Api { status, message }`, `ApiLogical`, `Json`, `Url`, `Validation`.

## Type Definitions

```rust
pub struct GeoLocation {
    pub continent: Option<String>,
    pub continent_code: Option<String>,
    pub country: Option<String>,
    pub country_code: Option<String>,
    pub capital: Option<String>,
    pub region: Option<String>,
    pub region_code: Option<String>,
    pub city: Option<String>,
    pub postal_code: Option<String>,
    pub dial_code: Option<String>,
    pub is_in_eu: Option<bool>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub accuracy_radius: Option<f64>,
}
```

Serde renames match the live wire (`postal_Code`, `flag_Icon`, `isVpn`, `asn_number`, `name_plural`, …). Also `Timezone`, `Flag`, `Currency`, `Connection`, `Security`, `UserAgent`.

## Troubleshooting

- Key: [ipwho.org](https://www.ipwho.org).
- HTTP 403: SDK sends `ipwho-rust-sdk/1.0.0`.
- Most nested values are `Option`.

## Testing

```bash
IPWHO_API_KEY=your_key cargo test
```

```bash
IPWHO_API_KEY=your_key cargo run --example smoke
```

The live check is `examples/smoke.rs`.

## Changelog

### v1.0.0

- `lookup`, `me`, `bulk` matching [api.ipwho.org](https://api.ipwho.org)

## License

MIT License — see [LICENSE](LICENSE).

## Support

- Documentation: [ipwho.org/docs](https://www.ipwho.org/docs)
- Contact: [ipwho.org/contact](https://www.ipwho.org/contact)
- GitHub Issues: [lavrox/SDK-IPWho-IP-Geolocation-Rust](https://github.com/lavrox/SDK-IPWho-IP-Geolocation-Rust/issues)
- Website: [ipwho.org](https://www.ipwho.org)

---

[IPWho](https://www.ipwho.org) — a [Lavrox](https://lavrox.com) network API.

[Lavrox](https://lavrox.com) — Independent API infrastructure. Lower latency, lower cost.
